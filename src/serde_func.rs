use serde::{Deserialize, Deserializer, Serializer};
use serde_json_lenient::Value;
use vector2d::Vector2D;
use rgb::Rgba;

pub(crate) fn ser_rgba_u8<S>(rgba: &Rgba<u8>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let string = if rgba.a == 255 {
        format!("{:X}{:X}{:X}", rgba.r, rgba.g, rgba.b)
    } else {
        format!("{:X}{:X}{:X}{:X}", rgba.r, rgba.g, rgba.b, rgba.a)
    };
    s.serialize_str(&string)
}
pub(crate) fn de_rgba_u8<'de, D>(de: D) -> Result<Rgba<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let res: serde_json_lenient::Value = Deserialize::deserialize(de)?;
    let src = res.as_str().ok_or(serde::de::Error::custom(""))?;
    let color = u32::from_str_radix(src, 16)
        .ok()
        .ok_or(serde::de::Error::custom(""))?;
    let color = if src.len() == 6 {
        (color * 0x100) + 0xff
    } else {
        color
    };
    Ok(Rgba {
        r: (color / (0x100 * 0x100 * 0x100)) as u8,
        g: (color / (0x100 * 0x100) % 0x100) as u8,
        b: (color / 0x100 % 0x100) as u8,
        a: (color % 0x100) as u8,
    })
}
pub(crate) fn de_bool<'de, D>(de: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let result: serde_json_lenient::Value = Deserialize::deserialize(de)?;
    match result {
        Value::Bool(b) => Ok(b),
        Value::String(ref s) if s == "Enabled" => Ok(true),
        Value::String(ref s) if s == "Disabled" => Ok(false),
        _ => Err(serde::de::Error::custom("Unexpected value")),
    }
}
pub(crate) fn ser_vector2d_f64<S>(v: &Vector2D<f64>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.collect_seq([v.x, v.y])
}
pub(crate) fn de_vector2d_f64<'de, D>(de: D) -> Result<Vector2D<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let result: serde_json_lenient::Value = Deserialize::deserialize(de)?;
    let array = result.as_array().ok_or(serde::de::Error::custom(""))?;
    let x = array[0].as_f64().ok_or(serde::de::Error::custom(""))?;
    let y = array[1].as_f64().ok_or(serde::de::Error::custom(""))?;
    Ok(Vector2D { x, y })
}
pub(crate) fn ser_vector2d_option_f64<S>(v: &Vector2D<Option<f64>>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.collect_seq([v.x, v.y])
}
pub(crate) fn de_vector2d_option_f64<'de, D>(de: D) -> Result<Vector2D<Option<f64>>, D::Error>
where
    D: Deserializer<'de>,
{
    let result: serde_json_lenient::Value = Deserialize::deserialize(de)?;
    let array = result.as_array().ok_or(serde::de::Error::custom(""))?;
    let x = match &array[0] {
        Value::Number(number) => Ok(Some(number.as_f64().unwrap())),
        Value::Null => Ok(None),
        _ => Err(serde::de::Error::custom("")),
    }?;
    let y = match &array[1] {
        Value::Number(number) => Ok(Some(number.as_f64().unwrap())),
        Value::Null => Ok(None),
        _ => Err(serde::de::Error::custom("")),
    }?;
    Ok(Vector2D { x, y })
}
pub(crate) fn ser_event_tag<S>(v: &Vec<String>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&v.join(" "))
}
pub(crate) fn de_event_tag<'de, D>(de: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let result: serde_json_lenient::Value = Deserialize::deserialize(de)?;
    let s = result.as_str().ok_or(serde::de::Error::custom(""))?;
    Ok(s.split_whitespace().map(|s| s.to_string()).collect())
}
