pub mod tile;
pub use tile::*;
pub mod level;
pub use level::*;
pub mod event;
pub use event::*;

const fn deg2rad(deg: f64) -> f64 {
    deg / 180.0 * std::f64::consts::PI
}

pub const fn bpm2crotchet(bpm: f64) -> f64 {
    60.0 / bpm
}

pub const fn bpm2mspb(bpm: f64) -> f64 {
    // 60.0 / bpm * 1000.0
    60000.0 / bpm
}
