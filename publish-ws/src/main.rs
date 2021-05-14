use std::collections::HashMap;
use toml::Value;
use std::io::{stdout,stderr,Write};
use toml::from_str;
use std::fs::{read_to_string, canonicalize};
use serde_derive::Deserialize;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct Workspace {
    members: Vec<String>
}

#[derive(Deserialize, Debug)]
struct WsCargoToml {
    workspace: Workspace
}

#[derive(Deserialize, Debug)]
struct CargoToml {
    dependencies: HashMap<String, Value>,
}

fn main() {
    let ws_cargo_toml_str = read_to_string("Cargo.toml").unwrap();
    let ws_cargo_toml: WsCargoToml = from_str(&ws_cargo_toml_str).unwrap();
    for member in ws_cargo_toml.workspace.members {
        /*
        let p = canonicalize(member).unwrap();
        let cargo_toml_file = p.join("Cargo.toml");
        println!("{:?}", cargo_toml_file);
        let cargo_toml_str = read_to_string(cargo_toml_file).unwrap();
        let cargo_toml: CargoToml = from_str(&cargo_toml_str).unwrap();
        println!("{:?}", cargo_toml);
        */
        let p = String::from(".\\") + &member;
        let _ = Command::new("cargo").arg("publish").current_dir(p).spawn().unwrap().wait();
        // stdout().write_all(&output.stdout).unwrap();
        // stderr().write_all(&output.stderr).unwrap();
    }
}
