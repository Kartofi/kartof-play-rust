use choki::structs::*;

use crate::utils::{ self };

pub mod home;
pub mod images;
pub mod player;
pub mod search;

pub type RouteData = (
    String,
    fn(req: Request, res: Response, database: Option<utils::mongodb::Database>),
);

pub fn get_routes() -> Vec<RouteData> {
    let mut result: Vec<RouteData> = Vec::new();
    result.push(player::get_route());
    result.push(home::get_route());
    result.push(images::get_route());
    result.push(search::get_route());
    return result;
}
