extern crate urlencoding;

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::{env, fs, thread};

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

use handlebars::Handlebars;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::init();
    pub static ref HANDLEBARS: Handlebars<'static> = handlebars();
}

fn handlebars() -> Handlebars<'static> {
    let mut reg = Handlebars::new();
    // enable dev mode for template reloading
    reg.set_dev_mode(true);
    // register a template from the file
    // modified the file after the server starts to see things changing
    reg.register_template_file("home", "./ui/index.hbs")
        .unwrap();
    reg.register_template_file("search", "./ui/search.hbs")
        .unwrap();
    reg.register_template_file("player", "./ui/player.hbs")
        .unwrap();
    reg.register_template_file("watch", "./ui/watch.hbs")
        .unwrap();

    reg
}
fn main() {
    println!(
        "{:?}",
        scrapers::anime_schedule::anime_schedule::get().unwrap()[0]
    );
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
    server
        .new_static("/static".to_string(), "./ui/static".to_string())
        .unwrap();
    server.listen(3000, None).unwrap();
    Server::<u8>::lock();
}
