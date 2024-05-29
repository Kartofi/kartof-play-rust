use crate::utils::http;
use crate::utils::types::*;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

pub fn get(id: &str) -> Result<AnimeDetails, ScraperError> {
    let mut data = AnimeDetails::new();
    let url = crate::GOGOANIMEURLL.to_owned() + "category/" + id;
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
                if other_names.len() > 0 {
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
