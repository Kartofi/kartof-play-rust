use std::{
    env,
    fs::{self},
    io,
    path::{Path, PathBuf},
};

fn main() {
    //Get the output folder
    let build_folder = "./target/".to_string() + &env::var("PROFILE").unwrap();
    let node_js_des = build_folder.clone() + "/node_js";
    //Check if node_js folder exists if not create one
    check_directory(&node_js_des);
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

    let ui_des = build_folder.clone() + "/ui";
    //Check if ui folder exists if not create one
    check_directory(&ui_des);
    //Copy whole ui folder
    copy_whole_dir("./ui", &ui_des);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=node_js");
    println!("cargo:rerun-if-changed=ui");
    println!("cargo:rerun-if-changed=Cargo.toml");
}
fn check_directory(path: &str) {
    if !Path::exists(&Path::new(path)) {
        fs::create_dir(path).unwrap();
    }
}
fn copy_whole_dir(from: &str, to: &str) {
    //Clean to folder
    let path = Path::new(to);

    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                fs::remove_dir_all(&path).unwrap();
            } else {
                fs::remove_file(&path).unwrap();
            }
        }
    }

    //Transfer files
    copy_folder_contents(&Path::new(from), &Path::new(to)).unwrap();
}
fn copy_folder_contents(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let mut dst_path = PathBuf::from(dst);
        dst_path.push(entry.file_name());

        if src_path.is_dir() {
            copy_folder_contents(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
