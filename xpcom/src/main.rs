use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
enum TypeDef {
    Struct(Struct),
    Interface { guid: String, methods: Vec<Method> },
}

type Library = HashMap<String, TypeDef>;

fn main() {
    let library = Library::from([
        ("S".to_string(), TypeDef::Struct(vec![
            Param { name: "begin".to_string(), r#type: Type::U8 },
        ])),
        (
            "IMy".to_string(),
            TypeDef::Interface {
                guid: "".to_string(),
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
