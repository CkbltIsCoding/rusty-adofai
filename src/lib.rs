//! Rusty-adofai is an open-source adofai parser built in Rust.
//! ## Example
//! ```rust
//! use rusty_adofai as adofai;
//! 
//! fn main() {
//!     const FILE_PATH: &str = "F:\\Levels\\(No VFX) Hello (BPM) 2025\\Hello (BPM) 2025.adofai";
//!     let level = adofai::Level::open(FILE_PATH).unwrap();
//!     level.parse().unwrap();
//!     for tile in &level.tiles {
//!         println!("{}", tile.data.seconds().unwrap());
//!     }
//! }
//! ```

pub use vector2d;
pub use rgb;

mod tile;
pub use tile::*;
mod level;
pub use level::*;
pub mod event;
mod easing;
pub use easing::*;


mod serde_func;
use serde_func::*;

#[inline(always)]
const fn deg2rad(deg: f64) -> f64 {
    deg / 180.0 * std::f64::consts::PI
}

#[inline(always)]
pub const fn bpm2crotchet(bpm: f64) -> f64 {
    60.0 / bpm
}

#[inline(always)]
pub const fn bpm2mspb(bpm: f64) -> f64 {
    // 60.0 / bpm * 1000.0
    60000.0 / bpm
}
