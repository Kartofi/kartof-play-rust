use std::time::Instant;

use dotenv::dotenv;

use choki::structs::*;
use choki::*;
use mongodb::Database;
use reqwest;
use threadpool::ThreadPool;
use utils::http;

mod caching;
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

pub static CACHE_ANIME_COUNTDOWN: i64 = 300; // 5 MINS
fn main() {
    dotenv().ok(); // Load ENV
    let start = Instant::now();

    let mut database = utils::mongodb::Database::new().unwrap();

    let po = ThreadPool::new(3);
    let clone = database.clone();
    let clone2 = database.clone();
    po.execute(move || {
        clone.cache_anime("isekai-suicide-squad").unwrap();
    });

    po.join();
    println!("{}", start.elapsed().as_secs());
    let mut server = Server::new(Some(1024), Some(database));

    server
        .get(
            "/".to_string(),
            |mut req: Request, mut res: Response, database: Option<utils::mongodb::Database>| {
                let body = reqwest::blocking::get("https://www.rust-lang.org")
                    .unwrap()
                    .text()
                    .unwrap();

                println!(
                    "{:?}",
                    database.unwrap().search_anime("naruto", 10).unwrap().len()
                );
                res.send_bytes(&body.as_bytes(), None);
            },
        )
        .unwrap();
    server.listen(3000, None).unwrap();
    Server::<u8>::lock();
}
