//! Room definitions and implementations. Room is responsible for media
//! connection establishment between concrete [`Member`]s.

use std::time::Duration;

use actix::{
    fut::wrap_future, Actor, ActorFuture, AsyncContext, Context, Handler,
    Message,
};
use failure::Fail;
use futures::future;
use hashbrown::HashMap;
use medea_client_api_proto::{Command, Event, IceCandidate};

use crate::{
    api::{
        client::rpc_connection::{
            AuthorizationError, Authorize, ClosedReason, CommandMessage,
            RpcConnectionClosed, RpcConnectionEstablished,
        },
        control::{MemberId, RoomId, RoomSpec, TryFromElementError},
    },
    log::prelude::*,
    media::{
        New, Peer, PeerId, PeerStateError, PeerStateMachine,
        WaitLocalHaveRemote, WaitLocalSdp, WaitRemoteSdp,
    },
    signalling::{participants::ParticipantService, peers::PeerRepository, state::member::Participant},
};

/// Ergonomic type alias for using [`ActorFuture`] for [`Room`].
type ActFuture<I, E> = Box<dyn ActorFuture<Actor = Room, Item = I, Error = E>>;

#[allow(clippy::module_name_repetitions)]
#[derive(Fail, Debug)]
pub enum RoomError {
    #[fail(display = "Couldn't find Peer with [id = {}]", _0)]
    PeerNotFound(PeerId),
    #[fail(display = "Couldn't find RpcConnection with Member [id = {}]", _0)]
    ConnectionNotExists(MemberId),
    #[fail(display = "Unable to send event to Member [id = {}]", _0)]
    UnableToSendEvent(MemberId),
    #[fail(display = "PeerError: {}", _0)]
    PeerStateError(PeerStateError),
    #[fail(display = "Generic room error {}", _0)]
    BadRoomSpec(String),
}

impl From<PeerStateError> for RoomError {
    fn from(err: PeerStateError) -> Self {
        RoomError::PeerStateError(err)
    }
}

impl From<TryFromElementError> for RoomError {
    fn from(err: TryFromElementError) -> Self {
        RoomError::BadRoomSpec(format!(
            "Element located in wrong place. {}",
            err
        ))
    }
}

/// Media server room with its [`Member`]s.
#[derive(Debug)]
pub struct Room {
    id: RoomId,

    /// [`RpcConnection`]s of [`Member`]s in this [`Room`].
    participants: ParticipantService,

    /// [`Peer`]s of [`Member`]s in this [`Room`].
    peers: PeerRepository,
}

impl Room {
    /// Create new instance of [`Room`].
    ///
    /// Returns [`RoomError::BadRoomSpec`] when error while [`Element`]
    /// transformation happens.
    pub fn new(
        room_spec: &RoomSpec,
        reconnect_timeout: Duration,
    ) -> Result<Self, RoomError> {
        Ok(Self {
            id: room_spec.id().clone(),
            peers: PeerRepository::from(HashMap::new()),
            participants: ParticipantService::new(
                room_spec,
                reconnect_timeout,
            )?,
        })
    }

    /// Returns [`RoomId`] of this [`Room`].
    pub fn get_id(&self) -> RoomId {
        self.id.clone()
    }

    /// Sends [`Event::PeerCreated`] to one of specified [`Peer`]s based on
    /// which of them has any outbound tracks. That [`Peer`] state will be
    /// changed to [`WaitLocalSdp`] state. Both provided peers must be in
    /// [`New`] state. At least one of provided peers must have outbound
    /// tracks.
    fn send_peer_created(
        &mut self,
        peer1_id: PeerId,
        peer2_id: PeerId,
    ) -> Result<ActFuture<(), RoomError>, RoomError> {
        let peer1: Peer<New> = self.peers.take_inner_peer(peer1_id)?;
        let peer2: Peer<New> = self.peers.take_inner_peer(peer2_id)?;

        // decide which peer is sender
        let (sender, receiver) = if peer1.is_sender() {
            (peer1, peer2)
        } else if peer2.is_sender() {
            (peer2, peer1)
        } else {
            self.peers.add_peer(peer1.id(), peer1);
            self.peers.add_peer(peer2.id(), peer2);
            return Err(RoomError::BadRoomSpec(format!(
                "Error while trying to connect Peer [id = {}] and Peer [id = \
                 {}] cause neither of peers are senders",
                peer1_id, peer2_id
            )));
        };
        self.peers.add_peer(receiver.id(), receiver);

        let sender = sender.start();
        let member_id = sender.member_id();
        let peer_created = Event::PeerCreated {
            peer_id: sender.id(),
            sdp_offer: None,
            tracks: sender.tracks(),
        };
        self.peers.add_peer(sender.id(), sender);
        Ok(Box::new(wrap_future(
            self.participants
                .send_event_to_member(member_id, peer_created),
        )))
    }

