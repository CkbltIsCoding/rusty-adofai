use super::*;
use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepeatEvents {
    #[serde(default)]
    pub angle_offset: f64,
    #[serde(default)]
    pub repeat_type: RepeatType,
    pub repetitions: u32,
    pub floor_count: Option<u32>,
    pub interval: f64,
    pub execute_on_current_floor: bool,
    #[serde(
        serialize_with = "serialize_repeat_events_tag",
        deserialize_with = "deserialize_repeat_events_tag"
    )]
    pub tag: Vec<String>,
}
impl Event for RepeatEvents {}
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
}
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum RepeatType {
    #[default]
    Beat,
    Floor,
}
