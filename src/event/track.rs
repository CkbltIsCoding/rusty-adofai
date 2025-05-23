use super::*;
use crate::*;
use rgb::Rgba;
use serde::{Deserialize, Serialize};
use std::error;
use vector2d::Vector2D;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorTrack {
    pub floor: usize,
    pub track_color_type: TrackColorType,
    #[serde(serialize_with = "ser_rgba_u8", deserialize_with = "de_rgba_u8")]
    pub track_color: Rgba<u8>,
    #[serde(serialize_with = "ser_rgba_u8", deserialize_with = "de_rgba_u8")]
    pub secondary_track_color: Rgba<u8>,
    pub track_color_anim_duration: f64,
    pub track_color_pulse: TrackColorPulse,
    pub track_pulse_length: u32,
    pub track_style: TrackStyle,
}
impl Event for ColorTrack {
    fn floor(&self) -> usize {
        self.floor
    }
    fn floor_mut(&mut self) -> &mut usize {
        &mut self.floor
    }
    fn set_floor(&mut self, new_floor: usize) {
        self.floor = new_floor
    }
}
impl StaticEvent for ColorTrack {
    fn apply(&self, data: &mut TileData) {
        data.color_type.orig = Some(self.track_color_type);
        data.color.orig = Some(self.track_color);
        data.secondary_color.orig = Some(self.secondary_track_color);
        data.color_anim_duration.orig = Some(self.track_color_anim_duration);
        data.color_pulse.orig = Some(self.track_color_pulse);
        data.pulse_length.orig = Some(self.track_pulse_length);
        data.style.orig = Some(self.track_style);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecolorTrack {
    pub floor: usize,
    #[serde(serialize_with = "ser_event_tag", deserialize_with = "de_event_tag")]
    pub event_tag: Vec<String>,
    pub angle_offset: f64,
    pub start_tile: RelativeIndex,
    pub end_tile: RelativeIndex,
    #[serde(default)]
    pub gap_length: u32,
    pub track_color_type: TrackColorType,
    #[serde(serialize_with = "ser_rgba_u8", deserialize_with = "de_rgba_u8")]
    pub track_color: Rgba<u8>,
    #[serde(serialize_with = "ser_rgba_u8", deserialize_with = "de_rgba_u8")]
    pub secondary_track_color: Rgba<u8>,
    pub track_color_anim_duration: f64,
    pub track_color_pulse: TrackColorPulse,
    pub track_pulse_length: u32,
    pub track_style: TrackStyle,
}
impl Event for RecolorTrack {
    fn floor(&self) -> usize {
        self.floor
    }
    fn floor_mut(&mut self) -> &mut usize {
        &mut self.floor
    }
    fn set_floor(&mut self, new_floor: usize) {
        self.floor = new_floor
    }
}
impl DynamicEvent for RecolorTrack {
    fn angle_offset(&self) -> f64 {
        self.angle_offset
    }
    fn apply(
        &self,
        data: (f64, f64),
        level: &mut Level,
        seconds: f64,
    ) -> Result<(), Box<dyn error::Error>> {
        let (_e_beats, e_seconds) = data;
        if seconds < e_seconds {
            return Ok(());
        }
        // let bpm = level.get_bpm_by_seconds(e_seconds)?;
        let bpm = level.get_bpm_by_floor_seconds(self.floor, e_seconds)?;
        let spb = bpm2crotchet(bpm);
        let start = self.start_tile.calc(self.floor, level.tiles.len() - 1);
        let end = self.end_tile.calc(self.floor, level.tiles.len() - 1);
        for f in start..=end.min(level.tiles.len() - 1) {
            if seconds < e_seconds + (f as f64 * self.gap_length as f64) * spb {
                return Ok(());
            }
            let data = &mut level.tiles[f].data;
            data.color_type.now = Some(self.track_color_type);
            data.color.now = Some(self.track_color);
            data.secondary_color.now = Some(self.secondary_track_color);
            data.color_anim_duration.now = Some(self.track_color_anim_duration);
            data.color_pulse.now = Some(self.track_color_pulse);
            data.pulse_length.now = Some(self.track_pulse_length);
            data.style.now = Some(self.track_style);
        }
        Ok(())
    }
    fn has_event_tag() -> bool {
        true
    }
    fn event_tag(&self) -> Option<&Vec<String>> {
        Some(&self.event_tag)
    }
    fn event_tag_mut(&mut self) -> Option<&mut Vec<String>> {
        Some(&mut self.event_tag)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveTrack {
    pub floor: usize,
    #[serde(serialize_with = "ser_event_tag", deserialize_with = "de_event_tag")]
    pub event_tag: Vec<String>,
    #[serde(default)]
    pub angle_offset: f64,
    pub start_tile: RelativeIndex,
    pub end_tile: RelativeIndex,
    #[serde(default)]
    pub gap_length: f64,
    pub duration: f64,
    #[serde(
        serialize_with = "ser_vector2d_option_f64",
        deserialize_with = "de_vector2d_option_f64"
    )]
    pub position_offset: Vector2D<Option<f64>>,
    #[serde(default)]
    pub rotation_offset: Option<f64>,
    #[serde(
        default,
        serialize_with = "ser_vector2d_option_f64",
        deserialize_with = "de_vector2d_option_f64"
    )]
    pub scale: Vector2D<Option<f64>>,
    #[serde(default)]
    pub opacity: Option<f64>,
    pub ease: Easing,
}
impl Event for MoveTrack {
    fn floor(&self) -> usize {
        self.floor
    }
    fn floor_mut(&mut self) -> &mut usize {
        &mut self.floor
    }
    fn set_floor(&mut self, new_floor: usize) {
        self.floor = new_floor
    }
}
impl DynamicEvent for MoveTrack {
    fn angle_offset(&self) -> f64 {
        self.angle_offset
    }
    fn apply(
        &self,
        data: (f64, f64),
        level: &mut Level,
        seconds: f64,
    ) -> Result<(), Box<dyn error::Error>> {
        let (_e_beats, e_seconds) = data;
        // let bpm = level.get_bpm_by_seconds(e_seconds)?;
        let bpm = level.get_bpm_by_floor_seconds(self.floor, e_seconds)?;
        let spb = bpm2crotchet(bpm);
        let (x, y);
        if seconds < e_seconds {
            return Ok(());
        }
        if self.duration == 0.0 {
            /* x = 1.0; */
            y = 1.0;
        } else {
            x = (seconds - e_seconds) / spb / self.duration;
            y = self.ease.calc(x);
        }

        let start = self.start_tile.calc(self.floor, level.tiles.len() - 1);
        let end = self.end_tile.calc(self.floor, level.tiles.len() - 1);
        for f in start..=end.min(level.tiles.len() - 1) {
            let data = &mut level.tiles[f].data;
            let now_position = data.position.now.as_mut().unwrap();
            let orig_position = data.position.orig.unwrap();
            let now_rotation = data.rotation.now.as_mut().unwrap();
            let now_scale = data.scale.now.as_mut().unwrap();
            let now_opacity = data.opacity.now.as_mut().unwrap();
            if let Some(offset_x) = self.position_offset.x {
                now_position.x += (orig_position.x + offset_x - now_position.x) * y;
            }
            if let Some(offset_y) = self.position_offset.y {
                now_position.y += (orig_position.y + offset_y - now_position.y) * y;
            }
            if let Some(offset) = self.rotation_offset {
                *now_rotation += (offset - *now_rotation) * y;
            }
            if let Some(scale_x) = self.scale.x {
                now_scale.x += (scale_x - now_scale.x) * y;
            }
            if let Some(scale_y) = self.scale.y {
                now_scale.y += (scale_y - now_scale.y) * y;
            }
            if let Some(opacity) = self.opacity {
                *now_opacity += (opacity - *now_opacity) * y;
            }
        }
        Ok(())
    }
    fn has_event_tag() -> bool {
        true
    }
    fn event_tag(&self) -> Option<&Vec<String>> {
        Some(&self.event_tag)
    }
    fn event_tag_mut(&mut self) -> Option<&mut Vec<String>> {
        Some(&mut self.event_tag)
    }
}

const fn relative_to_default() -> RelativeIndex {
    RelativeIndex {
        index: 0,
        relative_to: RelativeToTile::ThisTile,
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionTrack {
    pub floor: usize,
    #[serde(
        default,
        serialize_with = "ser_vector2d_f64",
        deserialize_with = "de_vector2d_f64"
    )]
    pub position_offset: Vector2D<f64>,
    #[serde(default = "relative_to_default")]
    pub relative_to: RelativeIndex,
    #[serde(default)]
    pub scale: f64,
    #[serde(default, deserialize_with = "de_bool")]
    pub just_this_tile: bool,
    #[serde(deserialize_with = "de_bool")]
    pub editor_only: bool,
    #[serde(default, deserialize_with = "de_bool")]
    pub stick_to_floors: bool,
}
impl Event for PositionTrack {
    fn floor(&self) -> usize {
        self.floor
    }
    fn floor_mut(&mut self) -> &mut usize {
        &mut self.floor
    }
    fn set_floor(&mut self, new_floor: usize) {
        self.floor = new_floor
    }
}
impl StaticEvent for PositionTrack {
    fn apply(&self, _data: &mut TileData) {}
}
