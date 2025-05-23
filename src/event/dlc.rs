use crate::*;
use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hold {
    pub floor: usize,
    pub duration: f64,
    pub distance_multiplier: f64,
    pub landing_animation: bool,
}
impl Event for Hold {
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
impl StaticEvent for Hold {
    fn apply(&self, data: &mut TileData) {
        data.hold_duration = Some(self.duration);
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScaleRadius {
    pub floor: usize,
    pub scale: f64,
}
impl Event for ScaleRadius {
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
impl StaticEvent for ScaleRadius {
    fn apply(&self, data: &mut TileData) {
        data.radius_scale = Some(self.scale);
    }
}
