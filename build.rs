use std::{
    env,
    fs::{self},
    path::Path,
};

fn main() {
    //Get the output folder
    let build_folder = "./target/".to_string() + &env::var("PROFILE").unwrap();
    let node_js_des = build_folder.clone() + "/node_js";
    //Check if node_js folder exists if not create one
    if !Path::exists(&Path::new(&node_js_des)) {
        fs::create_dir(&node_js_des).unwrap();
    }
    //Copy the get_streaming.js
    fs::copy(
        "./node_js/get_streaming.js",
        format!("{}/get_streaming.js", &node_js_des),
    )
    .unwrap();
    //Copy the package.json
    fs::copy(
        "./node_js/package.json",
        format!("{}/package.json", &node_js_des),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=node_js");
    println!("cargo:rerun-if-changed=Cargo.toml");
}
