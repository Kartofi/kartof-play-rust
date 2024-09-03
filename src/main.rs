extern crate urlencoding;

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::{env, fs, thread};

use dojang::Dojang;
use dotenv::dotenv;

use choki::structs::*;
use choki::*;
use mongodb::Database;
use reqwest;
use routes::get_routes;
use scrapers::{anime_schedule, gogoanime};
use serde_json::json;
use threadpool::ThreadPool;
use utils::types::{Anime, IdType};
use utils::{http, images};

mod caching;
mod routes;
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

pub static UPDATE_ALL_ANIME_THREADS: usize = 100;



pub static CACHE_COUNTDOWN: i64 = 300; // 5 MINS

pub static CACHE_HOME_FREQUENCY_NUM: i64 = 300; // 5 MINS
pub static CACHE_HOME_FREQUENCY: Duration = Duration::from_secs(CACHE_HOME_FREQUENCY_NUM as u64); // 5 MINS

pub static CACHE_ALL_ANIME_FREQUENCY: Duration = Duration::from_secs(604800); // 7 Days

pub static HTTP_REQUEST_TIMEOUT: Duration = Duration::from_secs(4);
pub static HTTP_FREQUENCY_TIMEOUT: Duration = Duration::from_secs(2);

pub static IMAGES_PATH: &str = "./images";
pub static CACHE_SLEEP: Duration = Duration::from_millis(100);
pub static REPORT_COUNT: usize = 100;

fn main() {
    dotenv().ok(); // Load ENV
    node_js::start(); // Setup node.js stuff
    images::setup(); // Setup things for image host

    let database = utils::mongodb::Database::new().unwrap();
   

    //caching::start(database.clone());

    let mut server = Server::new(Some(1024), Some(database));
    let routes = get_routes();
    for route in routes {
        server.get(route.0, route.1).unwrap();
    }
   

    server.listen(3000, None).unwrap();
    Server::<u8>::lock();
}
