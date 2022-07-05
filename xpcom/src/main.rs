mod guid;

use guid::Guid;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
struct Param {
    name: String,
    r#type: Type,
}

#[derive(Serialize, Deserialize, Clone)]
struct Method {
    name: String,
    params: Vec<Param>,
    result: Type,
}

type Struct = Vec<Param>;

#[derive(Serialize, Deserialize)]
enum TypeDef {
    Struct(Struct),
    Interface { guid: Guid, methods: Vec<Method> },
}

fn type_def(name: &str, def: TypeDef) -> (String, TypeDef) {
    (name.to_string(), def)
}

fn struct_(name: &str, params: &[Param]) -> (String, TypeDef) {
    type_def(name, TypeDef::Struct(params.to_vec()))
}

fn interface(name: &str, guid: u128, methods: &[Method]) -> (String, TypeDef) {
    type_def(
        name,
        TypeDef::Interface {
            guid: Guid(guid),
            methods: methods.to_vec(),
        },
    )
}

fn method(name: &str, params: &[Param], result: Type) -> Method {
    Method {
        name: name.to_string(),
        params: params.to_vec(),
        result,
    }
}

fn ptr(type_: Type) -> Type {
    Type::Ptr(Box::new(type_))
}

fn param(name: &str, type_: Type) -> Param {
    Param { name: name.to_string(), r#type: type_ }
}

type Library = HashMap<String, TypeDef>;

fn main() {
    let library = Library::from([
        struct_(
            "S",
            &[param("begin",Type::U8)],
        ),
        interface(
            "IMy",
            0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            &[method("GetValue", &[], ptr(Type::U32))],
        ),
    ]);

    // let x = serde_json::to_string(&library).unwrap();
    let x = serde_yaml::to_string(&library).unwrap();
    println!("{}", x);
}
