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
    let mut map = HashMap::<String, Dependencies>::default();
    for member in ws_cargo_toml.workspace.members {
        // don't use `canonicalize`. `cargo` can't handle properly such paths.
        // For example, `cargo` can't `canonicalize` such path `c:\lib\sha-compress\../fixed-array`.
        let p = String::from(".\\") + &member;
        let cargo_toml_file = p + "\\Cargo.toml";
        println!("{:?}", cargo_toml_file);
        let cargo_toml_str = read_to_string(cargo_toml_file).unwrap();
        let cargo_toml: CargoToml = from_str(&cargo_toml_str).unwrap();
        println!("{:?}", cargo_toml.dependencies);
        map.insert(member, cargo_toml.dependencies);
    }
    let mut sorted = Vec::new();
    while !map.is_empty() {
        let member = map
            .iter()
            .find(|(_, dependencies)| {
                dependencies
                    .iter()
                    .all(|(dependency, _)| !map.contains_key(dependency))
            })
            .unwrap().0.clone();
        sorted.push(member.clone());
        map.remove(&member);
        println!("{}", member);
    }
    for member in sorted {
        let p = String::from(".\\") + &member;
        let _ = Command::new("cargo")
            .arg("publish")
            .current_dir(p)
            .spawn()
            .unwrap()
            .wait();
    }
}
