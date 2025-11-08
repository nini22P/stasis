use serde::Deserialize;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/webview.c");

    compile_webview();
}

fn compile_webview() {
    let wv_sys_path = find_wv_sys_path();
    let mut build = cc::Build::new();
    build.file("src/webview.c");
    build.include(&wv_sys_path.join("libs\\include"));
    build.include(&wv_sys_path.join("webview\\core\\include"));
    build.compile("windows");
}

#[derive(Debug, Deserialize)]
struct Metadata {
    packages: Vec<Package>,
}
#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    manifest_path: PathBuf,
}

fn find_wv_sys_path() -> PathBuf {
    let output = Command::new("cargo")
        .arg("metadata")
        .output()
        .expect("Failed to run cargo metadata");

    let metadata: Metadata =
        serde_json::from_slice(&output.stdout).expect("Failed to parse cargo metadata");

    let wv_sys_pkg = metadata
        .packages
        .into_iter()
        .find(|pkg| pkg.name == "wv-sys")
        .expect("Could not find 'wv-sys' package in metadata");

    let path = wv_sys_pkg
        .manifest_path
        .parent()
        .expect("manifest_path has no parent");

    if !path.exists() {
        panic!(
            "Failed to find headers at: {}. wv-sys internal layout may have changed.",
            path.display()
        );
    }

    path.to_path_buf()
}
