mod parse;
mod serde_level;
mod update;
use event::*;

use std::{error, fmt, fs, path::Path};

use crate::*;
use getset::*;
use rgb::Rgba;
use serde::{Deserialize, Serialize};
use strip_bom::StripBom;
use vector2d::Vector2D;

#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Lenient,
    Normal,
    Strict,
}

#[derive(Debug, Clone, Copy)]
pub enum HitMargin {
    Perfect,
    LatePerfect,
    EarlyPerfect,
    VeryLate,
    VeryEarly,
    TooLate,
    TooEarly,
}

pub const PATH_ANGLE: [(char, f64); 29] = [
    ('R', 0.0),
    ('p', 15.0),
    ('J', 30.0),
    ('E', 45.0),
    ('T', 60.0),
    ('o', 75.0),
    ('U', 90.0),
    ('q', 105.0),
    ('G', 120.0),
    ('Q', 135.0),
    ('H', 150.0),
    ('W', 165.0),
    ('L', 180.0),
    ('x', 195.0),
    ('N', 210.0),
    ('Z', 225.0),
    ('F', 240.0),
    ('V', 255.0),
    ('D', 270.0),
    ('Y', 285.0),
    ('B', 300.0),
    ('C', 315.0),
    ('M', 330.0),
    ('A', 345.0),
    ('5', 555.0),
    ('6', 666.0),
    ('7', 777.0),
    ('8', 888.0),
    ('!', 999.0),
];
pub fn path2angle(path: char) -> Result<f64, ()> {
    for (cmp_path, angle) in PATH_ANGLE {
        if path == cmp_path {
            return Ok(angle);
        };
    }
    Err(())
}
pub fn angle2path(angle: f64) -> Result<char, ()> {
    for (path, cmp_angle) in PATH_ANGLE {
        if angle == cmp_angle {
            return Ok(path);
        };
    }
    Err(())
}

#[derive(Debug)]
pub struct LevelIsNotParsedError {
    calling_function: &'static str,
}
impl fmt::Display for LevelIsNotParsedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Level struct is not parsed when calling level.{}(). Call level.parse() first.",
            self.calling_function
        )
    }
}
impl error::Error for LevelIsNotParsedError {}

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RelativeToCamera {
    Tile,
    #[default]
    Player,
    Global,
    LastPosition,
}

const fn f64_1() -> f64 {
    1.0
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub version: u32,
    pub artist: String,
    pub song: String,
    pub author: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub separate_countdown_time: bool,
    pub song_filename: String,
    pub bpm: f64,
    pub volume: f64,
    pub offset: f64,
    pub pitch: f64,
    #[serde(default)]
    pub countdown_ticks: u32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub stick_to_floors: bool,

    pub track_color_type: TrackColorType,
    #[serde(
        serialize_with = "serialize_rgba_u8",
        deserialize_with = "deserialize_rgba_u8"
    )]
    pub track_color: Rgba<u8>,
    #[serde(
        serialize_with = "serialize_rgba_u8",
        deserialize_with = "deserialize_rgba_u8"
    )]
    pub secondary_track_color: Rgba<u8>,
    pub track_color_anim_duration: f64,
    pub track_color_pulse: TrackColorPulse,
    pub track_pulse_length: u32,
    pub track_style: TrackStyle,
    #[serde(default)]
    pub track_texture: String,
    #[serde(default = "f64_1")]
    pub track_texture_scale: f64,
    #[serde(default)]
    pub track_glow_intensity: f64,
    pub track_animation: TrackAnimation,
    pub beats_ahead: f64,
    pub track_disappear_animation: TrackDisappearAnimation,
    pub beats_behind: f64,
    #[serde(
        serialize_with = "serialize_rgba_u8",
        deserialize_with = "deserialize_rgba_u8"
    )]
    pub background_color: Rgba<u8>,

    #[serde(
        default,
        serialize_with = "serialize_vector2d_f64",
        deserialize_with = "deserialize_vector2d_f64"
    )]
    pub position: Vector2D<f64>,
    pub rotation: f64,
    pub zoom: f64,
    pub relative_to: RelativeToCamera,

    pub hitsound: Hitsound,
    #[serde(default="f64_100")]
    pub hitsound_volume: f64,
}
const fn f64_100() -> f64 {
    100.0
}

