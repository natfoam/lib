use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::process::Command;
use toml::from_str;
use toml::Value;

#[derive(Deserialize, Debug)]
struct Workspace {
    members: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct WsCargoToml {
    workspace: Workspace,
}

type Dependencies = HashMap<String, Value>;

#[derive(Deserialize, Debug)]
struct CargoToml {
    dependencies: Dependencies,
}

fn main() {
    let ws_cargo_toml_str = read_to_string("Cargo.toml").unwrap();
    let ws_cargo_toml: WsCargoToml = from_str(&ws_cargo_toml_str).unwrap();
    let mut map: HashMap<String, Dependencies> = ws_cargo_toml
        .workspace
        .members
        .iter()
        .map(|member| {
            let cargo_toml_file = String::from(".\\") + &member + "\\Cargo.toml";
            let cargo_toml_str = read_to_string(cargo_toml_file).unwrap();
            let cargo_toml: CargoToml = from_str(&cargo_toml_str).unwrap();
            (member.clone(), cargo_toml.dependencies)
        })
        .collect();
    // Publish dependent packages first.
    while !map.is_empty() {
        let member = map
            .iter()
            .find(|(_, dependencies)| {
                dependencies
                    .iter()
                    .all(|(dependency, _)| !map.contains_key(dependency))
            })
            .unwrap()
            .0
            .clone();
        println!("{}", member);
        println!();
        {
            // TODO:
            // 1. check if a package is available after uploading using `cargo search`.
            // 2. check if a package version is already uploaded.
            //
            // Note:
            // don't use `canonicalize`. `canonicalize` returns a UNC path.
            // https://github.com/rust-lang/rust/issues/42869
            // `cargo` can't handle properly UNC paths.
            let p = String::from(".\\") + &member;
            let _ = Command::new("cargo")
                .arg("publish")
                .current_dir(p)
                .spawn()
                .unwrap()
                .wait();
        }
        println!();
        map.remove(&member);
    }
}
