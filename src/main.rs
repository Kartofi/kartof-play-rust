use std::time::Instant;

use dotenv::dotenv;

use choki::structs::*;
use choki::*;
use reqwest;
use utils::http;

mod scrapers;
mod utils;
//Anime Schedule
pub static ANIMESCHEDULE: &str = "https://animeschedule.net/";
//AnimeGG
pub static ANIMEGG: &str = "https://www.animegg.org/";
//Mal
pub static MALURL: &str = "https://myanimelist.net/";
//Gogoanime
pub static GOGOANIMEURL: &str = "https://gogoanime3.co/";
pub static GOGOANIMEURL_AJAX: &str = "https://ajax.gogocdn.net/ajax/";

fn main() {
    dotenv().ok(); // Load ENV
    let start = Instant::now();

    println!("{:?}", scrapers::gogoanime::anime_list::get("1"));
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
