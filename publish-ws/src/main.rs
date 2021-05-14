use toml::from_str;
use std::fs::read_to_string;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct Workspace {
    members: Vec<String>
}

#[derive(Deserialize, Debug)]
struct CargoToml {
    workspace: Workspace
}

fn main() {
    let cargo_toml_str = read_to_string("Cargo.toml").unwrap();
    let cargo_toml: CargoToml = from_str(&cargo_toml_str).unwrap();
    println!("{:?}", cargo_toml);
}
