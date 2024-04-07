use std::fs;
use std::process::Command;

fn main() {
    let path = fs::canonicalize("./components").unwrap();
    Command::new("npm").args(&["run", "build"]).current_dir(path).status().unwrap();
    let srcPath = fs::canonicalize("./components/src/").unwrap();
    println!("cargo::rerun-if-changed={}", srcPath.display());
}
