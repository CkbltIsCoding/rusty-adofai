use crate::*;
use serde::{Deserialize, Serialize};
use serde_json_lenient::Value;
use vector2d::Vector2D;

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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub version: u32,
    pub artist: String,
    pub song: String,
    pub author: String,
    pub separate_countdown_time: bool,
    pub song_filename: String,
    pub bpm: f64,
    pub volume: f64,
    pub offset: f64,
    pub pitch: f64,
    pub countdown_ticks: i32,
}

pub struct Level {
    pub tiles: Vec<Tile>,
    pub settings: Settings,
}
impl From<&Value> for Level {
    fn from(item: &Value) -> Self {
        let json = item.as_object().unwrap();
        let settings: Settings = serde_json_lenient::from_value(item["settings"].clone()).unwrap();
        let mut tiles: Vec<Tile> = vec![];
        if json.contains_key("angleData") {
            for angle in json["angleData"].as_array().unwrap() {
                tiles.push(Tile::new(angle.as_f64().unwrap()));
            }
        }
        for data in json["actions"].as_array().unwrap() {
            let object = data.as_object().unwrap();
            let floor = object["floor"].as_u64().unwrap() as usize;
            let event_type = object["eventType"].as_str().unwrap();
            match event_type {
                "Twirl" => {
                    let twirl = Twirl::try_from(data).unwrap();
                    tiles[floor].events.push(Events::Twirl(twirl));
                }
                "SetSpeed" => {
                    let set_speed = SetSpeed::try_from(data).unwrap();
                    tiles[floor].events.push(Events::SetSpeed(set_speed));
                }
                "Pause" => {
                    let pause = Pause::try_from(data).unwrap();
                    tiles[floor].events.push(Events::Pause(pause));
                }
                "ScaleRadius" => {
                    let scale_radius = ScaleRadius::try_from(data).unwrap();
                    tiles[floor].events.push(Events::ScaleRadius(scale_radius));
                }
                _ => {}
            }
        }
        Level { tiles, settings }
    }
}
impl Level {
    pub fn parse(&mut self) {
        let tiles = &mut self.tiles;
        let length = tiles.len();
        tiles[0].tile_data = Default::default();
        tiles[0].tile_data.orbit = Some(Orbit::Clockwise);
        tiles[0].tile_data.beat = Some(0.0);
        tiles[0].tile_data.radius_scale = Some(100.0);
        tiles[0].tile_data.position = BeatValue {
            orig: Some(Default::default()),
            now: None,
        };
        tiles[0].tile_data.editor_position = Some(Default::default());
        for i in 0..length {
            if i != 0 {
                tiles[i].tile_data = tiles[i - 1].tile_data.clone();
            }
            let tile = &mut tiles[i];
            let mut pause_duration = 0.0;
            for event in &tile.events {
                match event {
                    Events::Twirl(_) => {
                        tile.tile_data.orbit = Some(tile.tile_data.orbit.unwrap().opposite());
                    }
                    Events::Pause(pause) => {
                        pause_duration = pause.duration;
                    }
                    Events::ScaleRadius(scale_radius) => {
                        tile.tile_data.radius_scale = Some(scale_radius.scale)
                    }
                    _ => {}
                }
            }
            if i != 0 {
                if tiles[i].angle == 999.0 {
                    tiles[i].tile_data.beat = tiles[i - 1].tile_data.beat;
                } else {
                    let k = if tiles[i - 1].tile_data.orbit.unwrap() == Orbit::Clockwise {
                        1.0
                    } else {
                        -1.0
                    };
                    let b = if i == 1 { -180.0 } else { 0.0 };
                    let angle = if tiles[i - 1].angle == 999.0 {
                        tiles[i - 2].angle - tiles[i - 1].angle
                    } else {
                        tiles[i - 1].angle - 180.0 - tiles[i].angle
                    };
                    let included_angle = (angle * k).rem_euclid(360.0) + b;
                    let gap_beat = included_angle / 180.0 + pause_duration;
                    tiles[i].tile_data.beat = Some(tiles[i - 1].tile_data.beat.unwrap() + gap_beat);
                }
                tiles[i].tile_data.position = tiles[i - 1].tile_data.position;
                tiles[i].tile_data.editor_position = tiles[i - 1].tile_data.editor_position;
                if (i == length - 1 || tiles[i + 1].angle != 999.0) && tiles[i].angle != 999.0 {
                    let (sin, cos) = deg2rad(tiles[i].angle).sin_cos();
                    let vec2d =
                        Vector2D::new(cos, sin) * tiles[i].tile_data.radius_scale.unwrap() / 100.0;
                    *tiles[i].tile_data.position.orig.as_mut().unwrap() += vec2d;
                    *tiles[i].tile_data.editor_position.as_mut().unwrap() += vec2d;
                }
            }
        }
        tiles[0].tile_data.beat = Some(-self.settings.countdown_ticks as f64);
        for tile in &mut self.tiles {
            for event in &mut tile.events {
                match event {
                    Events::SetSpeed(set_speed) => {
                        set_speed.beat =
                            Some(tile.tile_data.beat.unwrap() + set_speed.angle_offset / 180.0);
                    }
                    _ => {}
                }
            }
        }

        // for i in 0..length {
        //     self.tiles[i].tile_data.ms =
        //         self.beat2ms(self.tiles[i].tile_data.beat.unwrap());
        // }
    }

    pub fn beat2ms(&self, beat: f64) -> Option<f64> {
        let mut bpm = self.settings.bpm;
        let mut last_beat = 0.0;
        let mut ms = self.settings.offset;
        'tile_loop: for tile in &self.tiles {
            for event in &tile.events {
                if let Events::SetSpeed(set_speed) = event {
                    if beat > set_speed.beat? {
                        ms += bpm2mspb(bpm) * (set_speed.beat? - last_beat);
                    } else {
                        break 'tile_loop;
                    }
                    bpm = set_speed.get_bpm(bpm);
                    last_beat = set_speed.beat?;
                }
            }
        }
        ms += bpm2mspb(bpm) * (beat - last_beat);
        Some(ms)
    }
    pub fn ms2beat(&self, mut ms: f64) -> Option<f64> {
        ms -= self.settings.offset;
        let mut bpm = self.settings.bpm;
        let mut last_beat = 0.0;
        let mut beat = 0.0;
        'tile_loop: for tile in &self.tiles {
            for event in &tile.events {
                if let Events::SetSpeed(set_speed) = event {
                    let gap_ms = bpm2mspb(bpm) * (set_speed.beat? - last_beat);
                    if ms > gap_ms {
                        ms -= gap_ms;
                    } else {
                        break 'tile_loop;
                    }
                    beat += set_speed.beat? - last_beat;
                    bpm = set_speed.get_bpm(bpm);
                    last_beat = set_speed.beat?;
                }
            }
        }
        beat += ms / bpm2mspb(bpm);
        Some(beat)
    }
}
