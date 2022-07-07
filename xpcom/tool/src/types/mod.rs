use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::guid::Guid;

#[derive(Serialize, Deserialize, Clone)]
pub enum Type {
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Param {
    name: String,
    r#type: Type,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Method {
    name: String,
    params: Vec<Param>,
    result: Type,
}

type Struct = Vec<Param>;

#[derive(Serialize, Deserialize)]
pub enum TypeDef {
    Struct(Struct),
    Interface { guid: Guid, methods: Vec<Method> },
}

fn type_def(name: &str, def: TypeDef) -> (String, TypeDef) {
    (name.to_string(), def)
}

pub fn struct_(name: &str, params: &[Param]) -> (String, TypeDef) {
    type_def(name, TypeDef::Struct(params.to_vec()))
}

pub fn interface(name: &str, guid: u128, methods: &[Method]) -> (String, TypeDef) {
    type_def(
        name,
        TypeDef::Interface {
            guid: Guid(guid),
            methods: methods.to_vec(),
        },
    )
}

pub fn method(name: &str, params: &[Param], result: Type) -> Method {
    Method {
        name: name.to_string(),
        params: params.to_vec(),
        result,
    }
}

pub fn ptr(type_: Type) -> Type {
    Type::Ptr(Box::new(type_))
}

pub fn param(name: &str, type_: Type) -> Param {
    Param {
        name: name.to_string(),
        r#type: type_,
    }
}

pub type Library = HashMap<String, TypeDef>;
