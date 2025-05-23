use crate::*;
use event::*;
use std::error;
use vector2d::Vector2D;

impl Level {
    pub fn update(&mut self, seconds: f64) -> Result<(), Box<dyn error::Error>> {
        if !self.parsed {
            return Err(Box::new(LevelIsNotParsedError {
                calling_function: "update",
            }));
        }
        for tile in &mut self.tiles {
            tile.data.orig2now();
        }
        for i in 0..self.dynamic_events.len() {
            let EventData::Dynamic {
                event,
                beats,
                seconds: e_seconds,
            } = &self.dynamic_events[i]
            else {
                unreachable!()
            };
            event
                .clone()
                .apply(
                    (beats.unwrap(), e_seconds.unwrap()),
                    self,
                    seconds,
                )
                .unwrap();
        }
        Ok(())
    }
    fn calc_camera_player(
        &mut self,
        seconds: f64,
        floor: usize,
    ) -> Result<(), Box<dyn error::Error>> {
        // todo!();
        let delta = seconds - self.camera.last_seconds;
        if !delta.is_normal() {
            return Ok(());
        }
        // let floor = if self.tiles;
        // let mut a = if floor == self.tiles.len() - 1 {
        //     seconds - self.tiles[floor].data.seconds.unwrap()
        // } else {
        //     seconds.min(self.tiles[floor + 1].data.seconds.unwrap()) - self.tiles[floor].data.seconds.unwrap()
        // };
        // a = (a / 2.0).min(1.0);
        // self.camera.player_cam_pos += (target_pos - self.camera.player_cam_pos) * a;
        let target_pos = self.tiles[floor].data.position.orig.unwrap();
        if floor != self.camera.last_floor {
            self.camera.last_floor = floor;
            self.camera.last_change_pos = self.camera.player_cam_pos;
        }
        let gap_dis = (target_pos - self.camera.last_change_pos).length();
        let speed = gap_dis * self.get_bpm_by_seconds(seconds)? / 60.0 / 2.0;
        let v = target_pos - self.camera.player_cam_pos;
        let n = v.normalise() * delta * speed;
        if (self.camera.player_cam_pos - target_pos).length_squared() > n.length_squared() {
            self.camera.player_cam_pos += n;
        } else {
            self.camera.player_cam_pos = target_pos;
        }
        Ok(())
    }
    pub fn update_camera(
        &mut self,
        seconds: f64,
        floor: usize,
    ) -> Result<(), Box<dyn error::Error>> {
        if !self.parsed {
            return Err(Box::new(LevelIsNotParsedError {
                calling_function: "update_camera",
            }));
        }
        // todo!();
        use RelativeToCamera::*;
        let (mut pos, mut rot, mut zoom) = (
            Vector2D::new(0.0, 0.0),
            self.settings.rotation,
            self.settings.zoom,
        );
        let mut pos_off = self.settings.position;
        let mut last_rel_to = self.settings.relative_to;
        let mut rel_to_player = if matches!(last_rel_to, Player) {
            1.0
        } else {
            0.0
        };
        // let mut plp = 0;
        for p in self.dynamic_events.clone().iter().enumerate() {
            let (
                index,
                EventData::Dynamic {
                    event: dynamic_event,
                    seconds: Some(e_seconds),
                    ..
                },
            ) = &p
            else {
                continue;
            };
            let DynamicEvents::MoveCamera(move_camera) = dynamic_event else {
                continue;
            };

            // let bpm = self.get_bpm_by_seconds(e_seconds)?;
            let bpm = self.get_bpm_by_floor_seconds(dynamic_event.floor(), *e_seconds)?;
            let spb = bpm2crotchet(bpm);

            if seconds < *e_seconds {
                break;
            }
            let x = if move_camera.duration == 0.0 {
                1.0
            } else {
                (seconds - e_seconds) / spb / move_camera.duration
            };
            let y = move_camera.ease.calc(x);
            if let Some(rel_to) = move_camera.relative_to {
                match rel_to {
                    Tile | Global => {
                        let f = if matches!(rel_to, Tile) { dynamic_event.floor() } else { 0 };
                        let tile = &self.tiles[f];
                        rel_to_player += (0.0 - rel_to_player) * y;
                        if matches!(last_rel_to, Player) {
                            pos = tile.data.position.orig.unwrap();
                        } else {
                            pos += (tile.data.position.orig.unwrap() - pos) * y;
                        }
                    }
                    Player => {
                        if !matches!(last_rel_to, Player) && !matches!(last_rel_to, LastPosition) {
                            if self.camera.last_event_index < *index {
                                self.camera.player_cam_pos = pos + pos_off;
                                pos_off = Vector2D::new(0.0, 0.0);
                            }
                            last_rel_to = Player;
                        }
                        rel_to_player += (1.0 - rel_to_player) * y;
                    }
                    LastPosition => {
                        // if self.camera.v.len() <= plp {
                        //     self.camera.v.push(self.camera.player_cam_pos);
                        // }
                        // let mut new_pos = pos;
                        // // if matches!(last_rel_to, Player) {
                        // //     new_pos = new_pos * (1.0 - rel_to_player)
                        // //     + /*self.camera.player_cam_pos*/ self.camera.v[plp] * rel_to_player;
                        // // }
                        // pos = new_pos * (1.0 - y)
                        //     + (pos * (1.0 - rel_to_player) + self.camera.v[plp] * rel_to_player)
                        //         * y
                        //     + pos_off;
                        // pos_off = Vector2D::new(0.0, 0.0);
                        // rel_to_player = 0.0;
                        // plp += 1;
                        // if self.camera.v.len() <= lpi {
                        //     self.camera.v.push(self.camera.player_cam_pos);
                        // }
                        // let mut cam_pos = pos;
                        // if matches!(last_rel_to, Player) {
                        //     cam_pos = cam_pos * (1.0 - rel_to_player)
                        //     + /*self.camera.player_cam_pos*/ self.camera.v[lpi] * rel_to_player;
                        // }
                        // pos = cam_pos * (1.0 - y) + pos * y + pos_off;
                        // pos_off = Vector2D::new(0.0, 0.0);
                        // rel_to_player = 0.0;
                        // lpi += 1;
                    }
                }
            }
            if let Some(pos_x) = move_camera.position.x {
                pos_off.x += (pos_x - pos_off.x) * y;
            } else if let Some(rel_to) = move_camera.relative_to {
                if rel_to != last_rel_to {
                    pos_off.x += (0.0 - pos_off.x) * y;
                }
            }
            if let Some(pos_y) = move_camera.position.y {
                pos_off.y += (pos_y - pos_off.y) * y;
            } else if let Some(rel_to) = move_camera.relative_to {
                if rel_to != last_rel_to {
                    pos_off.y += (0.0 - pos_off.y) * y;
                }
            }
            if let Some(m_rot) = move_camera.rotation {
                rot += (m_rot - rot) * y;
            }
            if let Some(m_zoom) = move_camera.zoom {
                zoom += (m_zoom - zoom) * y;
            }

            if let Some(rel_to) = move_camera.relative_to {
                last_rel_to = rel_to;
            }

            self.camera.last_event_index = self.camera.last_event_index.max(*index);
        }
        self.calc_camera_player(seconds, floor).unwrap();
        self.camera.position =
            pos * (1.0 - rel_to_player) + self.camera.player_cam_pos * rel_to_player + pos_off;
        self.camera.rotation = rot;
        self.camera.zoom = zoom;

        self.camera.last_seconds = seconds;
        Ok(())
    }

    pub fn reset_camera(&mut self) {}
}
