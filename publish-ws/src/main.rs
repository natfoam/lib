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
struct CargoToml {
    workspace: Workspace
}

fn main() {
    let cargo_toml_str = read_to_string("Cargo.toml").unwrap();
    let cargo_toml: CargoToml = from_str(&cargo_toml_str).unwrap();
    for member in cargo_toml.workspace.members {
        println!("{}", member);
        let p = canonicalize(member).unwrap();
        println!("{:?}", p);
        let output = Command::new("cargo").arg("publish").current_dir(p).output().unwrap();
        stdout().write_all(&output.stdout).unwrap();
        stderr().write_all(&output.stderr).unwrap();
    }
}
