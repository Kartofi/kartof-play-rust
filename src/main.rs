extern crate urlencoding;

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::{env, fs};

use dojang::Dojang;
use dotenv::dotenv;

use choki::structs::*;
use choki::*;
use mongodb::Database;
use reqwest;
use scrapers::{anime_schedule, gogoanime};
use serde_json::json;
use threadpool::ThreadPool;
use utils::http;
use utils::types::{Anime, IdType};

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

pub static CACHE_ALL_ANIME_FREQUENCY: Duration = Duration::from_secs(604800); // 7 Days

pub static HTTP_REQUEST_TIMEOUT: Duration = Duration::from_secs(4);
pub static HTTP_FREQUENCY_TIMEOUT: Duration = Duration::from_secs(2);
fn main() {
    dotenv().ok(); // Load ENV
    node_js::start(); // Setup node.js stuff

    let mut database = utils::mongodb::Database::new().unwrap();
    caching::start(database.clone());

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
            "/watch/[id]/[ep_num]".to_string(),
            |mut req: Request, mut res: Response, database: Option<utils::mongodb::Database>| {
                let mut dojang = Dojang::new();
                assert!(dojang.load("./ui").is_ok());

                let id = req.params.get("id").unwrap();
                let ep_num = req.params.get("ep_num").unwrap();

                let anime = database
                    .unwrap()
                    .get_anime_id(id, &IdType::KartofPlay, false)
                    .unwrap_or(Anime::new());

                let url = scrapers::gogoanime::anime_streaming_url::get(
                    &(anime.gogo_id.to_string() + "-episode-" + ep_num),
                )
                .unwrap_or_default();

                let data = dojang.render("player.html", json!({"url": url})).unwrap();
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
