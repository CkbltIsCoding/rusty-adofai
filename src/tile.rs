use crate::*;
use getset::*;
use vector2d::Vector2D;

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
pub struct TileData {
    #[getset(get = "pub")]
    pub(crate) orbit: Option<Orbit>,
    #[getset(get = "pub")]
    pub(crate) beat: Option<f64>,
    #[getset(get = "pub")]
    pub(crate) ms: Option<f64>,
    #[getset(get = "pub")]
    pub(crate) radius_scale: Option<f64>,
    #[getset(get = "pub")]
    pub(crate) position: BeatValue<Vector2D<f64>>,
    #[getset(get = "pub")]
    pub(crate) editor_position: BeatValue<Vector2D<f64>>,
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
