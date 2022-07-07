use serde::{
    de::{Error, Visitor},
    Deserialize, Serialize,
};

pub struct Guid(pub u128);

impl Serialize for Guid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(format!("{:X}", self.0).as_str())
    }
}

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string GUID")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }
}

impl<'de> Deserialize<'de> for Guid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = deserializer.deserialize_string(StringVisitor)?;
        let x = u128::from_str_radix(s.as_str(), 16);
        match x {
            Ok(v) => Ok(Guid(v)),
            Err(e) => Err(D::Error::custom(e)),
        }
    }
}