    /// Sends [`Event::PeersRemoved`] to [`Member`].
    fn send_peers_removed(
        &mut self,
        member_id: MemberId,
        peers: Vec<PeerId>,
    ) -> ActFuture<(), RoomError> {
        Box::new(wrap_future(self.participants.send_event_to_member(
            member_id,
            Event::PeersRemoved { peer_ids: peers },
        )))
    }

    /// Sends [`Event::PeerCreated`] to provided [`Peer`] partner. Provided
    /// [`Peer`] state must be [`WaitLocalSdp`] and will be changed to
    /// [`WaitRemoteSdp`], partners [`Peer`] state must be [`New`] and will be
    /// changed to [`WaitLocalHaveRemote`].
    fn handle_make_sdp_offer(
        &mut self,
        from_peer_id: PeerId,
        sdp_offer: String,
    ) -> Result<ActFuture<(), RoomError>, RoomError> {
        let from_peer: Peer<WaitLocalSdp> =
            self.peers.take_inner_peer(from_peer_id)?;
        let to_peer_id = from_peer.partner_peer_id();
        let to_peer: Peer<New> = self.peers.take_inner_peer(to_peer_id)?;

        let from_peer = from_peer.set_local_sdp(sdp_offer.clone());
        let to_peer = to_peer.set_remote_sdp(sdp_offer.clone());

        let to_member_id = to_peer.member_id();
        let event = Event::PeerCreated {
            peer_id: to_peer_id,
            sdp_offer: Some(sdp_offer),
            tracks: to_peer.tracks(),
        };

        self.peers.add_peer(from_peer_id, from_peer);
        self.peers.add_peer(to_peer_id, to_peer);
        Ok(Box::new(wrap_future(
            self.participants.send_event_to_member(to_member_id, event),
        )))
    }

    /// Sends [`Event::SdpAnswerMade`] to provided [`Peer`] partner. Provided
    /// [`Peer`] state must be [`WaitLocalHaveRemote`] and will be changed to
    /// [`Stable`], partners [`Peer`] state must be [`WaitRemoteSdp`] and will
    /// be changed to [`Stable`].
    fn handle_make_sdp_answer(
        &mut self,
        from_peer_id: PeerId,
        sdp_answer: String,
    ) -> Result<ActFuture<(), RoomError>, RoomError> {
        let from_peer: Peer<WaitLocalHaveRemote> =
            self.peers.take_inner_peer(from_peer_id)?;
        let to_peer_id = from_peer.partner_peer_id();
        let to_peer: Peer<WaitRemoteSdp> =
            self.peers.take_inner_peer(to_peer_id)?;

        let from_peer = from_peer.set_local_sdp(sdp_answer.clone());
        let to_peer = to_peer.set_remote_sdp(&sdp_answer);

        let to_member_id = to_peer.member_id();
        let event = Event::SdpAnswerMade {
            peer_id: to_peer_id,
            sdp_answer,
        };

        self.peers.add_peer(from_peer_id, from_peer);
        self.peers.add_peer(to_peer_id, to_peer);

        Ok(Box::new(wrap_future(
            self.participants.send_event_to_member(to_member_id, event),
        )))
    }

    /// Sends [`Event::IceCandidateDiscovered`] to provided [`Peer`] partner.
    /// Both [`Peer`]s may have any state except [`New`].
    fn handle_set_ice_candidate(
        &mut self,
        from_peer_id: PeerId,
        candidate: IceCandidate,
    ) -> Result<ActFuture<(), RoomError>, RoomError> {
        let from_peer = self.peers.get_peer(from_peer_id)?;
        if let PeerStateMachine::New(_) = from_peer {
            return Err(RoomError::PeerStateError(PeerStateError::WrongState(
                from_peer_id,
                "Not New",
                format!("{}", from_peer),
            )));
        }

        let to_peer_id = from_peer.partner_peer_id();
        let to_peer = self.peers.get_peer(to_peer_id)?;
        if let PeerStateMachine::New(_) = to_peer {
            return Err(RoomError::PeerStateError(PeerStateError::WrongState(
                to_peer_id,
                "Not New",
                format!("{}", to_peer),
            )));
        }

        let to_member_id = to_peer.member_id();
        let event = Event::IceCandidateDiscovered {
            peer_id: to_peer_id,
            candidate,
        };

        Ok(Box::new(wrap_future(
            self.participants.send_event_to_member(to_member_id, event),
        )))
    }

