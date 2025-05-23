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

pub trait Event: std::fmt::Debug + Clone + Serialize + for<'a> Deserialize<'a> {
    fn floor(&self) -> usize;
    fn floor_mut(&mut self) -> &mut usize;
    fn set_floor(&mut self, new_floor: usize);
}
pub trait StaticEvent: Event {
    fn apply(&self, data: &mut TileData);
}
pub trait DynamicEvent: Event {
    fn apply(
        &self,
        data: (f64, f64),
        level: &mut Level,
        seconds: f64,
    ) -> Result<(), Box<dyn error::Error>>;
    fn angle_offset(&self) -> f64;
    fn has_event_tag() -> bool;
    fn event_tag(&self) -> Option<&Vec<String>>;
    fn event_tag_mut(&mut self) -> Option<&mut Vec<String>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Events {
    Static(StaticEvents),
    Dynamic(DynamicEvents),
}
impl Event for Events {
    fn floor(&self) -> usize {
        match self {
            Self::Static(event) => event.floor(),
            Self::Dynamic(event) => event.floor(),
        }
    }
    fn floor_mut(&mut self) -> &mut usize {
        match self {
            Self::Static(event) => event.floor_mut(),
            Self::Dynamic(event) => event.floor_mut(),
        }
    }
    fn set_floor(&mut self, new_floor: usize) {
        match self {
            Self::Static(event) => event.set_floor(new_floor),
            Self::Dynamic(event) => event.set_floor(new_floor),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EventData {
    Static {
        event: StaticEvents,
    },
    Dynamic {
        event: DynamicEvents,
        beats: Option<f64>,
        seconds: Option<f64>,
    },
}
impl Serialize for EventData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            EventData::Static { event } => event.serialize(serializer),
            EventData::Dynamic { event, .. } => event.serialize(serializer),
        }
    }
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
    Hold(Hold),
}
impl Event for StaticEvents {
    fn floor(&self) -> usize {
        match self {
            Self::Twirl(event) => event.floor(),
            Self::Pause(event) => event.floor(),
            Self::ScaleRadius(event) => event.floor(),
            Self::ColorTrack(event) => event.floor(),
            Self::PositionTrack(event) => event.floor(),
            Self::SetHitsound(event) => event.floor(),
            Self::Hold(event) => event.floor(),
        }
    }
    fn floor_mut(&mut self) -> &mut usize {
        match self {
            Self::Twirl(event) => event.floor_mut(),
            Self::Pause(event) => event.floor_mut(),
            Self::ScaleRadius(event) => event.floor_mut(),
            Self::ColorTrack(event) => event.floor_mut(),
            Self::PositionTrack(event) => event.floor_mut(),
            Self::SetHitsound(event) => event.floor_mut(),
            Self::Hold(event) => event.floor_mut(),
        }
    }
    fn set_floor(&mut self, new_floor: usize) {
        match self {
            Self::Twirl(event) => event.set_floor(new_floor),
            Self::Pause(event) => event.set_floor(new_floor),
            Self::ScaleRadius(event) => event.set_floor(new_floor),
            Self::ColorTrack(event) => event.set_floor(new_floor),
            Self::PositionTrack(event) => event.set_floor(new_floor),
            Self::SetHitsound(event) => event.set_floor(new_floor),
            Self::Hold(event) => event.set_floor(new_floor),
        }
    }
}
impl StaticEvent for StaticEvents {
    fn apply(&self, data: &mut TileData) {
        match self {
            Self::Twirl(twirl) => twirl.apply(data),
            Self::Pause(pause) => pause.apply(data),
            Self::ScaleRadius(scale_radius) => scale_radius.apply(data),
            Self::ColorTrack(color_track) => color_track.apply(data),
            Self::PositionTrack(position_track) => position_track.apply(data),
            Self::SetHitsound(set_hitsound) => set_hitsound.apply(data),
            Self::Hold(hold) => hold.apply(data),
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
    RepeatEvents(RepeatEvents),
}
impl DynamicEvents {
    pub fn self_has_event_tag(&self) -> bool {
        match self {
            Self::SetSpeed(_) => SetSpeed::has_event_tag(),
            Self::RecolorTrack(_) => RecolorTrack::has_event_tag(),
            Self::MoveTrack(_) => MoveTrack::has_event_tag(),
            Self::MoveCamera(_) => MoveCamera::has_event_tag(),
            Self::RepeatEvents(_) => RepeatEvents::has_event_tag(),
        }
    }
}
impl Event for DynamicEvents {
    fn floor(&self) -> usize {
        match self {
            Self::SetSpeed(event) => event.floor(),
            Self::RecolorTrack(event) => event.floor(),
            Self::MoveTrack(event) => event.floor(),
            Self::MoveCamera(event) => event.floor(),
            Self::RepeatEvents(event) => event.floor(),
        }
    }
    fn floor_mut(&mut self) -> &mut usize {
        match self {
            Self::SetSpeed(event) => event.floor_mut(),
            Self::RecolorTrack(event) => event.floor_mut(),
            Self::MoveTrack(event) => event.floor_mut(),
            Self::MoveCamera(event) => event.floor_mut(),
            Self::RepeatEvents(event) => event.floor_mut(),
        }
    }
    fn set_floor(&mut self, new_floor: usize) {
        match self {
            Self::SetSpeed(event) => event.set_floor(new_floor),
            Self::RecolorTrack(event) => event.set_floor(new_floor),
            Self::MoveTrack(event) => event.set_floor(new_floor),
            Self::MoveCamera(event) => event.set_floor(new_floor),
            Self::RepeatEvents(event) => event.set_floor(new_floor),
        }
    }
}
impl DynamicEvent for DynamicEvents {
    fn apply(
        &self,
        data: (f64, f64),
        level: &mut Level,
        seconds: f64,
    ) -> Result<(), Box<dyn error::Error>> {
        match self {
            Self::SetSpeed(e) => e.apply(data, level, seconds),
            Self::RecolorTrack(e) => e.apply(data, level, seconds),
            Self::MoveTrack(e) => e.apply(data, level, seconds),
            Self::MoveCamera(e) => e.apply(data, level, seconds),
            Self::RepeatEvents(e) => e.apply(data, level, seconds),
        }
    }
    fn angle_offset(&self) -> f64 {
        match self {
            Self::SetSpeed(e) => e.angle_offset(),
            Self::RecolorTrack(e) => e.angle_offset(),
            Self::MoveTrack(e) => e.angle_offset(),
            Self::MoveCamera(e) => e.angle_offset(),
            Self::RepeatEvents(e) => e.angle_offset(),
        }
    }
    fn has_event_tag() -> bool {
        panic!()
    }
    fn event_tag(&self) -> Option<&Vec<String>> {
        match self {
            Self::SetSpeed(e) => e.event_tag(),
            Self::RecolorTrack(e) => e.event_tag(),
            Self::MoveTrack(e) => e.event_tag(),
            Self::MoveCamera(e) => e.event_tag(),
            Self::RepeatEvents(e) => e.event_tag(),
        }
    }
    fn event_tag_mut(&mut self) -> Option<&mut Vec<String>> {
        match self {
            Self::SetSpeed(e) => e.event_tag_mut(),
            Self::RecolorTrack(e) => e.event_tag_mut(),
            Self::MoveTrack(e) => e.event_tag_mut(),
            Self::MoveCamera(e) => e.event_tag_mut(),
            Self::RepeatEvents(e) => e.event_tag_mut(),
        }
    }
}
