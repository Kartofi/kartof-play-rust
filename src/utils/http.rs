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