#[derive(Debug, Getters)]
pub struct Camera {
    #[getset(get = "pub")]
    position: Vector2D<f64>,
    #[getset(get = "pub")]
    rotation: f64,
    #[getset(get = "pub")]
    zoom: f64,
    player_cam_pos: Vector2D<f64>,
    last_seconds: f64,
    last_floor: usize,
    last_change_pos: Vector2D<f64>,
    last_event_index: usize,
    v: Vec<Vector2D<f64>>
}
impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: Vector2D::new(0.0, 0.0),
            zoom: 100.0,
            rotation: 0.0,
            player_cam_pos: Vector2D::new(0.0, 0.0),
            last_seconds: f64::NEG_INFINITY,
            last_floor: 0,
            last_change_pos: Vector2D::new(0.0, 0.0),
            last_event_index: 0,
            v: vec![]
        }
    }
}

#[derive(Debug, Default, Getters)]
pub struct Level {
    pub tiles: Vec<Tile>,
    pub settings: Settings,
    #[getset(get = "pub")]
    parsed: bool,
    #[getset(get = "pub")]
    camera: Camera,
    dynamic_events: Vec<EventData>,
}

impl Level {
    pub fn new(tiles: Vec<Tile>, settings: Settings) -> Self {
        Level {
            tiles,
            settings,
            ..Default::default()
        }
    }
    pub fn open<P>(path: P) -> Result<Level, Box<dyn error::Error>>
    where
        P: AsRef<Path>,
    {
        let string_bom = fs::read_to_string(path).unwrap();
        let string_data = string_bom.strip_bom();
        let level: Level = serde_json_lenient::from_str(string_data)?;
        Ok(level)
    }
    pub fn beats2seconds(&self, beats: f64) -> Result<f64, Box<dyn error::Error>> {
        if !self.parsed {
            return Err(Box::new(LevelIsNotParsedError {
                calling_function: "beats2seconds",
            }));
        }
        let mut bpm = self.settings.bpm;
        let mut last_beats = 0.0;
        let mut seconds = self.settings.offset / 1000.0;
        'tile_loop: for tile in &self.tiles {
            for event in &tile.events {
                if let EventData::Dynamic {
                    event: DynamicEvents::SetSpeed(set_speed),
                    beats: ss_beats_option,
                    ..
                } = *event
                {
                    let ss_beats = ss_beats_option.ok_or(DynamicValueEmptyError)?;
                    if beats > ss_beats {
                        seconds += bpm2crotchet(bpm) * (ss_beats - last_beats);
                    } else {
                        break 'tile_loop;
                    }
                    bpm = set_speed.get_bpm(bpm);
                    last_beats = ss_beats;
                }
            }
        }
        seconds += bpm2crotchet(bpm) * (beats - last_beats);
        Ok(seconds)
    }
    pub fn seconds2beats(&self, mut seconds: f64) -> Result<f64, Box<dyn error::Error>> {
        if !self.parsed {
            return Err(Box::new(LevelIsNotParsedError {
                calling_function: "seconds2beats",
            }));
        }
        seconds -= self.settings.offset / 1000.0;
        let mut bpm = self.settings.bpm;
        let mut last_beats = 0.0;
        let mut beats = 0.0;
        'tile_loop: for tile in &self.tiles {
            for event in &tile.events {
                if let EventData::Dynamic {
                    event: DynamicEvents::SetSpeed(set_speed),
                    beats: ss_beats_option,
                    ..
                } = *event
                {
                    let ss_beats = ss_beats_option.ok_or(DynamicValueEmptyError)?;
                    let gap_seconds = bpm2crotchet(bpm) * (ss_beats - last_beats);
                    if seconds > gap_seconds {
                        seconds -= gap_seconds;
                    } else {
                        break 'tile_loop;
                    }
                    beats += ss_beats - last_beats;
                    bpm = set_speed.get_bpm(bpm);
                    last_beats = ss_beats;
                }
            }
        }
        beats += seconds / bpm2crotchet(bpm);
        Ok(beats)
    }
    pub fn get_bpm_until<F>(&self, function: F) -> Result<f64, Box<dyn error::Error>>
    where
        F: Fn(SetSpeed, usize, Option<f64>, Option<f64>) -> bool,
    {
        if !self.parsed {
            return Err(Box::new(LevelIsNotParsedError {
                calling_function: "get_bpm_until",
            }));
        }
        let mut bpm = self.settings.bpm;
        'tile_loop: for tile in &self.tiles {
            for event in &tile.events {
                if let EventData::Dynamic {
                    event: DynamicEvents::SetSpeed(set_speed),
                    floor: ss_floor,
                    beats: ss_beats,
                    seconds: ss_seconds,
                    ..
                } = *event
                {
                    if function(set_speed, ss_floor, ss_beats, ss_seconds) {
                        break 'tile_loop;
                    }
                    bpm = set_speed.get_bpm(bpm);
                }
            }
        }
        Ok(bpm)
    }
    pub fn get_bpm_by_beats(&self, beats: f64) -> Result<f64, Box<dyn error::Error>> {
        if !self.parsed {
            return Err(Box::new(LevelIsNotParsedError {
                calling_function: "get_bpm_by_beats",
            }));
        }
        let mut bpm = self.settings.bpm;
        'tile_loop: for tile in &self.tiles {
            for event in &tile.events {
                if let EventData::Dynamic {
                    event: DynamicEvents::SetSpeed(set_speed),
                    beats: ss_beats,
                    ..
                } = *event
                {
                    if beats < ss_beats.ok_or(DynamicValueEmptyError)? {
                        break 'tile_loop;
                    }
                    bpm = set_speed.get_bpm(bpm);
                }
            }
        }
        Ok(bpm)
    }
    pub fn get_bpm_excluding_beats(&self, beats: f64) -> Result<f64, Box<dyn error::Error>> {
        if !self.parsed {
            return Err(Box::new(LevelIsNotParsedError {
                calling_function: "get_bpm_excluding_beats",
            }));
        }
        let mut bpm = self.settings.bpm;
        'tile_loop: for tile in &self.tiles {
            for event in &tile.events {
                if let EventData::Dynamic {
                    event: DynamicEvents::SetSpeed(set_speed),
                    beats: ss_beats,
                    ..
                } = *event
                {
                    if beats <= ss_beats.ok_or(DynamicValueEmptyError)? {
                        break 'tile_loop;
                    }
                    bpm = set_speed.get_bpm(bpm);
                }
            }
        }
        Ok(bpm)
    }
    pub fn get_bpm_by_seconds(&self, seconds: f64) -> Result<f64, Box<dyn error::Error>> {
        if !self.parsed {
            return Err(Box::new(LevelIsNotParsedError {
                calling_function: "get_bpm_by_beats",
            }));
        }
        let mut bpm = self.settings.bpm;
        'tile_loop: for tile in &self.tiles {
            for event in &tile.events {
                if let EventData::Dynamic {
                    event: DynamicEvents::SetSpeed(set_speed),
                    seconds: ss_seconds,
                    ..
                } = *event
                {
                    if seconds < ss_seconds.ok_or(DynamicValueEmptyError)? {
                        break 'tile_loop;
                    }
                    bpm = set_speed.get_bpm(bpm);
                }
            }
        }
        Ok(bpm)
    }
    pub fn get_bpm_by_floor_seconds(
        &self,
        floor: usize,
        seconds: f64,
    ) -> Result<f64, Box<dyn error::Error>> {
        if !self.parsed {
            return Err(Box::new(LevelIsNotParsedError {
                calling_function: "get_bpm_by_floor_seconds",
            }));
        }
        let mut bpm = self.settings.bpm;
        'tile_loop: for tile in &self.tiles {
            for event in &tile.events {
                if let EventData::Dynamic {
                    event: DynamicEvents::SetSpeed(set_speed),
                    floor: ss_floor,
                    seconds: ss_seconds,
                    ..
                } = *event
                {
                    if floor < ss_floor || seconds < ss_seconds.ok_or(DynamicValueEmptyError)? {
                        break 'tile_loop;
                    }
                    bpm = set_speed.get_bpm(bpm);
                }
            }
        }
        Ok(bpm)
    }
    pub fn planets_direction(
        &self,
        floor: usize,
        seconds: f64,
    ) -> Result<f64, Box<dyn error::Error>> {
        if !self.parsed {
            return Err(Box::new(LevelIsNotParsedError {
                calling_function: "planets_direction",
            }));
        }
        let bpm = self.get_bpm_by_floor_seconds(floor, seconds)?;
        let spb = bpm2crotchet(bpm);
        let angle;
        if floor == 0 {
            angle = -seconds / spb * 180.0;
        } else {
            let k = if self.tiles[floor].data.orbit.ok_or(DynamicValueEmptyError)?
                == Orbit::Clockwise
            {
                -1.0
            } else {
                1.0
            };
            angle = (if self.tiles[floor].angle == 999.0 {
                self.tiles[floor - 1].angle
            } else {
                self.tiles[floor].angle - 180.0
            }) + ((seconds
                - self.tiles[floor]
                    .data
                    .seconds
                    .ok_or(DynamicValueEmptyError)?)
                / spb
                * 180.0
                * k);
        }
        Ok(angle)
    }
    pub fn planets_position(
        &self,
        floor: usize,
        seconds: f64,
    ) -> Result<(Vector2D<f64>, Vector2D<f64>), Box<dyn error::Error>> {
        if !self.parsed {
            return Err(Box::new(LevelIsNotParsedError {
                calling_function: "planets_position",
            }));
        }
        let pos1;
        let pos2;
        pos1 = if self.tiles[floor]
            .data
            .stick_to_floors
            .ok_or(DynamicValueEmptyError)?
        {
            self.tiles[floor]
                .data
                .position
                .now
                .ok_or(DynamicValueEmptyError)?
        } else {
            self.tiles[floor]
                .data
                .position
                .orig
                .ok_or(DynamicValueEmptyError)?
        };
        let (sin, cos) = deg2rad(self.planets_direction(floor, seconds)?).sin_cos();
        pos2 = pos1.clone() + Vector2D::new(cos, sin);
        if floor % 2 == 0 {
            Ok((pos1, pos2))
        } else {
            Ok((pos2, pos1))
        }
    }
    pub fn get_floor_by_seconds(&self, seconds: f64) -> Result<usize, Box<dyn error::Error>> {
        if !self.parsed {
            return Err(Box::new(LevelIsNotParsedError {
                calling_function: "get_floor_by_seconds",
            }));
        }
        let mut floor = self.tiles.len() - 1;
        for (i, tile) in self.tiles.iter().enumerate() {
            if seconds < tile.data.seconds().unwrap() {
                floor = i;
                break;
            }
        }
        if floor > 0 {
            floor -= 1;
        }
        Ok(floor)
    }

    pub fn get_timing(&self, floor: usize, seconds: f64) -> Result<f64, Box<dyn error::Error>> {
        if !self.parsed {
            return Err(Box::new(LevelIsNotParsedError {
                calling_function: "get_timing",
            }));
        }
        Ok(seconds
            - self.tiles[floor]
                .data
                .seconds
                .ok_or(DynamicValueEmptyError)?)
    }
    pub fn get_hit_margin_bound(&self, floor: usize, difficulty: Difficulty) -> Result<(f64, f64, f64), Box<dyn error::Error>> {
        if !self.parsed {
            return Err(Box::new(LevelIsNotParsedError {
                calling_function: "get_hit_margin_bound",
            }));
        }
        let max_bpm: f64 = match difficulty {
            Difficulty::Lenient => 210.0,
            Difficulty::Normal => 330.0,
            Difficulty::Strict => 500.0,
        };
        let judge_sec = bpm2crotchet(max_bpm.min(self.get_bpm_excluding_beats(
            self.tiles[floor].data.beats.ok_or(DynamicValueEmptyError)?,
        )?));
        let p = judge_sec / 6.0;
        let lep = judge_sec / 4.0;
        let vle = judge_sec / 3.0;
        Ok((p, lep, vle))
    }
    pub fn get_hit_margin(
        &self,
        floor: usize,
        seconds: f64,
        difficulty: Difficulty,
    ) -> Result<(HitMargin, f64), Box<dyn error::Error>> {
        if !self.parsed {
            return Err(Box::new(LevelIsNotParsedError {
                calling_function: "get_hit_margin",
            }));
        }
        // let max_bpm: f64 = match difficulty {
        //     Difficulty::Lenient => 210.0,
        //     Difficulty::Normal => 330.0,
        //     Difficulty::Strict => 500.0,
        // };
        // let judge_sec = bpm2crotchet(max_bpm.min(self.get_bpm_excluding_beats(
        //     self.tiles[floor].data.beats.ok_or(DynamicValueEmptyError)?,
        // )?));
        // let p = judge_sec / 6.0;
        // let lep = judge_sec / 4.0;
        // let vle = judge_sec / 3.0;
        let (p, lep, vle) = self.get_hit_margin_bound(floor, difficulty)?;
        let timing = self.get_timing(floor, seconds)?;
        Ok((
            match timing {
                _ if timing > vle => HitMargin::TooLate,
                _ if timing > lep => HitMargin::VeryLate,
                _ if timing > p => HitMargin::LatePerfect,
                _ if timing > -p => HitMargin::Perfect,
                _ if timing > -lep => HitMargin::EarlyPerfect,
                _ if timing > -vle => HitMargin::VeryEarly,
                _ => HitMargin::TooEarly,
            },
            timing,
        ))
    }
}

