use std::{collections::HashMap, fmt::format};

use serde::{Serialize, Deserialize, de::Visitor};
use serde_derive::{Deserialize, Serialize};
use serde::de::Error;

#[derive(Serialize, Deserialize)]
enum Type {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    Ptr(Box<Type>),
    Id(String),
}

#[derive(Serialize, Deserialize)]
struct Param {
    name: String,
    r#type: Type,
}

#[derive(Serialize, Deserialize)]
struct Method {
    name: String,
    params: Vec<Param>,
    result: Type,
}

type Struct = Vec<Param>;

struct Guid (u128);

impl Serialize for Guid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
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
            E: Error, {
        Ok(v)
    }
}

impl<'de> Deserialize<'de> for Guid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let s = deserializer.deserialize_string(StringVisitor)?;
        let x = u128::from_str_radix(s.as_str(), 16);
        match x {
            Ok(v) => Ok(Guid(v)),
            Err(e) => Err(e).map_err(D::Error::custom)
        }
    }
}

#[derive(Serialize, Deserialize)]
enum TypeDef {
    Struct(Struct),
    Interface { guid: Guid, methods: Vec<Method> },
}

type Library = HashMap<String, TypeDef>;

fn main() {
    let library = Library::from([
        (
            "S".to_string(),
            TypeDef::Struct(vec![Param {
                name: "begin".to_string(),
                r#type: Type::U8,
            }]),
        ),
        (
            "IMy".to_string(),
            TypeDef::Interface {
                guid: Guid(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF),
                methods: vec![Method {
                    name: "GetValue".to_string(),
                    params: vec![],
                    result: Type::Ptr(Box::new(Type::U32)),
                }],
            },
        ),
    ]);

    // let x = serde_json::to_string(&library).unwrap();
    let x = serde_yaml::to_string(&library).unwrap();
    println!("{}", x);
}
