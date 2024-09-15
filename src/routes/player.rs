use choki::structs::*;
use serde_json::json;

use crate::{
    scrapers,
    utils::{
        self,
        types::{Anime, IdType},
    },
    HANDLEBARS,
};

use super::RouteData;

pub static PATH: &str = "/player/[id]/[ep_num]";

pub fn get(mut req: Request, mut res: Response, database: Option<utils::mongodb::Database>) {
    let id = req.params.get("id").unwrap();
    let ep_str = req.params.get("ep_num").unwrap();
    let ep_num: usize = ep_str.parse().unwrap_or(1);

    let database = database.unwrap();

    let anime = database
        .get_anime_id(id, &IdType::KartofPlay, false)
        .unwrap_or(Anime::new());

    let mut url: String = "".to_string();

    if anime.id.len() != 0 {
        let stream_url = database.get_gogo_stream(&anime.id, ep_num).unwrap();
        if stream_url.is_none() {
            url = scrapers::gogoanime::anime_streaming_url::get(
                &(anime.gogo_id.to_string() + "-episode-" + ep_str),
            )
            .unwrap_or_default();

            let _e = database
                .set_gogo_stream(id, ep_num, url.clone(), anime.details.episodes as usize)
                .unwrap();
        } else {
            let stream_url = stream_url.unwrap();
            if stream_url.is_expired() || stream_url.url.len() == 0 {
                url = scrapers::gogoanime::anime_streaming_url::get(
                    &(anime.gogo_id.to_string() + "-episode-" + ep_str),
                )
                .unwrap_or_default();

                let _e = database
                    .set_gogo_stream(id, ep_num, url.clone(), anime.details.episodes as usize)
                    .unwrap();
            } else {
                url = stream_url.url;
            }
        }
    }

    let data = HANDLEBARS.render("player", &json!({"url": url})).unwrap();
    res.set_header(&Header::new(
        "Access-Control-Allow-Origin".to_string(),
        "*".to_string(),
    ));
    res.send_bytes(data.as_bytes(), Some(ContentType::Html));
}

pub fn get_route() -> RouteData {
    return (PATH.to_string(), get);
}
