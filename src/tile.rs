use crate::*;
use getset::*;
use rgb::Rgba;
use serde::Serialize;
use vector2d::Vector2D;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum TrackStyle {
    Standard,
    Neon,
    NeonLight,
    Basic,
    Minimal,
    Gems,
}
impl Default for TrackStyle {
    fn default() -> Self {
        TrackStyle::Standard
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct BeatValue<T> {
    pub orig: Option<T>,
    pub now: Option<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orbit {
    Clockwise,
    Anticlockwise,
}
impl Default for Orbit {
    fn default() -> Self {
        Orbit::Clockwise
    }
}
impl Orbit {
    pub fn opposite(&self) -> Self {
        match *self {
            Orbit::Clockwise => Orbit::Anticlockwise,
            Orbit::Anticlockwise => Orbit::Clockwise,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, Getters, CopyGetters, MutGetters, Setters, WithSetters)]
#[getset(get = "pub")]
pub struct TileData {
    pub(crate) orbit: Option<Orbit>,
    pub(crate) beat: Option<f64>,
    pub(crate) ms: Option<f64>,
    pub(crate) stick_to_floors: Option<bool>,
    pub(crate) editor_position: Option<Vector2D<f64>>,
    pub(crate) radius_scale: Option<f64>,

    // BeatValues
    pub(crate) position: BeatValue<Vector2D<f64>>,

    pub(crate) rotation: BeatValue<f64>,

    pub(crate) color: BeatValue<Rgba<u8>>,
    pub(crate) secondary_color: BeatValue<Rgba<u8>>,
    pub(crate) style: BeatValue<TrackStyle>,
}

#[derive(Default, Debug)]
pub struct Tile {
    pub angle: f64,
    pub events: Vec<Events>,
    pub tile_data: TileData,
}
impl Tile {
    pub fn new(angle: f64) -> Tile {
        Tile {
            angle,
            events: vec![],
            tile_data: Default::default(),
        }
    }
}