    /// Create [`Peer`] between [`Member`]s and interconnect it by control API
    /// spec.
    fn create_and_interconnect_peers(
        &mut self,
        first_member: &Participant,
        second_member: &Participant,
        ctx: &mut <Self as Actor>::Context,
    ) {
        debug!(
            "Created peer member {} with member {}",
            first_member.id(),
            second_member.id()
        );

        let (first_peer_id, second_peer_id) =
            self.peers.create_peers(first_member, second_member);

        ctx.notify(ConnectPeers(first_peer_id, second_peer_id));

        // println!("Peers: {:#?}", self.peers);
    }

    /// Create and interconnect all [`Peer`]s between connected [`Member`]
    /// and all available at this moment [`Member`]s from [`MemberSpec`].
    ///
    /// Availability is determines by checking [`RpcConnection`] of all
    /// [`Member`]s from [`WebRtcPlayEndpoint`]s and from receivers of
    /// connected [`Member`].
    fn create_peers_with_available_members(
        &mut self,
        member_id: &MemberId,
        ctx: &mut <Self as Actor>::Context,
    ) {
        let member =
            if let Some(m) = self.participants.get_member_by_id(member_id) {
                m.clone()
            } else {
                error!(
                    "Try to create necessary peers for nonexistent member \
                     with ID {}. Room will be stopped.",
                    member_id
                );
                ctx.notify(CloseRoom {});
                return;
            };

        let mut need_create = Vec::new();

        // connect receivers
//        let (_, receivers) = match member.publish() {
//            Ok(r) => r,
//            Err(e) => {
//                error!(
//                    "Member {} has wrong room pipeline. Room will be stopped. \
//                     Here is error: {:?}",
//                    member.lock().unwrap().borrow().id(),
//                    e
//                );
//                ctx.notify(CloseRoom {});
//                return;
//            }
//        };
        let receivers = member.publish();
        let mut already_connected_members = Vec::new();
        for (recv_member_id, _) in receivers {
            if self.participants.member_has_connection(&recv_member_id) {
                if let Some(recv_member) =
                    self.participants.get_member_by_id(&recv_member_id)
                {
                    already_connected_members.push(recv_member_id.clone());
                    need_create.push((&member, recv_member.clone()));
                } else {
                    error!(
                        "Try to create peer for nonexistent member with ID \
                         {}. Room will be stopped.",
                        recv_member_id
                    );
                    ctx.notify(CloseRoom {});
                }
            }
        }

        // connect senders
        for (_, play) in member.play() {
            let sender_member_id = play.id();
            if already_connected_members.contains(&sender_member_id) {
                continue;
            }

            if self.participants.member_has_connection(&sender_member_id) {
                if let Some(sender_member) =
                    self.participants.get_member_by_id(&sender_member_id)
                {
                    need_create.push((&member, sender_member.clone()));
                } else {
                    error!(
                        "Try to get member with ID {} which has active \
                         RpcConnection but not presented in participants! \
                         Room will be stopped.",
                        sender_member_id
                    );
                    ctx.notify(CloseRoom {});
                }
            }
        }

        need_create
            .into_iter()
            .for_each(|(first_member, second_member)| {
                self.create_and_interconnect_peers(
                    first_member,
                    &second_member,
                    ctx,
                );
            });
    }
}

/// [`Actor`] implementation that provides an ergonomic way
/// to interact with [`Room`].
impl Actor for Room {
    type Context = Context<Self>;
}

impl Handler<Authorize> for Room {
    type Result = Result<(), AuthorizationError>;

    /// Responses with `Ok` if `RpcConnection` is authorized, otherwise `Err`s.
    fn handle(
        &mut self,
        msg: Authorize,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        self.participants
            .get_member_by_id_and_credentials(&msg.member_id, &msg.credentials)
            .map(|_| ())
    }
}

/// Signal of start signaling between specified [`Peer`]'s.
#[derive(Debug, Message)]
#[rtype(result = "Result<(), ()>")]
pub struct ConnectPeers(PeerId, PeerId);

impl Handler<ConnectPeers> for Room {
    type Result = ActFuture<(), ()>;

