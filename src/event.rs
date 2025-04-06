use serde_json_lenient::Value;

#[derive(Debug, Clone, Copy)]
pub enum Events {
    Twirl(Twirl),
    SetSpeed(SetSpeed),
    Pause(Pause),
    ScaleRadius(ScaleRadius),
}

pub trait Event: std::fmt::Debug {
}
pub trait BeatEvent: Event {
    fn beat(&self) -> Option<f64>;
    fn timer(&self) -> Option<f64>;
}
pub trait TagBeatEvent: BeatEvent {
    fn tags(&self) -> &Vec<&str>;
}

#[derive(Debug, Clone, Copy)]
pub struct Twirl {
}
impl Event for Twirl {
}
impl TryFrom<&Value> for Twirl {
    type Error = ();

    fn try_from(_v: &Value) -> Result<Self, Self::Error> {
        Ok(Twirl {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SetSpeed {
    pub(crate) beat: Option<f64>,
    pub(crate) timer: Option<f64>,
    pub speed: Speed,
    pub angle_offset: f64,
}
impl SetSpeed {
    pub const fn get_bpm(&self, orig_bpm: f64) -> f64 {
        match self.speed {
            Speed::Bpm(ss_bpm) => ss_bpm,
            Speed::Multiplier(mul) => orig_bpm * mul
        }
    }
}
impl Event for SetSpeed {
}
impl BeatEvent for SetSpeed {
    fn beat(&self) -> Option<f64> {
        self.beat
    }
    fn timer(&self) -> Option<f64> {
        self.timer
    }
}
impl TryFrom<&Value> for SetSpeed {
    type Error = ();

    fn try_from(v: &Value) -> Result<Self, Self::Error> {
        let object = v.as_object().ok_or(())?;
        Ok(SetSpeed {
            beat: None,
            timer: None,
            speed: if object["speedType"] == "Bpm" {
                Speed::Bpm(object["beatsPerMinute"].as_f64().ok_or(())?)
            } else {
                Speed::Multiplier(object["bpmMultiplier"].as_f64().ok_or(())?)
            },
            angle_offset: object["angleOffset"].as_f64().ok_or(())?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Speed {
    Bpm(f64),
    Multiplier(f64),
}

#[derive(Debug, Clone, Copy)]
pub struct Pause {
    pub duration: f64,
    pub countdown: f64,
    pub angle_correction_dir: AngleCorrectionDir,
}
impl Event for Pause {
}
impl TryFrom<&Value> for Pause {
    type Error = ();

    fn try_from(item: &Value) -> Result<Self, Self::Error> {
        let acd = &item["angleCorrectionDir"];
        Ok(Pause {
            duration: item["duration"].as_f64().ok_or(())?,
            countdown: item["countdownTicks"].as_f64().ok_or(())?,
            angle_correction_dir: if acd.is_string() {
                acd.as_str().unwrap().try_into()?
            } else {
                (acd.as_i64().ok_or(())? as isize).try_into()?
            },
        })
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct ScaleRadius {
    pub scale: f64,
}
impl Event for ScaleRadius {
}
impl TryFrom<&Value> for ScaleRadius {
    type Error = ();
    fn try_from(v: &Value) -> Result<Self, Self::Error> {
        Ok(ScaleRadius { scale: v["scale"].as_f64().ok_or(())? })
    }
}
