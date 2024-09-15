use std::fs;

use choki::structs::*;

use serde_json::json;

use crate::{
    scrapers,
    utils::{
        self, images,
        types::{Anime, IdType},
    },
};

use super::RouteData;

pub static PATH: &str = "/images/[id]";

pub fn get(mut req: Request, mut res: Response, database: Option<utils::mongodb::Database>) {
    let image = images::get_image(req.params.get("id").unwrap());
    if image.is_none() {
        res.send_code(404);
    } else {
        res.send_bytes(&image.unwrap().data, None);
    }
}

pub fn get_route() -> RouteData {
    return (PATH.to_string(), get);
}
