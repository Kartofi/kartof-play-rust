use std::{os::windows::raw::HANDLE, str::FromStr};

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

pub static PATH: &str = "/watch/[id]";

pub fn get(mut req: Request, mut res: Response, database: Option<utils::mongodb::Database>) {
    let db = database.unwrap();

    let anime = db.get_anime_id(req.params.get("id").unwrap(), &IdType::KartofPlay, false);

    if anime.is_none() {
        res.send_code(404);
        return;
    }
    let mut anime = anime.unwrap();
    anime.gogo_id = "".to_string();
    anime.animegg_id = "".to_string();
    anime.schedule_id = "".to_string();
    anime.details.movie_id = None;
    anime.details.id = None;

    let json = serde_json::to_value(anime).unwrap_or_default();

    let data = HANDLEBARS.render("watch", &json).unwrap();

    res.set_header(&Header::new(
        "Access-Control-Allow-Origin".to_string(),
        "*".to_string(),
    ));
    res.send_bytes(data.as_bytes(), Some(ContentType::Html));
}

pub fn get_route() -> RouteData {
    return (PATH.to_string(), get);
}
