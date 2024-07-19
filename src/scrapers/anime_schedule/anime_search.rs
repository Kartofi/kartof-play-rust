use crate::utils::http;
use crate::utils::types::*;
use chrono::DurationRound;
use chrono::FixedOffset;
use mongodb::results;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

use chrono::{DateTime, TimeZone, Utc};

pub fn get(query: &str) -> Result<Vec<AnimeDetails>, ScraperError> {
    let mut data: Vec<AnimeDetails> = Vec::new();
    let url = crate::ANIMESCHEDULE.to_owned() + "shows?mt=all&st=search&q=" + &query.replace(" ","+");
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
                if data.len() == 0 {
                    let time_el = root.find("time[id='release-time-subs']");
                    let mut details = AnimeDetails::new();
                    if time_el.is_empty() == false {
                        let time: String = root
                            .find("time[id='release-time-subs']")
                            .attr("datetime")
                            .unwrap()
                            .to_string()
                            .replace("&#43;", ":00+");

                        let datetime: DateTime<FixedOffset> =
                            DateTime::parse_from_rfc3339(&time).unwrap();

                        let timestamp = datetime.timestamp();

                        details.new_ep = timestamp;
                    }
                    let id_el = root.find("html head meta[property='og:url']");
                    if id_el.is_empty() == false {
                        if id_el.has_attr("content") {
                            details.id = Some(
                                id_el
                                    .attr("content")
                                    .unwrap()
                                    .to_string()
                                    .replace("https://animeschedule.net/anime/", ""),
                            );
                        }
                    }
                    data.push(details);
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
