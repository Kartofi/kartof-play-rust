#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";

use std::io::Read;
use std::process::Command;
use std::process::Stdio;

use std::env;
use std::path::Path;

// Setup node.js
pub fn start() {
    env::set_current_dir("./node_js").unwrap();

    println!("Checking for node.js installation");
    let version = check_node_install();
    if version.0 == false {
        panic!("No node.js install found! Please install node.js version >= 12.5.0 https://nodejs.org/")
    }
    println!("Node.js version {} found!", version.1.trim());

    //Install node.js modules
    println!("Updating/Installing node.js modules");
    update_node_modules();
}
fn update_node_modules() {
    let mut cmd = Command::new(NPM)
        .args(["i", "@consumet/extensions"])
        .stdin(Stdio::null()) // No input to provide
        .stdout(Stdio::inherit()) // Print output to the same place as the Rust process
        .stderr(Stdio::inherit()) // Print errors to the same place as the Rust process
        .spawn()
        .unwrap();

    // Wait for the command to finish and check the status
    let status = cmd.wait().unwrap();

    if status.success() {
        println!("Node modules updated successfully.");
    } else {
        eprintln!("Failed to update node modules. Exit status: {}", status);
    }
}
fn check_node_install() -> (bool, String) {
    let cmd = Command::new("node")
        .arg("-v")
        .output()
        .expect("Failed to execute command");
    (
        cmd.status.success(),
        String::from_utf8_lossy(&cmd.stdout).trim().to_string(),
    )
}
