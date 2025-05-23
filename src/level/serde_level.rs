use crate::*;
use event::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for Level {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
        // let mut map = serializer.serialize_map(Some(3))?;
        // let mut angle_data = vec![];
        // let mut actions = vec![];
        // for tile in &self.tiles {
        //     angle_data.push(tile.angle);
        //     for event in &tile.events {
        //         actions.push(event.clone());
        //     }
        // }
        // map.serialize_entry("angleData", &angle_data);
        // map.serialize_entry("settings", &self.settings);
        // map.serialize_entry("actions", &actions);
        // map.end()
    }
}
impl<'de> Deserialize<'de> for Level {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error as e;
        let json: serde_json_lenient::Value = serde::Deserialize::deserialize(deserializer)?;
        let object = json
            .as_object()
            .ok_or(e::custom("The value is not an object"))?;
        let settings_result = serde_json_lenient::from_value(object["settings"].clone());
        if let Err(err) = settings_result {
            return Err(e::custom(&err.to_string()));
        }
        let settings: Settings = settings_result.unwrap();
        let mut tiles: Vec<Tile> = vec![];
        tiles.push(Tile::new(0.0));
        if object.contains_key("angleData") {
            for angle in object["angleData"].as_array().ok_or(e::custom(""))? {
                tiles.push(Tile::new(angle.as_f64().ok_or(e::custom(""))?));
            }
        } else if object.contains_key("pathData") {
            for path in object["pathData"].as_str().ok_or(e::custom(""))?.chars() {
                match path2angle(path) {
                    Ok(angle) => {
                        tiles.push(Tile::new(angle));
                    }
                    Err(_) => {
                        return Err(e::custom("msg"));
                    }
                }
            }
        } else {
            return Err(serde::de::Error::custom("msg"));
        }
        for data in object["actions"].as_array().ok_or(e::custom(""))? {
            let object = data.as_object().ok_or(e::custom(""))?;
            let floor = object["floor"].as_u64().ok_or(e::custom(""))? as usize;
            let result: Result<Events, _> = serde_json_lenient::from_value(data.clone());
            match result {
                Ok(event) => {
                    tiles[floor].events.push(match event {
                        Events::Static(event) => EventData::Static { event },
                        Events::Dynamic(event) => EventData::Dynamic {
                            event,
                            beats: None,
                            seconds: None,
                        },
                    });
                }
                Err(_err) => {
                    // if !err.to_string().starts_with("unknown variant") {
                    //     println!("{}", err);
                    // }
                }
            }
        }
        Ok(Level::new(tiles, settings))
    }
}
