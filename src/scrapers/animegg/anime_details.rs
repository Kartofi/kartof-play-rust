use crate::utils::http;
use crate::utils::types::*;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

pub fn get(id: &str) -> Result<AnimeDetails, ScraperError> {
    let mut data: AnimeDetails = AnimeDetails::new();
    let url = crate::ANIMEGG.to_owned() + "series/" + id;
    let response: Option<String> = http::get(&url);
    if response.is_none() == false {
        match Vis::load(response.unwrap()) {
            Ok(root) => {
                let image_el = root.find("body > div.navbar.navbar-inverse.bs-docs-nav > div.fattynav > div.fattynavinside > div > div > a > img");
                if image_el.has_attr("src") {
                    data.cover_url = image_el.attr("src").unwrap().to_string();
                }

                data.title = Some(root.find("body > div.navbar.navbar-inverse.bs-docs-nav > div.fattynav > div.fattynavinside > div > div > div > div.first > h1").html());
                data.description = root.find("body > div.navbar.navbar-inverse.bs-docs-nav > div.fattynav > div.container > p").html();
                data.id = Some(id.to_string());

                let animeinfo = root.find("p.infoami");

                for info in animeinfo.children("") {
                    let html = info.html();

                    if html.starts_with("Episodes:") {
                        data.episodes = html
                            .replace("Episodes: ", "")
                            .trim()
                            .parse::<u32>()
                            .unwrap_or_default();
                    } else if html.starts_with("Alternate") {
                        data.other_names = html
                            .replace("Alternate Titles: ", "")
                            .split("; ")
                            .map(|item| item.to_string())
                            .collect();
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
pub fn get_episodes(id: &str) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    let url = crate::ANIMEGG.to_owned() + "series/" + id;
    let response = http::get(&url);
    if response.is_none() == false {
        match Vis::load(response.unwrap()) {
            Ok(root) => {
                let els = root.find("a.anm_det_pop");

                for ep in els {
                    if ep.has_attribute("href") {
                        if let Some(href) = ep.get_attribute("href") {
                            ids.push(href.to_string().trim().to_string().replace("/", ""));
                        }
                    }
                }
            }
            Err(_err) => {}
        }
    }
    ids
}
