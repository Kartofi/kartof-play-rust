use crate::utils::http;
use crate::utils::types::*;
use chrono::DurationRound;
use mongodb::results;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

use chrono::{DateTime, TimeZone, Utc};

pub fn get(query: &str) -> Result<Vec<AnimeDetails>, ScraperError> {
    let mut data: Vec<AnimeDetails> = Vec::new();
    let url = crate::ANIMESCHEDULE.to_owned() + "shows?q=" + query;
    let response: Option<String> = http::get(&url);

    if response.is_none() == false {
        match Vis::load(response.unwrap()) {
            Ok(root) => {
                let results = root.find("div.anime-tile.lozad");

                for result in results {
                    let mut anime = AnimeDetails::new();

                    if result.has_attribute("id") {
                        anime.id = Some(result.get_attribute("id").unwrap().to_string());
                    } else {
                        continue;
                    }

                    if result.has_attribute("genres") {
                        anime.genres = result
                            .get_attribute("genres")
                            .unwrap()
                            .to_string()
                            .split(',')
                            .map(|item| item.to_string())
                            .collect::<Vec<String>>();
                    }
                    let children = result.children();

                    anime.title = Some(children.find("h2.anime-tile-title").text());

                    let image_el = children.find("img.anime-tile-thumbnail");

                    if image_el.has_attr("src") {
                        anime.cover_url = children
                            .find("img.anime-tile-thumbnail")
                            .attr("src")
                            .unwrap()
                            .to_string();
                    }

                    let description = children.find("p.anime-tile-description").text();
                    anime.description = description.trim().to_string();

                    data.push(anime);
                }
            }
            Err(err) => {
                return Err(ScraperError {
                    reason: "Error Parsing page".to_owned(),
                })
            }
        }
    } else {
        return Err(ScraperError {
            reason: "Failed to make http request".to_owned(),
        });
    }
    Ok(data)
}
