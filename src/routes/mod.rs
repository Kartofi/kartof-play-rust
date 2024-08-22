use choki::structs::*;

use crate::utils;

pub mod player;

pub type RouteData = (
    String,
    fn(req: Request, res: Response, database: Option<utils::mongodb::Database>),
);

pub fn get_routes() -> Vec<RouteData> {
    let mut result: Vec<RouteData> = Vec::new();
    result.push(player::get_route());

    return result;
}
