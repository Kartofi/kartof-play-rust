pub fn get(url: &str) -> Option<String> {
    match reqwest::blocking::get(url) {
        Ok(req) => match req.text() {
            Ok(text) => Some(text),
            Err(_err) => {
                println!("ERROR REQ URL: {}", url);
                None
            }
        },
        Err(_err) => {
            println!("ERROR REQ URL: {}", url);
            None
        }
    }
}

use reqwest::blocking::Client;
use reqwest::Proxy;
pub fn get_proxy(url: &str, proxy_url: &str) -> Option<String> {
    // Create a reqwest client with proxy settings
    let client = match Client::builder()
        .proxy(Proxy::all(proxy_url).unwrap()) // Set the proxy URL
        .build()
    {
        Ok(client) => client,
        Err(_err) => {
            println!("ERROR CREATING CLIENT WITH PROXY: {}", proxy_url);
            return None;
        }
    };

    // Make the request using the client with the proxy
    match client.get(url).send() {
        Ok(req) => match req.text() {
            Ok(text) => Some(text),
            Err(_err) => {
                println!("ERROR REQ URL: {}", url);
                None
            }
        },
        Err(_err) => {
            println!("ERROR REQ URL: {}", url);
            None
        }
    }
}
