use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};

use actix::{
    Actor, Addr, AsyncContext, Handler, Message, StreamHandler, WrapFuture,
};
use futures::channel::mpsc;
use medea_client_api_proto::{
    stats::StatId, PeerId, PeerMetrics::PeerConnectionStateChanged,
};
use patched_redis::{ConnectionInfo, Msg};

use crate::{
    api::control::RoomId,
    signalling::{room::PeerStarted, Room},
    turn::coturn_stats::{CoturnAllocationEvent, CoturnEvent},
};

#[derive(Debug)]
pub struct MetricsService {
    stats: HashMap<RoomId, RoomStats>,
    coturn_metrics: Addr<CoturnMetrics>,
}

impl MetricsService {
    // TODO: Cotext here
    pub fn new(cf: &crate::conf::turn::Turn) -> Addr<Self> {
        MetricsService::create(move |ctx| {
            let coturn_metrics = CoturnMetrics::new(cf, ctx.address()).start();

            Self {
                stats: HashMap::new(),
                coturn_metrics,
            }
        })
    }

    pub fn remove_peer(&mut self, room_id: RoomId, peer_id: PeerId) {
        if let Some(room) = self.stats.get_mut(&room_id) {
            room.tracks.remove(&peer_id);
        }
        self.coturn_metrics
            .do_send(Unsubscribe(CoturnUsername { room_id, peer_id }));
    }
}

use crate::turn::coturn_metrics::{CoturnMetrics, CoturnUsername, Unsubscribe};
use futures::StreamExt;

impl Actor for MetricsService {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_secs(10), |this, ctx| {
            for stat in this.stats.values() {
                for track in stat.tracks.values() {
                    if let PeerState::Started(_) = &track.state {
                        if track.last_update
                            < Instant::now() - Duration::from_secs(10)
                        {
                            ctx.notify(TrafficStopped {
                                source: StoppedMetricSource::Timeout,
                                peer_id: track.peer_id,
                                room_id: stat.room_id.clone(),
                                timestamp: Instant::now(),
                            });
                        }
                    }
                }
            }
        });
    }
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct TrafficFlows {
    pub room_id: RoomId,
    pub peer_id: PeerId,
    pub timestamp: Instant,
    pub source: FlowMetricSource,
}

impl Handler<TrafficFlows> for MetricsService {
    type Result = ();

    fn handle(
        &mut self,
        msg: TrafficFlows,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        if let Some(room) = self.stats.get_mut(&msg.room_id) {
            if let Some(peer) = room.tracks.get_mut(&msg.peer_id) {
                peer.last_update = msg.timestamp;
                match &mut peer.state {
                    PeerState::Started(sources) => {
                        sources.insert(msg.source);
                    }
                    PeerState::Stopped => {
                        let mut srcs = HashSet::new();
                        srcs.insert(msg.source);
                        peer.state = PeerState::Started(srcs);

                        ctx.run_later(
                            Duration::from_secs(15),
                            move |this, ctx| {
                                if let Some(room) =
                                    this.stats.get_mut(&msg.room_id)
                                {
                                    if let Some(peer) =
                                        room.tracks.get_mut(&msg.peer_id)
                                    {
                                        if let PeerState::Started(srcs) =
                                            &peer.state
                                        {
                                            // TODO: change it to enum variants
                                            // count
                                            if srcs.len() < 2 {
                                                panic!(
                                                    "\n\n\n\n\n\n\n\n\n\n\n\\
                                                     nVALIDATION \
                                                     FAILED\n\n\n\n\n\\n\n\n\\
                                                     n\n\n\n\n\n\n\n\n\n\n"
                                                )
                                            // TODO: FATAL ERROR
                                            } else {
                                                println!(
                                                    "\n\n\nYAAAAAY VALIDATION \
                                                     PASSED\n\n\n"
                                                );
                                            }
                                        }
                                    }
                                }
                            },
                        );

                        room.room.do_send(PeerStarted(peer.peer_id));
                    }
                }
            }
        }
    }
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct TrafficStopped {
    pub room_id: RoomId,
    pub peer_id: PeerId,
    pub timestamp: Instant,
    pub source: StoppedMetricSource,
}

impl Handler<TrafficStopped> for MetricsService {
    type Result = ();

    fn handle(
        &mut self,
        msg: TrafficStopped,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        if let Some(room) = self.stats.get_mut(&msg.room_id) {
            if let Some(peer) = room.tracks.remove(&msg.peer_id) {
                room.room.do_send(PeerStopped(peer.peer_id));
            }
        }
        self.remove_peer(msg.room_id, msg.peer_id);
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum FlowMetricSource {
    // TODO: PartnerPeer,
    Peer,
    Coturn,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum StoppedMetricSource {
    // TODO: PartnerPeer,
    Peer,
    Coturn,
    Timeout,
}

#[derive(Debug)]
pub enum PeerState {
    Started(HashSet<FlowMetricSource>),
    Stopped,
}

#[derive(Debug)]
pub struct PeerStat {
    pub peer_id: PeerId,
    pub state: PeerState,
    pub last_update: Instant,
}

#[derive(Debug)]
pub struct RoomStats {
    room_id: RoomId,
    room: Addr<Room>,
    tracks: HashMap<PeerId, PeerStat>,
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct RegisterRoom {
    pub room_id: RoomId,
    pub room: Addr<Room>,
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct UnregisterRoom(pub RoomId);

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct AddPeer {
    pub room_id: RoomId,
    pub peer_id: PeerId,
}

use crate::{signalling::room::PeerStopped, turn::coturn_metrics};

impl Handler<AddPeer> for MetricsService {
    type Result = ();

    fn handle(
        &mut self,
        msg: AddPeer,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        self.coturn_metrics.do_send(coturn_metrics::Subscribe(
            CoturnUsername {
                room_id: msg.room_id.clone(),
                peer_id: msg.peer_id,
            },
        ));

        if let Some(room) = self.stats.get_mut(&msg.room_id) {
            room.tracks.insert(
                msg.peer_id,
                PeerStat {
                    peer_id: msg.peer_id,
                    state: PeerState::Stopped,
                    last_update: Instant::now(),
                },
            );
        }
    }
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct RemovePeer {
    pub room_id: RoomId,
    pub peer_id: PeerId,
}

impl Handler<RemovePeer> for MetricsService {
    type Result = ();

    fn handle(
        &mut self,
        msg: RemovePeer,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        self.remove_peer(msg.room_id, msg.peer_id);
    }
}

impl Handler<RegisterRoom> for MetricsService {
    type Result = ();

    fn handle(
        &mut self,
        msg: RegisterRoom,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        self.stats.insert(
            msg.room_id.clone(),
            RoomStats {
                room_id: msg.room_id,
                room: msg.room,
                tracks: HashMap::new(),
            },
        );
    }
}