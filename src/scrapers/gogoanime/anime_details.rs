use crate::utils::http;
use crate::utils::types::*;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

pub fn get(id: &str) -> Result<AnimeDetails, ScraperError> {
    let mut data = AnimeDetails::new();
    let url = crate::GOGOANIMEURL.to_owned() + "category/" + id;
    let response: Option<String> = http::get(&url);
    if response.is_none() == false {
        match Vis::load(response.unwrap()) {
            Ok(root) => {
                // Get the cover
                let cover: Option<String> = root
                .find("#wrapper_bg > section > section.content_left > div.main_body > div.anime_info_body > div.anime_info_body_bg > img")
                .attr("src")
                .map(|attr| attr.to_string());
                data.cover_url = cover.unwrap_or_default();
                //Get the title
                let title: String = root
                .find("#wrapper_bg > section > section.content_left > div.main_body > div.anime_info_body > div.anime_info_body_bg > h1")
                .text();
                data.title = if title == "" { None } else { Some(title) };
                //Get the description
                let description: String = root
                .find("html body div#wrapper_inside div#wrapper div#wrapper_bg section.content section.content_left div.main_body div.anime_info_body div.anime_info_body_bg div.description")
                .text().replace("Plot Summary: ", "");

                if description.len() > 1 {
                    data.description = description;
                }

                //Get other names
                let other_names: Vec<String> = root
                .find("html body div#wrapper_inside div#wrapper div#wrapper_bg section.content section.content_left div.main_body div.anime_info_body div.anime_info_body_bg p.type.other-name a")
                .text().replace("Other name: ", "").split("; ").map(|item|  item.to_string()).collect();
                if other_names.len() > 0 && other_names[0].len() > 0 {
                    data.other_names = other_names;
                }
                //Get the genres
                let genres_els: Elements = root
                .find("html body div#wrapper_inside div#wrapper div#wrapper_bg section.content section.content_left div.main_body div.anime_info_body div.anime_info_body_bg p.type").find("a");

                for genre in genres_els {
                    let href = genre.get_attribute("href");
                    let title = genre.get_attribute("title");

                    if href.is_none() == false && title.is_none() == false {
                        let genre_text = title.unwrap().to_string();
                        let href_text = href.unwrap().to_string();
                        if href_text.contains("genre")
                            && href_text.ends_with(&genre_text.to_lowercase().replace(" ", "-"))
                        {
                            data.genres.push(genre_text);
                        }
                    }
                }

                //Get episodes
                let episodes_els = root.find("#episode_page > li");
                let mut episodes: u32 = 0;
                if episodes_els.length() > 0 {
                    episodes = episodes_els
                        .last()
                        .text()
                        .trim()
                        .split("-")
                        .nth(1)
                        .unwrap_or_default()
                        .parse::<u32>()
                        .unwrap_or(0);
                }

                data.episodes = episodes;
                //Get movie id
                let movie_id: Option<String> = root
                    .find("input.movie_id")
                    .first()
                    .attr("value")
                    .map(|att| att.to_string());
                data.movie_id = movie_id;
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
pub fn get_episodes(movie_id: &str) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    let url = crate::GOGOANIMEURL_AJAX.to_owned()
        + "load-list-episode?ep_start=0&ep_end=999999&id="
        + movie_id;
    let response = http::get(&url);
    if response.is_none() == false {
        match Vis::load(response.unwrap()) {
            Ok(root) => {
                let els = root.find("li").children("");

                for ep in els {
                    if ep.has_attribute("href") {
                        if let Some(href) = ep.get_attribute("href") {
                            ids.push(href.to_string().trim().to_string());
                        }
                    }
                }
            }
            Err(_err) => {}
        }
    }
    ids
}
