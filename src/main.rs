extern crate urlencoding;

use std::time::{Duration, Instant};
use std::{env, fs};

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

pub static CACHE_HOME_FREQUENCY_NUM: i64 = 300; // 5 MINS
pub static CACHE_HOME_FREQUENCY: Duration = Duration::from_secs(CACHE_HOME_FREQUENCY_NUM as u64); // 5 MINS

fn main() {
    dotenv().ok(); // Load ENV
    node_js::start(); // Setup node.js stuff

    let mut database = utils::mongodb::Database::new().unwrap();
    caching::start(database.clone());

    //database.cache_home().unwrap();

    let po = ThreadPool::new(100);
    let mut second: bool = false;
    for i in 0..0 {
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
        let mut waited = false;
        if second == true {
            second = false;
            waited = true;
            po.join();
        }
        if waited == false {
            second = true;
        }
    }

    let mut server = Server::new(Some(1024), Some(database));

    server
        .get(
            "/search/[query]/[page]".to_string(),
            |mut req: Request, mut res: Response, database: Option<utils::mongodb::Database>| {
                let query = req.params.get("query").unwrap();
                let result = database.unwrap().search_anime(query, 2, 2).unwrap();

                let mut str_o = "".to_string();
                for res in result {
                    str_o.push_str("<img src='");
                    str_o.push_str(&res.details.cover_url);
                    str_o.push_str("'>");
                    str_o.push_str("<br>");
                    str_o.push_str(&res.title);
                    str_o.push_str("<br>");
                }
                res.send_bytes(&str_o.as_bytes(), Some(ContentType::Html));
            },
        )
        .unwrap();
    server
        .get(
            "/[id]".to_string(),
            |mut req: Request, mut res: Response, database: Option<utils::mongodb::Database>| {
                let data = fs::read_to_string("./ui/index.html").unwrap();
                res.set_header(&Header::new(
                    "Access-Control-Allow-Origin".to_string(),
                    "*".to_string(),
                ));
                res.send_bytes(data.as_bytes(), Some(ContentType::Html));
            },
        )
        .unwrap();
    server.listen(3000, None).unwrap();
    Server::<u8>::lock();
}
