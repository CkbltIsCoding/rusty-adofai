use crate::*;
use event::*;
use getset::*;
use rgb::Rgba;
use serde::{Deserialize, Serialize};
use std::{error, fmt};
use vector2d::Vector2D;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum TrackStyle {
    #[default]
    Standard,
    Neon,
    NeonLight,
    Basic,
    Minimal,
    Gems,
}

#[derive(Debug)]
pub struct DynamicValueEmptyError;
impl fmt::Display for DynamicValueEmptyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DynamicValue is empty")
    }
}
impl error::Error for DynamicValueEmptyError {}

#[derive(Debug, Default, Clone, Copy)]
pub struct DynamicValue<T: Clone> {
    pub orig: Option<T>,
    pub now: Option<T>,
}
impl<T: Clone> DynamicValue<T> {
    pub(crate) fn orig2now(&mut self) {
        self.now = self.orig.clone();
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Orbit {
    #[default]
    Clockwise,
    Anticlockwise,
}
impl Orbit {
    pub fn opposite(&self) -> Self {
        match *self {
            Orbit::Clockwise => Orbit::Anticlockwise,
            Orbit::Anticlockwise => Orbit::Clockwise,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum TrackColorType {
    #[default]
    Single,
    Stripes,
    Glow,
    Blink,
    Switch,
    Rainbow,
    Volume,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum TrackColorPulse {
    Backward,
    #[default]
    None,
    Forward,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum TrackAnimation {
    // TODO
    #[default]
    None,
    Fade,
    Scatter,
    #[serde(rename="Scatter_Far")]
    ScatterFar,
    Assemble,
    Extend,
    #[serde(rename="Grow_Spin")]
    GrowSpin
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum TrackDisappearAnimation {
    // TODO
    #[default]
    None,
    Fade,
    Retract,
    Scatter,
    #[serde(rename="Scatter_Far")]
    ScatterFar,
    #[serde(rename="Shrink_Spin")]
    ShrinkSpin,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum Hitsound {
    None,
    #[default]
    Kick,
    Sizzle,
    Shaker,
    FireTile,
    Hat,
    VehiclePositive,
    VehicleNegative,
    Squareshot,
    PowerDown,
    ReverbClap,
    ReverbClack,
    Hammer,
    SnareAcoustic2,
    SnareHouse,
    Sidestick,
    HatHouse,
    ShakerLoud,
    Chuck
}

#[derive(Default, Debug, Clone, Copy, Getters, CopyGetters)]
#[getset(get = "pub")]
pub struct TileData {
    pub(crate) orbit: Option<Orbit>,
    pub(crate) beats: Option<f64>,
    pub(crate) seconds: Option<f64>,
    pub(crate) stick_to_floors: Option<bool>,
    pub(crate) editor_position: Option<Vector2D<f64>>,
    pub(crate) radius_scale: Option<f64>,
    pub(crate) pause_duration: Option<f64>,
    pub(crate) hitsound: Option<Hitsound>,
    pub(crate) hitsound_volume: Option<f64>,
    pub(crate) midspin_hitsound: Option<Hitsound>,
    pub(crate) midspin_hitsound_volume: Option<f64>,

    pub(crate) hold_duration: Option<f64>,

    // Dynamic values
    pub(crate) position: DynamicValue<Vector2D<f64>>,
    pub(crate) scale: DynamicValue<Vector2D<f64>>,
    pub(crate) rotation: DynamicValue<f64>,

    pub(crate) opacity: DynamicValue<f64>,

    pub(crate) color_type: DynamicValue<TrackColorType>,
    pub(crate) color: DynamicValue<Rgba<u8>>,
    pub(crate) secondary_color: DynamicValue<Rgba<u8>>,
    pub(crate) color_anim_duration: DynamicValue<f64>,
    pub(crate) color_pulse: DynamicValue<TrackColorPulse>,
    pub(crate) pulse_length: DynamicValue<u32>,
    pub(crate) style: DynamicValue<TrackStyle>,

}
impl TileData {
    pub(crate) fn orig2now(&mut self) {
        self.position.orig2now();
        self.scale.orig2now();
        self.rotation.orig2now();

        self.opacity.orig2now();

        self.color_type.orig2now();
        self.color.orig2now();
        self.secondary_color.orig2now();
        self.color_anim_duration.orig2now();
        self.color_pulse.orig2now();
        self.pulse_length.orig2now();
        self.style.orig2now();
    }
}

#[derive(Default, Debug)]
pub struct Tile {
    pub angle: f64,
    pub events: Vec<EventData>,
    pub data: TileData,
}
impl Tile {
    pub fn new(angle: f64) -> Tile {
        Tile {
            angle,
            events: vec![],
            data: Default::default(),
        }
    }
}
