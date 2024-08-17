use crate::utils::types::*;
use std::env;
use std::io::Read;
use std::process::Command;
use std::process::Stdio;

pub fn get(ep_id: &str) -> Result<String, ScraperError> {
    let mut cmd = Command::new("node")
        .args(["./node_js/get_streaming.js", ep_id])
        .stdout(Stdio::piped()) // Redirect stdout to null
        .stderr(Stdio::null()) // Redirect stderr to null
        .spawn()
        .unwrap();

    let mut output = String::new();
    if let Some(mut stdout) = cmd.stdout.take() {
        stdout.read_to_string(&mut output).unwrap();
    }

    cmd.wait().unwrap(); // Waits for node.js process to finish

    //Clears the output to be only the url
    let mut url = output
        .replace("exit code: 0", "")
        .replace("exit code: 1", "error")
        .replace("\n", "");
    //Check if there is axios errors bc axios console.logs even if you dont want it to
    if url.len() > 200 {
        url = "error".to_string();
    }
    if url == "error" {
        return Err(ScraperError {
            reason: "Id is invalid".to_string(),
        });
    }
    return Ok(url);
}