// #[derive(Getters)]
// #[getset(get = "pub")]
// pub struct PlayingLevel {
//     level: Level,
//     now_floor: usize,
//     player_floor: usize,
//     last_seconds: f64,
// }
// impl PlayingLevel {
//     pub fn new(level: Level) -> Self {
//         PlayingLevel {
//             level,
//             now_floor: 0,
//             player_floor: 0,
//             last_seconds: f64::NEG_INFINITY,
//         }
//     }
//     pub fn start(&mut self) {
//         self.now_floor = 0;
//         self.player_floor = 0;
//         self.last_seconds = 0.0;
//     }
//     pub fn update(&mut self, seconds: f64) {
//         self.now_floor = self.level.get_floor_by_seconds(seconds);
//         let result = self.update(seconds);

//         self.last_seconds = seconds;
//     }
//     pub fn input(&mut self, seconds: f64) -> (HitMargin, f64) {
//         loop {
//             if key_count.0 == 0 {
//                 break;
//             }
//             key_count.0 -= 1;
//             player_tile.0 += 1;
//             let (hit_margin, timing) = level
//                 .0
//                 .get_hit_margin(player_tile.0, seconds, difficulty.0)
//                 .unwrap();
//             if matches!(hit_margin, rusty_adofai::HitMargin::TooEarly) {
//                 player_tile.0 -= 1;
//             }
//         }
//         loop {
//             player_tile.0 += 1;
//             let (hit_margin, timing) = level
//                 .0
//                 .get_hit_margin(player_tile.0, seconds, difficulty.0)
//                 .unwrap();
//             if !matches!(hit_margin, rusty_adofai::HitMargin::TooLate) {
//                 player_tile.0 -= 1;
//                 break;
//             }
//         }
//     }
//     pub fn stop(&self) {}
// }
