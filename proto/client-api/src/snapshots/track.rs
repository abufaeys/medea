use serde::{Deserialize, Serialize};

use crate::{Direction, MediaType, TrackId, TrackPatch};

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct TrackSnapshot {
    pub id: TrackId,
    pub is_muted: bool,
    pub direction: Direction,
    pub media_type: MediaType,
}

pub trait TrackSnapshotAccessor {
    fn new(
        id: TrackId,
        is_muted: bool,
        direction: Direction,
        media_type: MediaType,
    ) -> Self;

    fn update(&mut self, patch: TrackPatch) {
        if let Some(is_muted) = patch.is_muted {
            self.set_is_muted(is_muted);
        }
    }

    fn set_is_muted(&mut self, is_muted: bool);

    fn get_direction(&self) -> &Direction;

    fn get_media_type(&self) -> &MediaType;

    fn get_is_muted(&self) -> bool;

    fn get_id(&self) -> TrackId;

    fn update_snapshot(&mut self, snapshot: TrackSnapshot);
}

impl TrackSnapshotAccessor for TrackSnapshot {
    fn new(
        id: TrackId,
        is_muted: bool,
        direction: Direction,
        media_type: MediaType,
    ) -> Self {
        Self {
            id,
            is_muted,
            direction,
            media_type,
        }
    }

    fn set_is_muted(&mut self, is_muted: bool) {
        self.is_muted = is_muted;
    }

    fn get_direction(&self) -> &Direction {
        &self.direction
    }

    fn get_media_type(&self) -> &MediaType {
        &self.media_type
    }

    fn get_is_muted(&self) -> bool {
        self.is_muted
    }

    fn get_id(&self) -> TrackId {
        self.id
    }

    fn update_snapshot(&mut self, snapshot: TrackSnapshot) {
        self.is_muted = snapshot.is_muted;
    }
}
