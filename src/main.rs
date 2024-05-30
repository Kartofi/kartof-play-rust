use std::time::Instant;

use dotenv::dotenv;

use choki::structs::*;
use choki::*;
use reqwest;

mod scrapers;
mod utils;

pub static GOGOANIMEURL: &str = "https://gogoanime3.co/";
pub static GOGOANIMEURL_AJAX: &str =
    "https://ajax.gogocdn.net/ajax/load-list-episode?ep_start=0&ep_end=999999&id=";

fn main() {
    dotenv().ok(); // Load ENV
    let start = Instant::now();
    println!(
        "{} elapsed: {}",
        scrapers::gogoanime::anime_stream::get("urusei-yatsura-2022-2nd-season-episode-20")
            .unwrap(),
        start.elapsed().as_millis()
    );
    utils::mongodb::connect().unwrap();

    let mut server = Server::new(Some(1024));
    server
        .get("/".to_string(), |mut req: Request, mut res: Response| {
            let body = reqwest::blocking::get("https://www.rust-lang.org")
                .unwrap()
                .text()
                .unwrap();

            res.send_bytes(&body.as_bytes(), None);
        })
        .unwrap();
    server.listen(3000, None).unwrap();
    Server::lock();
}
