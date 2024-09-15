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

pub static PATH: &str = "/search";

pub fn get(mut req: Request, mut res: Response, database: Option<utils::mongodb::Database>) {
    let db = database.unwrap();
    println!("{:?}", req.query);
    let mut ress = db
        .search_anime(req.query.get("q").unwrap(), 10, 1)
        .unwrap_or_default();

    let json = json!({ "data": ress});

    let data = HANDLEBARS.render("search", &json).unwrap();

    res.set_header(&Header::new(
        "Access-Control-Allow-Origin".to_string(),
        "*".to_string(),
    ));
    res.send_bytes(data.as_bytes(), Some(ContentType::Html));
}

pub fn get_route() -> RouteData {
    return (PATH.to_string(), get);
}
