use std::str::FromStr;

use choki::structs::*;
use mongodb::bson;
use serde_json::{json, Value};

use crate::{
    scrapers,
    utils::{
        self,
        types::{Anime, IdType},
    },
    HANDLEBARS,
};

use super::RouteData;

pub static PATH: &str = "/";

pub fn get(mut req: Request, mut res: Response, database: Option<utils::mongodb::Database>) {
    let db = database.unwrap();

    let mut home = db.get_home(&utils::get_date_string()).unwrap_or_default();

    let mut days_ago: i64 = 1;
    while home.is_none() {
        home = db
            .get_home(&&utils::get_previous_date_string(&days_ago))
            .unwrap_or_default();
        if days_ago >= 30 {
            break;
        }
        days_ago += 1;
    }

    if home.is_none() {
        res.send_code(404);
    }
    let home = home.unwrap();

    let json = serde_json::to_value(home).unwrap_or_default();

    let data = HANDLEBARS.render("home", &json).unwrap();

    res.set_header(&Header::new(
        "Access-Control-Allow-Origin".to_string(),
        "*".to_string(),
    ));
    res.send_bytes(data.as_bytes(), Some(ContentType::Html));
}

pub fn get_route() -> RouteData {
    return (PATH.to_string(), get);
}
