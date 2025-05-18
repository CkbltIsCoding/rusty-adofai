use crate::*;
use event::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for Level {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
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
            let event_tag = if object.contains_key("eventTag") {
                Some(object["eventTag"].as_str().unwrap())
            } else {
                None
            };
            let tags: Option<Vec<String>> = if let Some(event_tag) = event_tag {
                Some(event_tag.split_whitespace().map(|s| s.to_string()).collect())
            } else {
                None
            };
            match result {
                Ok(event) => {
                    tiles[floor].events.push(match event {
                        Events::Static(event) => EventData::Static { event, floor },
                        Events::Dynamic(event) => EventData::Dynamic {
                            event,
                            floor,
                            beats: None,
                            seconds: None,
                            tags,
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
