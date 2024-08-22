use choki::structs::*;
use dojang::Dojang;
use serde_json::json;

use crate::{
    scrapers,
    utils::{
        self,
        types::{Anime, IdType},
    },
};

use super::RouteData;

pub static PATH: &str = "/player/[id]/[ep_num]";

pub fn get(mut req: Request, mut res: Response, database: Option<utils::mongodb::Database>) {
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
}

pub fn get_route() -> RouteData {
    return (PATH.to_string(), get);
}
