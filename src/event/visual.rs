use super::*;
use crate::*;
use serde::{Deserialize, Serialize};
use std::error;
use vector2d::Vector2D;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveCamera {
    pub floor: usize,
    #[serde(serialize_with = "ser_event_tag", deserialize_with = "de_event_tag")]
    pub event_tag: Vec<String>,
    pub duration: f64,
    #[serde(default)]
    pub relative_to: Option<RelativeToCamera>,
    #[serde(
        default,
        serialize_with = "ser_vector2d_option_f64",
        deserialize_with = "de_vector2d_option_f64"
    )]
    pub position: Vector2D<Option<f64>>,
    #[serde(default)]
    pub rotation: Option<f64>,
    #[serde(default)]
    pub zoom: Option<f64>,
    pub angle_offset: f64,
    pub ease: Easing,
}
impl Event for MoveCamera {
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
impl DynamicEvent for MoveCamera {
    fn angle_offset(&self) -> f64 {
        self.angle_offset
    }
    fn apply(
        &self,
        _data: (f64, f64),
        _level: &mut Level,
        _seconds: f64,
    ) -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }
    fn has_event_tag() -> bool {
        true
    }
    fn event_tag(&self) -> Option<&Vec<String>> {
        Some(&self.event_tag)
    }
    fn event_tag_mut(&mut self) -> Option<&mut Vec<String>> {
        Some(&mut self.event_tag)
    }
}
