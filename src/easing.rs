use serde::{Deserialize, Serialize};

const PI: f64 = std::f64::consts::PI;
const C1: f64 = 1.70158;
const C2: f64 = C1 * 1.525;
const C3: f64 = C1 + 1.0;
const C4: f64 = (2.0 * PI) / 3.0;
const C5: f64 = (2.0 * PI) / 4.5;
const N1: f64 = 7.5625;
const D1: f64 = 2.75;
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Easing {
    Linear,
    InSine,
    OutSine,
    InOutSine,
    InQuad,
    OutQuad,
    InOutQuad,
    InCubic,
    OutCubic,
    InOutCubic,
    InQuart,
    OutQuart,
    InOutQuart,
    InQuint,
    OutQuint,
    InOutQuint,
    InExpo,
    OutExpo,
    InOutExpo,
    InCirc,
    OutCirc,
    InOutCirc,
    InBack,
    OutBack,
    InOutBack,
    InElastic,
    OutElastic,
    InOutElastic,
    InBounce,
    OutBounce,
    InOutBounce,
    InFlash,
    OutFlash,
    InOutFlash,
}
impl Easing {
    pub fn calc(&self, x: f64) -> f64 {
        if x <= 0.0 {
            return 0.0;
        }
        if x >= 1.0 {
            return 1.0;
        }
        match self {
            Self::Linear => x,
            Self::InSine => 1.0 - ((x * PI) / 2.0).cos(),
            Self::OutSine => ((x * PI) / 2.0).sin(),
            Self::InOutSine => -((PI * x).cos() - 1.0) / 2.0,
            Self::InQuad => x * x,
            Self::OutQuad => 1.0 - (1.0 - x) * (1.0 - x),
            Self::InOutQuad => {
                if x < 0.5 {
                    2.0 * x * x
                } else {
                    1.0 - (-2.0 * x + 2.0) * (-2.0 * x + 2.0) / 2.0
                }
            }
            Self::InCubic => x * x * x,
            Self::OutCubic => 1.0 - (1.0 - x) * (1.0 - x) * (1.0 - x),
            Self::InOutCubic => {
                if x < 0.5 {
                    4.0 * x * x * x
                } else {
                    1.0 - (-2.0 * x + 2.0) * (-2.0 * x + 2.0) * (-2.0 * x + 2.0) / 2.0
                }
            }
            Self::InQuart => x * x * x * x,
            Self::OutQuart => 1.0 - (1.0 - x) * (1.0 - x) * (1.0 - x) * (1.0 - x),
            Self::InOutQuart => {
                if x < 0.5 {
                    8.0 * x * x * x * x
                } else {
                    1.0 - (-2.0 * x + 2.0) * (-2.0 * x + 2.0) * (-2.0 * x + 2.0) * (-2.0 * x + 2.0)
                        / 2.0
                }
            }
            Self::InQuint => x * x * x * x * x,
            Self::OutQuint => 1.0 - (1.0 - x) * (1.0 - x) * (1.0 - x) * (1.0 - x) * (1.0 - x),
            Self::InOutQuint => {
                if x < 0.5 {
                    16.0 * x * x * x * x * x
                } else {
                    1.0 - (-2.0 * x + 2.0)
                        * (-2.0 * x + 2.0)
                        * (-2.0 * x + 2.0)
                        * (-2.0 * x + 2.0)
                        * (-2.0 * x + 2.0)
                        / 2.0
                }
            }
            Self::InExpo => 2f64.powf(10.0 * x - 10.0),
            Self::OutExpo => 1.0 - 2f64.powf(-10.0 * x),
            Self::InOutExpo => {
                if x < 0.5 {
                    2f64.powf(20.0 * x - 10.0) / 2.0
                } else {
                    (2.0 - 2f64.powf(-20.0 * x + 10.0)) / 2.0
                }
            }
            Self::InCirc => 1.0 - (1.0 - x * x).sqrt(),
            Self::OutCirc => (1.0 - (x - 1.0) * (x - 1.0)).sqrt(),
            Self::InOutCirc => {
                if x < 0.5 {
                    (1.0 - (1.0 - (2.0 * x).powf(2.0)).sqrt()) / 2.0
                } else {
                    ((1.0 - (-2.0 * x + 2.0).powf(2.0)).sqrt() + 1.0) / 2.0
                }
            }
            Self::InBack => C3 * x * x * x - C1 * x * x,
            Self::OutBack => 1.0 + C3 * (x - 1.0).powf(3.0) + C1 * (x - 1.0).powf(2.0),
            Self::InOutBack => {
                if x < 0.5 {
                    ((2.0 * x).powf(2.0) * ((C2 + 1.0) * 2.0 * x - C2)) / 2.0
                } else {
                    ((2.0 * x - 2.0).powf(2.0) * ((C2 + 1.0) * (x * 2.0 - 2.0) + C2) + 2.0) / 2.0
                }
            }
            Self::InElastic => -2f64.powf(10.0 * x - 10.0) * ((x * 10.0 - 10.75) * C4).sin(),
            Self::OutElastic => 2f64.powf(-10.0 * x) * ((x * 10.0 - 0.75) * C4).sin() + 1.0,
            Self::InOutElastic => {
                if x < 0.5 {
                    -(2f64.powf(20.0 * x - 10.0) * ((20.0 * x - 11.125) * C5).sin()) / 2.0
                } else {
                    (2f64.powf(-20.0 * x + 10.0) * ((20.0 * x - 11.125) * C5).sin()) / 2.0 + 1.0
                }
            }
            Self::InBounce => 1.0 - out_bounce(1.0 - x),
            Self::OutBounce => out_bounce(x),
            Self::InOutBounce => {
                if x < 0.5 {
                    (1.0 - out_bounce(1.0 - 2.0 * x)) / 2.0
                } else {
                    (1.0 + out_bounce(2.0 * x - 1.0)) / 2.0
                }
            }
            _ => 1.0,
        }
    }
}

const fn out_bounce(mut x: f64) -> f64 {
    if x < 1.0 / D1 {
        N1 * x * x
    } else if x < 2.0 / D1 {
        x -= 1.5;
        N1 * (x / D1) * x + 0.75
    } else if x < 2.5 / D1 {
        x -= 2.25;
        N1 * (x / D1) * x + 0.9375
    } else {
        x -= 2.625;
        N1 * (x / D1) * x + 0.984375
    }
}