    /// Check state of interconnected [`Peer`]s and sends [`Event`] about
    /// [`Peer`] created to remote [`Member`].
    fn handle(
        &mut self,
        msg: ConnectPeers,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        match self.send_peer_created(msg.0, msg.1) {
            Ok(res) => {
                Box::new(res.map_err(|err, _, ctx: &mut Context<Self>| {
                    error!(
                        "Failed handle command, because {}. Room will be \
                         stopped.",
                        err
                    );
                    ctx.notify(CloseRoom {})
                }))
            }
            Err(err) => {
                error!(
                    "Failed handle command, because {}. Room will be stopped.",
                    err
                );
                ctx.notify(CloseRoom {});
                Box::new(wrap_future(future::ok(())))
            }
        }
    }
}

#[derive(Debug, Message)]
#[rtype(result = "Result<(), ()>")]
pub struct PeersRemoved {
    pub peers_id: Vec<PeerId>,
    pub member_id: MemberId,
}

impl Handler<PeersRemoved> for Room {
    type Result = ActFuture<(), ()>;

    /// Send [`Event::PeersRemoved`] to [`Member`].
    fn handle(
        &mut self,
        msg: PeersRemoved,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        Box::new(
            self.send_peers_removed(msg.member_id, msg.peers_id)
                .map_err(|err, _, ctx: &mut Context<Self>| {
                    error!(
                        "Failed PeersEvent command, because {}. Room will be \
                         stopped.",
                        err
                    );
                    ctx.notify(CloseRoom {})
                }),
        )
    }
}

impl Handler<CommandMessage> for Room {
    type Result = ActFuture<(), ()>;

    /// Receives [`Command`] from Web client and passes it to corresponding
    /// handlers. Will emit [`CloseRoom`] on any error.
    fn handle(
        &mut self,
        msg: CommandMessage,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        let result = match msg.into() {
            Command::MakeSdpOffer { peer_id, sdp_offer } => {
                self.handle_make_sdp_offer(peer_id, sdp_offer)
            }
            Command::MakeSdpAnswer {
                peer_id,
                sdp_answer,
            } => self.handle_make_sdp_answer(peer_id, sdp_answer),
            Command::SetIceCandidate { peer_id, candidate } => {
                self.handle_set_ice_candidate(peer_id, candidate)
            }
        };

        match result {
            Ok(res) => {
                Box::new(res.map_err(|err, _, ctx: &mut Context<Self>| {
                    error!(
                        "Failed handle command, because {}. Room will be \
                         stopped.",
                        err
                    );
                    ctx.notify(CloseRoom {})
                }))
            }
            Err(err) => {
                error!(
                    "Failed handle command, because {}. Room will be stopped.",
                    err
                );
                ctx.notify(CloseRoom {});
                Box::new(wrap_future(future::ok(())))
            }
        }
    }
}

impl Handler<RpcConnectionEstablished> for Room {
    type Result = ActFuture<(), ()>;

    /// Saves new [`RpcConnection`] in [`ParticipantService`], initiates media
    /// establishment between members.
    /// Create and interconnect all necessary [`Member`]'s [`Peer`]s.
    fn handle(
        &mut self,
        msg: RpcConnectionEstablished,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        info!("RpcConnectionEstablished for member {}", msg.member_id);

        // save new connection
        self.participants.connection_established(
            ctx,
            &msg.member_id,
            msg.connection,
        );

        self.create_peers_with_available_members(&msg.member_id, ctx);

        Box::new(wrap_future(future::ok(())))
    }
}

/// Signal of close [`Room`].
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct CloseRoom {}

impl Handler<CloseRoom> for Room {
    type Result = ();

    /// Sends to remote [`Member`] the [`Event`] about [`Peer`] removed.
    /// Closes all active [`RpcConnection`]s.
    fn handle(
        &mut self,
        _msg: CloseRoom,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        info!("Closing Room [id = {:?}]", self.id);
        let drop_fut = self.participants.drop_connections(ctx);
        ctx.wait(wrap_future(drop_fut));
    }
}

impl Handler<RpcConnectionClosed> for Room {
    type Result = ();

    /// Passes message to [`ParticipantService`] to cleanup stored connections.
    /// Remove all related for disconnected [`Member`] [`Peer`]s.
    fn handle(&mut self, msg: RpcConnectionClosed, ctx: &mut Self::Context) {
        info!(
            "RpcConnectionClosed for member {}, reason {:?}",
            msg.member_id, msg.reason
        );

        if let ClosedReason::Closed = msg.reason {
            self.peers.connection_closed(&msg.member_id, ctx);
        }

        self.participants
            .connection_closed(ctx, msg.member_id, &msg.reason);
    }
}
