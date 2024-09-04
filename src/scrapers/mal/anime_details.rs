use std::mem::replace;
use std::u32;

use crate::utils::http;
use crate::utils::types::*;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

pub fn get(id: &str) -> Result<AnimeDetails, ScraperError> {
    let mut data: AnimeDetails = AnimeDetails::new();
    let url = crate::SETTINGS.MALURL.to_owned() + "anime/" + id;
    let response: Option<String> = http::get(&url);

    if response.is_none() == false {
        match Vis::load(response.unwrap()) {
            Ok(root) => {
                //Title
                let title_untrimmed = root.find(".title-name > strong:nth-child(1)").text();
                let title = title_untrimmed.trim();
                //Cover
                let mut image: String = "".to_string();
                let image_el = root
                    .find(".leftside > div:nth-child(1) > a:nth-child(1) > img.lazyload")
                    .first();
                if image_el.is_empty() == false {
                    if image_el.has_attr("src") {
                        image = image_el.attr("src").unwrap().to_string();
                    } else if image_el.has_attr("data-src") {
                        image = image_el.attr("data-src").unwrap().to_string();
                    }
                }
                //Description
                let description_dirty = root
                    .find("p[itemprop='description']")
                    .text()
                    .replace("\n[Written by MAL Rewrite]", "");
                let description = description_dirty.trim();
                //Rating
                data.rating = root.find("span.score-label[itemprop='ratingValue']").text();
                //Genres
                let genres_el = root.find("span[itemprop='genre']");
                for genre_el in genres_el {
                    data.genres.push(genre_el.text());
                }
                //Titles
                let titles_els = root
                    .find("div.js-alternative-titles")
                    .children("div.spaceit_pad");
                for title_el in titles_els {
                    data.other_names.push(
                        title_el
                            .text()
                            .split(": ")
                            .skip(1)
                            .collect::<Vec<&str>>()
                            .join(": ")
                            .trim()
                            .replace("\n", "")
                            .to_string(),
                    );
                }
                //Released
                let released = root
                    .find("span:contains('Premiered:')")
                    .parent("")
                    .text()
                    .replace("\n          ", "")
                    .split(":")
                    .nth(1)
                    .unwrap_or_default()
                    .trim()
                    .to_string();
                //Episodes
                let episodes_str = root
                    .find("span:contains('Episodes:')")
                    .parent("")
                    .text()
                    .replace("\n          ", "")
                    .split(":")
                    .nth(1)
                    .unwrap_or_default()
                    .trim()
                    .to_string();

                data.title = Some(title.to_owned());
                data.id = Some(id.to_string());

                data.description = description.to_string();
                data.released = Some(released);
                data.episodes = episodes_str.parse::<u32>().unwrap_or_default();

                data.cover_url = image;
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
