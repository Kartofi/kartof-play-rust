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
use utils::caching_info::{self, CachingInfo};
use utils::settings::Settings;
use utils::types::{Anime, IdType};
use utils::{http, images};

mod caching;
mod routes;
mod scrapers;
mod utils;

mod node_js;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::init();
}
fn main() {
    dotenv().ok(); // Load ENV
    node_js::start(); // Setup node.js stuff
    images::setup(); // Setup things for image host

    caching_info::CachingInfo::init();

    let database = utils::mongodb::Database::new().unwrap();

    caching::start(database.clone());

    let mut server = Server::new(Some(1024), Some(database));
    let routes = get_routes();
    for route in routes {
        server.get(route.0, route.1).unwrap();
    }

    server.listen(3000, None).unwrap();
    Server::<u8>::lock();
}
