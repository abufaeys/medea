//! Repository that stores [`Room`]s [`Peer`]s.

use std::{
    convert::{TryFrom, TryInto},
    fmt,
};

use actix::{AsyncContext as _, Context};
use medea_client_api_proto::Incrementable;
use std::collections::HashMap;

use crate::{
    api::control::MemberId,
    log::prelude::*,
    media::{New, Peer, PeerId, PeerStateMachine, TrackId},
    signalling::{
        elements::Member,
        room::{PeersRemoved, Room, RoomError},
    },
};

#[derive(Debug)]
pub struct PeerRepository {
    /// [`Peer`]s of [`Member`]s in this [`Room`].
    peers: HashMap<PeerId, PeerStateMachine>,

    /// Count of [`Peer`]s in this [`Room`].
    peers_count: Counter<PeerId>,

    /// Count of [`MediaTrack`]s in this [`Room`].
    tracks_count: Counter<TrackId>,
}

/// Simple ID counter.
#[derive(Default, Debug)]
pub struct Counter<T: Incrementable + Copy> {
    count: T,
}

impl<T: Incrementable + Copy> Counter<T> {
    /// Returns id and increase counter.
    pub fn next_id(&mut self) -> T {
        let id = self.count;
        self.count = self.count.increment();

        id
    }
}

impl<T: Incrementable + std::fmt::Display + Copy> fmt::Display for Counter<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.count)
    }
}

impl PeerRepository {
    /// Store [`Peer`] in [`Room`].
    pub fn add_peer<S: Into<PeerStateMachine>>(&mut self, peer: S) {
        let peer = peer.into();
        self.peers.insert(peer.id(), peer);
    }

    /// Returns borrowed [`PeerStateMachine`] by its ID.
    pub fn get_peer(
        &self,
        peer_id: PeerId,
    ) -> Result<&PeerStateMachine, RoomError> {
        self.peers
            .get(&peer_id)
            .ok_or_else(|| RoomError::PeerNotFound(peer_id))
    }

    /// Create interconnected [`Peer`]s for provided [`Member`]s.
    pub fn create_peers(
        &mut self,
        first_member: &Member,
        second_member: &Member,
    ) -> (Peer<New>, Peer<New>) {
        debug!(
            "Created peer between {} and {}.",
            first_member.id(),
            second_member.id()
        );
        let first_peer_id = self.peers_count.next_id();
        let second_peer_id = self.peers_count.next_id();

        let first_peer = Peer::new(
            first_peer_id,
            first_member.id().clone(),
            second_peer_id,
            second_member.id().clone(),
        );
        let second_peer = Peer::new(
            second_peer_id,
            second_member.id().clone(),
            first_peer_id,
            first_member.id().clone(),
        );

        (first_peer, second_peer)
    }

    /// Returns mutable reference to track counter.
    pub fn get_tracks_counter(&mut self) -> &mut Counter<TrackId> {
        &mut self.tracks_count
    }

    /// Lookup [`Peer`] of [`Member`] with ID `member_id` which
    /// connected with `partner_member_id`.
    ///
    /// Return Some(peer_id, partner_peer_id) if that [`Peer`] found.
    ///
    /// Return None if that [`Peer`] not found.
    pub fn get_peer_by_members_ids(
        &self,
        member_id: &MemberId,
        partner_member_id: &MemberId,
    ) -> Option<(PeerId, PeerId)> {
        for (_, peer) in &self.peers {
            if &peer.member_id() == member_id
                && &peer.partner_member_id() == partner_member_id
            {
                return Some((peer.id(), peer.partner_peer_id()));
            }
        }

        None
    }

    /// Returns borrowed [`Peer`] by its ID.
    pub fn get_inner_peer<'a, S>(
        &'a self,
        peer_id: PeerId,
    ) -> Result<&'a Peer<S>, RoomError>
    where
        &'a Peer<S>: std::convert::TryFrom<&'a PeerStateMachine>,
        <&'a Peer<S> as TryFrom<&'a PeerStateMachine>>::Error: Into<RoomError>,
    {
        match self.peers.get(&peer_id) {
            Some(peer) => peer.try_into().map_err(Into::into),
            None => Err(RoomError::PeerNotFound(peer_id)),
        }
    }

    /// Returns all [`Peer`]s of specified [`Member`].
    pub fn get_peers_by_member_id<'a>(
        &'a self,
        member_id: &'a MemberId,
    ) -> impl Iterator<Item = &'a PeerStateMachine> {
        self.peers.iter().filter_map(move |(_, peer)| {
            if &peer.member_id() == member_id {
                Some(peer)
            } else {
                None
            }
        })
    }

    /// Returns owned [`Peer`] by its ID.
    pub fn take_inner_peer<S>(
        &mut self,
        peer_id: PeerId,
    ) -> Result<Peer<S>, RoomError>
    where
        Peer<S>: TryFrom<PeerStateMachine>,
        <Peer<S> as TryFrom<PeerStateMachine>>::Error: Into<RoomError>,
    {
        match self.peers.remove(&peer_id) {
            Some(peer) => peer.try_into().map_err(Into::into),
            None => Err(RoomError::PeerNotFound(peer_id)),
        }
    }

    /// Close all related to disconnected [`Member`] [`Peer`]s and partner
    /// [`Peer`]s.
    ///
    /// Send [`Event::PeersRemoved`] to all affected [`Member`]s.
    pub fn connection_closed(
        &mut self,
        member_id: &MemberId,
        ctx: &mut Context<Room>,
    ) {
        let mut peers_to_remove: HashMap<MemberId, Vec<PeerId>> =
            HashMap::new();

        self.get_peers_by_member_id(member_id).for_each(|peer| {
            self.get_peers_by_member_id(&peer.partner_member_id())
                .filter(|partner_peer| {
                    &partner_peer.partner_member_id() == member_id
                })
                .for_each(|partner_peer| {
                    peers_to_remove
                        .entry(partner_peer.member_id())
                        .or_insert(Vec::new())
                        .push(partner_peer.id());
                });

            peers_to_remove
                .entry(peer.partner_member_id())
                .or_insert(Vec::new())
                .push(peer.id());

            peers_to_remove
                .entry(peer.member_id())
                .or_insert(Vec::new())
                .push(peer.id());
        });

        for (peer_member_id, peers_id) in peers_to_remove {
            for peer_id in &peers_id {
                self.peers.remove(peer_id);
            }
            ctx.notify(PeersRemoved {
                member_id: peer_member_id,
                peers_id,
            })
        }
    }
}

impl From<HashMap<PeerId, PeerStateMachine>> for PeerRepository {
    fn from(map: HashMap<PeerId, PeerStateMachine>) -> Self {
        Self {
            peers: map,
            peers_count: Counter::default(),
            tracks_count: Counter::default(),
        }
    }
}
