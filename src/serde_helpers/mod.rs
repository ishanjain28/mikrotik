use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};

pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(Error::unknown_variant(s, &["true", "false"])),
    }
}

pub fn deserialize_u16<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    match s.parse::<u16>() {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::invalid_value(Unexpected::Str(s), &"an integer")),
    }
}

pub fn deserialize_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    match s.parse::<u64>() {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::invalid_value(Unexpected::Str(s), &"an integer")),
    }
}

pub fn maybe_deserialize_u16<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    match s.parse::<u16>() {
        Ok(v) => Ok(Some(v)),
        Err(_) => Err(Error::invalid_value(Unexpected::Str(s), &"an integer ")),
    }
}

pub fn maybe_deserialize_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    match s.parse::<u64>() {
        Ok(v) => Ok(Some(v)),
        Err(_) => Err(Error::invalid_value(Unexpected::Str(s), &"an integer")),
    }
}
