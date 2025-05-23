use crate::*;
use event::*;
use std::{error, fmt};
use vector2d::Vector2D;

#[derive(Debug)]
struct LevelParseError {
    msg: String,
}
impl fmt::Display for LevelParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
impl error::Error for LevelParseError {}

impl Level {
    pub fn parse(&mut self) -> Result<(), Box<dyn error::Error>> {
        if self.parsed {
            return Ok(());
        }
        self.parsed = true;
        let tiles = &mut self.tiles;
        let length = tiles.len();

        if length < 2 {
            return Err(Box::from(LevelParseError {
                msg: format!("Tiles are not enough (level.tiles.len() < 2)."),
            }));
        }

        let data = &mut tiles[0].data;
        *data = Default::default();
        data.orbit = Some(Orbit::Clockwise);
        data.hitsound = Some(self.settings.hitsound);
        data.midspin_hitsound = Some(self.settings.hitsound);
        data.hitsound_volume = Some(self.settings.hitsound_volume);
        data.midspin_hitsound_volume = Some(self.settings.hitsound_volume);
        data.beats = Some(0.0);
        data.stick_to_floors = Some(self.settings.stick_to_floors);
        data.radius_scale = Some(100.0);
        data.editor_position = Some(Vector2D::new(0.0, 0.0));
        data.position = DynamicValue {
            orig: Some(Vector2D::new(0.0, 0.0)),
            now: None,
        };
        data.scale = DynamicValue {
            orig: Some(Vector2D::new(100.0, 100.0)),
            now: None,
        };
        data.rotation.orig = Some(0.0);
        data.opacity.orig = Some(100.0);
        data.color_type.orig = Some(self.settings.track_color_type);
        data.color.orig = Some(self.settings.track_color);
        data.secondary_color.orig = Some(self.settings.secondary_track_color);
        data.color_anim_duration.orig = Some(self.settings.track_color_anim_duration);
        data.color_pulse.orig = Some(self.settings.track_color_pulse);
        data.pulse_length.orig = Some(self.settings.track_pulse_length);
        data.style.orig = Some(self.settings.track_style);

        let mut last_position_offset = (Vector2D::new(0.0, 0.0), Vector2D::new(0.0, 0.0));
        for i in 0..length {
            let i_is_first = i == 0;
            let i_is_last = i == length - 1;
            if !i_is_first {
                tiles[i].data = tiles[i - 1].data.clone();
            }

            tiles[i].data.pause_duration = Some(0.0);
            tiles[i].data.hold_duration = Some(0.0);

            let mut option_position_track: Option<PositionTrack> = None;

            for event in tiles[i].events.clone() {
                if let EventData::Static {
                    event: static_event,
                    ..
                } = event
                {
                    static_event.apply(&mut tiles[i].data);
                    if let StaticEvents::PositionTrack(position_track_event) = static_event {
                        option_position_track = Some(position_track_event);
                    }
                }
            }

            if !i_is_first {
                if tiles[i].angle == 999.0 {
                    tiles[i].data.beats = tiles[i - 1].data.beats;
                } else {
                    let mut angle;
                    let beats;
                    if tiles[i - 1].angle == 999.0 {
                        angle = tiles[i - 2].angle - tiles[i].angle;
                    } else {
                        angle = tiles[i - 1].angle - 180.0 - tiles[i].angle;
                    }
                    if tiles[i - 1].data.orbit.unwrap() == Orbit::Anticlockwise {
                        angle *= -1.0;
                    }
                    angle = angle.rem_euclid(360.0);
                    if angle == 0.0 {
                        angle += 360.0
                    }
                    if i == 1 {
                        angle -= 180.0;
                    }
                    beats = angle / 180.0
                        + tiles[i - 1].data.pause_duration.unwrap()
                        + tiles[i - 1].data.hold_duration.unwrap();
                    *tiles[i].data.beats.as_mut().unwrap() += beats;
                }

                if !i_is_last {
                    tiles[i + 1].data.position.orig = Some(Vector2D::new(0.0, 0.0));
                    tiles[i + 1].data.editor_position = Some(Vector2D::new(0.0, 0.0));
                }
                if (i_is_last || tiles[i + 1].angle != 999.0) && tiles[i].angle != 999.0 {
                    let (sin, cos) = deg2rad(tiles[i].angle).sin_cos();
                    let vec2d =
                        Vector2D::new(cos, sin) * tiles[i].data.radius_scale.unwrap() / 100.0;
                    *tiles[i].data.position.orig.as_mut().unwrap() +=
                        vec2d + last_position_offset.0;
                    *tiles[i].data.editor_position.as_mut().unwrap() +=
                        vec2d + last_position_offset.1;
                }
                last_position_offset = (Vector2D::new(0.0, 0.0), Vector2D::new(0.0, 0.0));
                if let Some(position_track) = option_position_track {
                    *tiles[i].data.editor_position.as_mut().unwrap() +=
                        position_track.position_offset;
                    if !position_track.editor_only {
                        *tiles[i].data.position.orig.as_mut().unwrap() +=
                            position_track.position_offset;
                    }
                    if position_track.just_this_tile && !i_is_last {
                        last_position_offset.1 = -position_track.position_offset;
                        if !position_track.editor_only {
                            last_position_offset.0 = -position_track.position_offset;
                        }
                    }
                    tiles[i].data.stick_to_floors = Some(position_track.stick_to_floors);
                }
            }
        }
        tiles[0].data.beats = Some(-(self.settings.countdown_ticks as f64));
        for tile in &mut self.tiles {
            for event in &mut tile.events {
                if let EventData::Dynamic {
                    event: DynamicEvents::SetSpeed(set_speed),
                    beats: ss_beats,
                    ..
                } = event
                {
                    *ss_beats = Some(tile.data.beats.unwrap() + set_speed.angle_offset / 180.0);
                }
            }
        }

        self.dynamic_events.clear();
        let mut repeat_events = vec![];
        for floor in 0..length {
            self.tiles[floor].data.seconds = Some(
                self.beats2seconds(self.tiles[floor].data.beats.unwrap())
                    .unwrap(),
            );
            for event_index in 0..self.tiles[floor].events.len() {
                let mut beats = 0.0;
                let mut seconds = 0.0;
                if let EventData::Dynamic {
                    event: dynamic_event,
                    beats: e_beats,
                    ..
                } = &self.tiles[floor].events[event_index]
                {
                    if let DynamicEvents::SetSpeed(set_speed) = dynamic_event {
                        beats = e_beats.unwrap() + set_speed.angle_offset / 180.0;
                        seconds = self.beats2seconds(e_beats.unwrap()).unwrap();
                    } else {
                        let bpm = self.get_bpm_until(|ss, _, _| {
                            floor < ss.floor || dynamic_event.angle_offset() < ss.angle_offset
                        })?;
                        let spb = bpm2crotchet(bpm);
                        seconds = self.tiles[floor].data.seconds.unwrap()
                            + dynamic_event.angle_offset() / 180.0 * spb;
                        beats = self.seconds2beats(seconds).unwrap();
                    }
                }
                if let EventData::Dynamic {
                    event: dyn_e,
                    beats: e_beats,
                    seconds: e_seconds,
                    ..
                } = &mut self.tiles[floor].events[event_index]
                {
                    *e_beats = Some(beats);
                    *e_seconds = Some(seconds);

                    if let DynamicEvents::RepeatEvents(_) = dyn_e {
                        repeat_events.push(self.tiles[floor].events[event_index].clone());
                    }
                    self.dynamic_events
                        .push(self.tiles[floor].events[event_index].clone());
                }
            }
        }
        fn match_tag(re_tags: &Vec<String>, e_tags: &Vec<String>) -> bool {
            for re_tag in re_tags {
                if e_tags.contains(re_tag) {
                    return true;
                }
            }
            false
        }
        for re_data in repeat_events {
            let EventData::Dynamic { event: re_dyn, .. } = re_data else {
                unreachable!()
            };
            let DynamicEvents::RepeatEvents(re) = re_dyn else {
                unreachable!()
            };
            match re.repeat_type {
                RepeatType::Beat => {
                    for event_data in &self.tiles[re.floor].events {
                        let EventData::Dynamic {
                            event,
                            seconds: Some(seconds),
                            ..
                        } = event_data
                        else {
                            continue;
                        };
                        if !event.self_has_event_tag()
                            || !match_tag(&re.tag, event.event_tag().unwrap())
                        {
                            continue;
                        }
                        let spb = self.get_bpm_by_floor_seconds(event.floor(), *seconds)?;
                        for i in 1..=(re.repetitions + 1) {
                            let new_seconds = seconds + i as f64 * re.interval * spb;
                            let new_beats = self.seconds2beats(new_seconds)?;
                            self.dynamic_events.push(EventData::Dynamic {
                                event: event.clone(),
                                beats: Some(new_beats),
                                seconds: Some(new_seconds),
                            });
                        }
                    }
                }
                RepeatType::Floor => {
                    let Some(floor_count) = re.floor_count else {
                        unreachable!()
                    };
                    for event_data in &self.tiles[re.floor].events {
                        let EventData::Dynamic { event, .. } = event_data else {
                            continue;
                        };
                        if !event.self_has_event_tag()
                            || !match_tag(&re.tag, event.event_tag().unwrap())
                        {
                            continue;
                        }
                        let bpm = self.get_bpm_until(|ss, _, _| {
                            event.floor() < ss.floor || event.angle_offset() < ss.angle_offset
                        })?;
                        let spb = bpm2crotchet(bpm);
                        let offset = event.angle_offset() / 180.0 * spb;

                        for i in 1..=(floor_count + 1) {
                            let new_floor = event.floor() + i as usize;
                            let new_seconds = self.tiles[new_floor].data.seconds.unwrap() + offset;
                            let new_beats = self.seconds2beats(new_seconds)?;
                            let mut new_event = event.clone();
                            if re.execute_on_current_floor {
                                new_event.set_floor(new_floor)
                            };
                            self.dynamic_events.push(EventData::Dynamic {
                                event: new_event,
                                beats: Some(new_beats),
                                seconds: Some(new_seconds),
                            });
                        }
                    }
                }
            }
            // let dyn_e = EventData::Dynamic { event: (), floor: (), beats: (), seconds: (), tag: () }
            // let tile = self.tiles[event_data]
        }
        self.dynamic_events.sort_by(|a, b| {
            let EventData::Dynamic {
                seconds: Some(a_seconds),
                ..
            } = a
            else {
                unreachable!()
            };
            let EventData::Dynamic {
                seconds: Some(b_seconds),
                ..
            } = b
            else {
                unreachable!()
            };
            a_seconds.partial_cmp(b_seconds).unwrap()
        });
        Ok(())
    }
}
