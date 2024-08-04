extern crate urlencoding;

use std::time::Instant;

use dotenv::dotenv;

use choki::structs::*;
use choki::*;
use mongodb::Database;
use reqwest;
use scrapers::{anime_schedule, gogoanime};
use threadpool::ThreadPool;
use utils::http;
use utils::types::IdType;

mod caching;
mod scrapers;
mod utils;

mod node_js;
//Anime Schedule
pub static ANIMESCHEDULE: &str = "https://animeschedule.net/";
//AnimeGG
pub static ANIMEGG: &str = "https://www.animegg.org/";
//Mal
pub static MALURL: &str = "https://myanimelist.net/";
//Gogoanime
pub static GOGOANIMEURL: &str = "https://gogoanime3.co/";
pub static GOGOANIMEURL_AJAX: &str = "https://ajax.gogocdn.net/ajax/";

pub static CACHE_COUNTDOWN: i64 = 300; // 5 MINS
fn main() {
    dotenv().ok(); // Load ENV
    node_js::start(); // Setup node.js stuff

    let mut database = utils::mongodb::Database::new().unwrap();

    let po = ThreadPool::new(100);

    for i in 0..100 {
        let page1 = scrapers::gogoanime::anime_list::get(&i.to_string()).unwrap_or_default();
        println!("Started page {}", i);
        for anime in page1 {
            let clone = database.clone();

            po.execute(move || {
                println!("Started - {}", anime);
                clone.cache_anime(&anime, IdType::Gogoanime).unwrap();
                println!("Done - {}", anime);
            });
        }
        po.join();
    }

    let mut server = Server::new(Some(1024), Some(database));

    server
        .get(
            "/".to_string(),
            |mut req: Request, mut res: Response, database: Option<utils::mongodb::Database>| {
                let query = req.query.get("query").unwrap();
                let result = database.unwrap().search_anime(query, 10).unwrap();

                let mut str_o = "".to_string();
                for res in result {
                    str_o.push_str(&res.title);
                    str_o.push_str("\n");
                }
                res.send_string(&str_o);
            },
        )
        .unwrap();
    server.listen(3000, None).unwrap();
    Server::<u8>::lock();
}
