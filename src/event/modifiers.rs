use super::*;
use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepeatEvents {
    pub floor: usize,
    #[serde(default)]
    pub angle_offset: f64,
    #[serde(default)]
    pub repeat_type: RepeatType,
    pub repetitions: u32,
    pub floor_count: Option<u32>,
    pub interval: f64,
    pub execute_on_current_floor: bool,
    #[serde(
        serialize_with = "ser_event_tag",
        deserialize_with = "de_event_tag"
    )]
    pub tag: Vec<String>,
}
impl Event for RepeatEvents {
    fn floor(&self) -> usize {
        self.floor
    }
    fn floor_mut(&mut self) -> &mut usize {
        &mut self.floor
    }
    fn set_floor(&mut self, new_floor: usize) {
        self.floor = new_floor
    }
}
impl DynamicEvent for RepeatEvents {
    fn apply(
        &self,
        _data: (usize, f64, f64, Option<Vec<String>>),
        _level: &mut Level,
        _seconds: f64,
    ) -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }
    fn angle_offset(&self) -> f64 {
        self.angle_offset
    }
    fn has_event_tag() -> bool {
        false
    }
    fn event_tag(&self) -> Option<&Vec<String>> {
        None
    }
    fn event_tag_mut(&mut self) -> Option<&mut Vec<String>> {
        None
    }
}
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum RepeatType {
    #[default]
    Beat,
    Floor,
}
