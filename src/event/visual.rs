use super::*;
use crate::*;
use serde::{Deserialize, Serialize};
use std::error;
use vector2d::Vector2D;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveCamera {
    pub duration: f64,
    #[serde(default)]
    pub relative_to: Option<RelativeToCamera>,
    #[serde(
        default,
        serialize_with = "serialize_vector2d_option_f64",
        deserialize_with = "deserialize_vector2d_option_f64"
    )]
    pub position: Vector2D<Option<f64>>,
    #[serde(default)]
    pub rotation: Option<f64>,
    #[serde(default)]
    pub zoom: Option<f64>,
    pub angle_offset: f64,
    pub ease: Easing,
}
impl Event for MoveCamera {}
impl DynamicEvent for MoveCamera {
    fn angle_offset(&self) -> f64 {
        self.angle_offset
    }
    fn apply(
        &self,
        _data: (usize, f64, f64, Option<Vec<String>>),
        _level: &mut Level,
        _seconds: f64,
    ) -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }
}
