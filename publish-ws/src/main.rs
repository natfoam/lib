use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_derive::Deserialize;
use std::{collections::HashMap, fmt, fmt::Formatter, fs::read_to_string, process::Command};
use toml::from_str;

const SLASH: &str = if cfg!(windows) { "\\" } else { "/" };

#[derive(Deserialize, Debug)]
struct Workspace {
    members: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct WsCargoToml {
    workspace: Workspace,
}

#[derive(Debug)]
struct Dependency(String);

impl<'de> Deserialize<'de> for Dependency {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DependencyVisitor;

        impl<'de> Visitor<'de> for DependencyVisitor {
            type Value = Dependency;
            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("String or Table")
            }
            fn visit_str<E>(self, value: &str) -> Result<Dependency, E> {
                Ok(Dependency(value.to_string()))
            }
            fn visit_map<V>(self, mut visitor: V) -> Result<Dependency, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut result = None;
                while let Some((key, value)) = visitor.next_entry::<String, String>()? {
                    if key == "version" {
                        result = Some(value);
                    }
                }
                Ok(Dependency(result.unwrap()))
            }
        }

        d.deserialize_any(DependencyVisitor)
    }
}

type Dependencies = HashMap<String, Dependency>;

#[derive(Deserialize, Debug)]
struct Package {
    version: String,
}

#[derive(Deserialize, Debug)]
struct CargoToml {
    package: Package,
    dependencies: Dependencies,
}

fn main() {
    let ws_cargo_toml_str = read_to_string("Cargo.toml").unwrap();
    let ws_cargo_toml: WsCargoToml = from_str(&ws_cargo_toml_str).unwrap();
    let mut map: HashMap<String, CargoToml> = ws_cargo_toml
        .workspace
        .members
        .iter()
        .map(|member| {
            let cargo_toml_file = String::from(".") + SLASH + &member + SLASH + "Cargo.toml";
            let cargo_toml_str = read_to_string(cargo_toml_file).unwrap();
            let cargo_toml: CargoToml = from_str(&cargo_toml_str).unwrap();
            (member.clone(), cargo_toml)
        })
        .collect();
    // check versions of internal dependencies
    for (name, cargo_toml) in map.iter() {
        for (d_name, d_version) in cargo_toml.dependencies.iter() {
            if let Some(p) = map.get(d_name) {
                if p.package.version != d_version.0 {
                    eprintln!(
                        "Version mismatch: {}.{} = {}, {} = {}",
                        name, d_name, d_version.0, d_name, p.package.version
                    );
                    return;
                }
            }
        }
    }
    // Publish dependent packages first.
    while !map.is_empty() {
        let member = map
            .iter()
            .find(|(_, cargo_toml)| {
                cargo_toml
                    .dependencies
                    .iter()
                    .all(|(dependency, _)| !map.contains_key(dependency))
            })
            .unwrap();
        println!("{} = {}", member.0, member.1.package.version);
        let key = member.0.clone();
        println!();
        let crate_version = {
            let x = Command::new("cargo")
                .arg("search")
                .arg(key.clone())
                .arg("--limit")
                .arg("1")
                .output()
                .unwrap();
            let line = String::from_utf8(x.stdout)
                .unwrap()
                .lines()
                .next()
                .unwrap()
                .to_string();
            let x: HashMap<String, String> = from_str(&line).unwrap();
            let i = x.iter().next().unwrap();
            println!("crates.io: {} = {}", i.0, i.1);
            i.1.clone()
        };
        if crate_version != member.1.package.version {
            // TODO:
            // 1. check if a package is available after uploading using `cargo search`.
            // 2. check if package dependencies are updated then the package should have a new version.
            // 3. check if the package is changed (using Git and a commit id from crates.io) (we need to include a commit id into package metadata)
            // 4. check if all changes are committed. It looks like Cargo checks only package changes.
            //
            // Note:
            // don't use `canonicalize`. `canonicalize` returns a UNC path.
            // https://github.com/rust-lang/rust/issues/42869
            // `cargo` can't handle properly UNC paths.
            let p = String::from(".") + SLASH + &key;
            let _ = Command::new("cargo")
                .arg("publish")
                .current_dir(p)
                .spawn()
                .unwrap()
                .wait();
        }
        println!();
        map.remove(&key);
    }
}
