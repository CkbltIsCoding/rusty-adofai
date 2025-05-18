use rusty_adofai::*;
use std::fs;
use strip_bom::StripBom;
const FILE_PATH: &str = "F:\\Levels\\(No VFX) Hello (BPM) 2025\\Hello (BPM) 2025.adofai";
// const FILE_PATH: &str = "F:\\Levels\\FToTJ\\First Town Of This Journey.adofai";
// const FILE_PATH: &str = "F:\\Levels\\Hello (BPM) 2025_fix\\Hello (BPM) 2025.adofai";

fn get_level() -> Level {
    // let string_data = fs::read_to_string(FILE_PATH).unwrap().strip_bom();
    // let json_data: Value = serde_json_lenient::from_str(string_data).unwrap();
    // Level::from(json_data)
    let string_bom = fs::read_to_string(FILE_PATH).unwrap();
    let string_data = string_bom.strip_bom();
    serde_json_lenient::from_str(string_data).unwrap()
}

#[test]
fn convertion_between_beats_and_seconds() {
    let mut level = get_level();
    level.parse().unwrap();
    // let begin = -10000;
    let begin = -100;
    // let end = 10000;
    let end = 100;
    let epsilon = 0.00000000005;
    for i in begin..end {
        let beats = i as f64;
        let new_beats = level.seconds2beats(level.beats2seconds(beats).unwrap()).unwrap();
        let d1 = (beats - new_beats).abs();

        let ms = i as f64;
        let new_ms = level.beats2seconds(level.seconds2beats(ms).unwrap()).unwrap();
        let d2 = (ms - new_ms).abs();
        
        assert!(d1 <= epsilon, "d1 = {} > epsilon = {}", d1, epsilon);
        assert!(d2 <= epsilon, "d2 = {} > epsilon = {}", d2, epsilon);
    }
}

#[test]
fn t() {
    const FILE_PATH: &str = "F:\\Levels\\(No VFX) Hello (BPM) 2025\\Hello (BPM) 2025.adofai";
    let mut level = rusty_adofai::Level::open(FILE_PATH).unwrap();
    level.parse().unwrap();
    for tile in &level.tiles {
        println!("{}", tile.data.seconds().unwrap());
    }
}