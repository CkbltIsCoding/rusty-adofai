use std::error;

use serde::{Deserialize, Serialize};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

mod gameplay;
pub use gameplay::*;
mod track;
pub use track::*;
mod visual;
pub use visual::*;
mod modifiers;
pub use modifiers::*;
mod dlc;
pub use dlc::*;

use crate::*;

pub trait Event: std::fmt::Debug + Clone + Serialize + for<'a> Deserialize<'a> {}
pub trait StaticEvent: Event {
    fn apply(&self, data: &mut TileData);
}
pub trait DynamicEvent: Event {
    fn apply(
        &self,
        data: (usize, f64, f64, Option<Vec<String>>),
        level: &mut Level,
        seconds: f64,
    ) -> Result<(), Box<dyn error::Error>>;
    fn angle_offset(&self) -> f64;
}

// pub struct

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Events {
    Static(StaticEvents),
    Dynamic(DynamicEvents),
}
impl Event for Events {}

#[derive(Debug, Clone)]
pub enum EventData {
    Static {
        event: StaticEvents,
        floor: usize,
    },
    Dynamic {
        event: DynamicEvents,
        floor: usize,
        beats: Option<f64>,
        seconds: Option<f64>,
        tags: Option<Vec<String>>,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RelativeToTile {
    Start,
    ThisTile,
    End,
}
#[derive(Debug, Clone, Copy, Serialize_tuple, Deserialize_tuple)]
pub struct RelativeIndex {
    index: isize,
    relative_to: RelativeToTile,
}
impl RelativeIndex {
    pub fn calc(&self, this_floor: usize, last_floor: usize) -> usize {
        match self.relative_to {
            RelativeToTile::Start => {
                if self.index <= 0 {
                    0
                } else {
                    self.index as usize
                }
            }
            RelativeToTile::ThisTile => {
                if self.index >= 0 {
                    this_floor.saturating_add(self.index as usize)
                } else {
                    this_floor.saturating_sub((-self.index) as usize)
                }
            }
            RelativeToTile::End => {
                if self.index >= 0 {
                    last_floor
                } else {
                    last_floor.saturating_sub(self.index as usize)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "eventType")]
pub enum StaticEvents {
    Twirl(Twirl),
    Pause(Pause),
    ScaleRadius(ScaleRadius),
    ColorTrack(ColorTrack),
    PositionTrack(PositionTrack),
    SetHitsound(SetHitsound),
    Hold(Hold)
}
impl Event for StaticEvents {}
impl StaticEvent for StaticEvents {
    fn apply(&self, data: &mut TileData) {
        match self {
            StaticEvents::Twirl(twirl) => twirl.apply(data),
            StaticEvents::Pause(pause) => pause.apply(data),
            StaticEvents::ScaleRadius(scale_radius) => scale_radius.apply(data),
            StaticEvents::ColorTrack(color_track) => color_track.apply(data),
            StaticEvents::PositionTrack(position_track) => position_track.apply(data),
            StaticEvents::SetHitsound(set_hitsound) => set_hitsound.apply(data),
            StaticEvents::Hold(hold) => hold.apply(data),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "eventType")]
pub enum DynamicEvents {
    SetSpeed(SetSpeed),
    RecolorTrack(RecolorTrack),
    MoveTrack(MoveTrack),
    MoveCamera(MoveCamera),
    RepeatEvents(RepeatEvents)
}
impl Event for DynamicEvents {}
impl DynamicEvent for DynamicEvents {
    fn apply(
        &self,
        data: (usize, f64, f64, Option<Vec<String>>),
        level: &mut Level,
        seconds: f64,
    ) -> Result<(), Box<dyn error::Error>> {
        use DynamicEvents::*;
        match self {
            SetSpeed(e) => e.apply(data, level, seconds),
            RecolorTrack(e) => e.apply(data, level, seconds),
            MoveTrack(e) => e.apply(data, level, seconds),
            MoveCamera(e) => e.apply(data, level, seconds),
            RepeatEvents(e) => e.apply(data, level, seconds),
        }
    }
    fn angle_offset(&self) -> f64 {
        use DynamicEvents::*;
        match self {
            SetSpeed(e) => e.angle_offset(),
            RecolorTrack(e) => e.angle_offset(),
            MoveTrack(e) => e.angle_offset(),
            MoveCamera(e) => e.angle_offset(),
            RepeatEvents(e) => e.angle_offset(),
        }
    }
}
