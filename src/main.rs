use std::time::Instant;

use dotenv::dotenv;

use choki::structs::*;
use choki::*;
use reqwest;

mod scrapers;
mod utils;

pub static GOGOANIMEURLL: &str = "https://gogoanime3.co/";

fn main() {
    dotenv().ok(); // Load ENV
    let start = Instant::now();
    println!(
        "{} elapsed: {}",
        scrapers::gogoanime::anime_details::get("naruto")
            .unwrap()
            .description,
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
