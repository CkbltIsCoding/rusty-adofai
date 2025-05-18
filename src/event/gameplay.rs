use super::*;
use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSpeed {
    #[serde(default)]
    pub speed_type: SpeedType,
    pub beats_per_minute: f64,
    #[serde(default)]
    pub bpm_multiplier: f64,
    #[serde(default)]
    pub angle_offset: f64,
}
impl SetSpeed {
    pub const fn get_bpm(&self, orig_bpm: f64) -> f64 {
        match self.speed_type {
            SpeedType::Bpm => self.beats_per_minute,
            SpeedType::Multiplier => orig_bpm * self.bpm_multiplier,
        }
    }
}
impl Event for SetSpeed {}
impl DynamicEvent for SetSpeed {
    fn apply(
        &self, _data: (usize, f64, f64, Option<Vec<String>>), _level: &mut Level, _seconds: f64
    ) -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }
    fn angle_offset(&self) -> f64 {
        self.angle_offset
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum SpeedType {
    #[default]
    Bpm,
    Multiplier,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Twirl {}
impl Event for Twirl {}
impl StaticEvent for Twirl {
    fn apply(&self, data: &mut TileData) {
        data.orbit = Some(data.orbit.unwrap().opposite());
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pause {
    pub duration: f64,
    pub countdown_ticks: f64,
    pub angle_correction_dir: AngleCorrectionDir,
}
impl Event for Pause {}
impl StaticEvent for Pause {
    fn apply(&self, data: &mut TileData) {
        data.pause_duration = Some(self.duration);
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GameSound {
    Hitsound,
    Midspin,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetHitsound {
    pub game_sound: GameSound,
    pub hitsound: Hitsound,
    pub hitsound_volume: f64,
}
impl Event for SetHitsound {
}
impl StaticEvent for SetHitsound {
    fn apply(&self, data: &mut TileData) {
        match self.game_sound {
            GameSound::Hitsound => {
                data.hitsound = Some(self.hitsound);
                data.hitsound_volume = Some(self.hitsound_volume);
            }
            GameSound::Midspin => {
                data.midspin_hitsound = Some(self.hitsound);
                data.midspin_hitsound_volume = Some(self.hitsound_volume);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AngleCorrectionDir {
    Backward = -1,
    None = 0,
    Forward = 1,
}
impl TryFrom<isize> for AngleCorrectionDir {
    type Error = ();

    fn try_from(v: isize) -> Result<Self, Self::Error> {
        match v {
            -1 => Ok(AngleCorrectionDir::Backward),
            0 => Ok(AngleCorrectionDir::None),
            1 => Ok(AngleCorrectionDir::Forward),
            _ => Err(()),
        }
    }
}
impl TryFrom<&str> for AngleCorrectionDir {
    type Error = ();

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        match v {
            "Backward" => Ok(AngleCorrectionDir::Backward),
            "None" => Ok(AngleCorrectionDir::None),
            "Forward" => Ok(AngleCorrectionDir::Forward),
            _ => Err(()),
        }
    }
}
