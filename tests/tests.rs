use rusty_adofai::*;
use std::fs;
use serde_json_lenient::Value;
use strip_bom::StripBom;
const FILE_PATH: &str = "F:\\Levels\\(No VFX) Hello (BPM) 2025\\Hello (BPM) 2025.adofai";

#[test]
fn convert_between_beat_and_ms() {
    let string_data = fs::read_to_string(FILE_PATH).unwrap();
    let json_data: Value = serde_json_lenient::from_str(string_data.strip_bom()).unwrap();
    let mut level = Level::from(&json_data);
    level.parse();
    let begin = -10000;
    let end = 10000;
    let epsilon = 0.00000000005;
    for i in begin..end {
        let beat = i as f64;
        let new_beat = level.ms2beat(level.beat2ms(beat).unwrap()).unwrap();
        let d1 = (beat - new_beat).abs();

        let ms = i as f64;
        let new_ms = level.beat2ms(level.ms2beat(ms).unwrap()).unwrap();
        let d2 = (ms - new_ms).abs();
        
        assert!(d1 <= epsilon, "d1 = {} > epsilon = {}", d1, epsilon);
        assert!(d2 <= epsilon, "d2 = {} > epsilon = {}", d2, epsilon);
    }
}