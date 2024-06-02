use std::u32;

use crate::utils::http;
use crate::utils::types::*;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

pub fn get(query: &str) -> Result<Vec<AnimeDetails>, ScraperError> {
    let mut data: Vec<AnimeDetails> = Vec::new();
    let mut query = query;

    if query.len() > 100 {
        query = query.split(",").nth(1).unwrap_or_default();
    }
    let url = crate::MALURL.to_owned() + "anime.php?cat=anime&q=" + query;
    let response: Option<String> = http::get(&url);
    if response.is_none() == false {
        match Vis::load(response.unwrap()) {
            Ok(root) => {
                let results = root
                    .find("div.js-categories-seasonal.js-block-list.list > table")
                    .children("tr");

                for result in results.into_iter().skip(1) {
                    let mut anime = AnimeDetails::new();

                    let childs = result.children();

                    let mut title: String = "".to_string();

                    let mut image: String = "".to_string();
                    let image_el = childs.find("a.hoverinfo_trigger").children("").first();
                    if let Some(img) = image_el.attr("data-srcset") {
                        let img_str = img.to_string();
                        let parts: Vec<&str> = img_str.split(" ").collect();
                        if parts.len() >= 3 {
                            image = parts[2]
                                .replace(".jpg", "l.jpg")
                                .replace("/r/100x140", "")
                                .to_string();
                        }
                    }
                    if let Some(title_att) = image_el.attr("alt") {
                        title = title_att.to_string();
                    }
                    let desc = childs.find("div.pt4").text();
                    let rating = childs.last().text().trim().to_string();
                    let res = childs.find("td");

                    let mut episodes_count = 0;
                    if res.length() >= 4 {
                        let t: u32 = res.children("").first().text().parse().unwrap_or_default();
                        episodes_count = t;
                    }
                    let mut id: Option<String> = None;

                    let pic_surround = childs.find("div.picSurround").first();
                    if pic_surround.is_empty() == false {
                        let childs = pic_surround.children("a").first();
                        if childs.is_empty() == false {
                            if let Some(href) = childs.attr("href") {
                                let string_href = href.to_string();

                                let parts: Vec<&str> = string_href.split("anime/").collect();
                                if parts.len() == 2 {
                                    let id_title: Vec<&str> = parts[1].split("/").collect();
                                    if id_title.len() == 2 {
                                        id = Some(id_title[0].to_string());
                                    }
                                }
                            }
                        }
                    }

                    anime.id = id;
                    anime.title = Some(title);
                    anime.cover_url = image;
                    anime.description = desc;
                    anime.rating = rating;
                    anime.episodes = episodes_count;

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
