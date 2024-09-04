use crate::utils::http;
use crate::utils::types::*;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

pub fn get(page: &str) -> Result<Vec<String>, ScraperError> {
    let mut data: Vec<String> = Vec::new();
    let url = crate::SETTINGS.GOGOANIMEURL.to_owned() + "anime-list.html?page=" + page;
    let response: Option<String> = http::get(&url);

    if response.is_none() == false {
        match Vis::load(response.unwrap()) {
            Ok(root) => {
                let animes = root.find("div.anime_list_body > ul > li > a");
                for anime in animes {
                    if anime.has_attribute("href") {
                        data.push(
                            anime
                                .get_attribute("href")
                                .unwrap()
                                .to_string()
                                .replace("/category/", ""),
                        );
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
