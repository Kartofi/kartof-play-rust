use crate::utils::http;
use crate::utils::types::*;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

pub fn get(page: &str) -> Result<Vec<AnimeRelease>, ScraperError> {
    let mut data: Vec<AnimeRelease> = Vec::new();
    let url = crate::SETTINGS.GOGOANIMEURL_AJAX.to_owned() + "page-recent-release.html?page=" + page;
    let response: Option<String> = http::get(&url);

    if response.is_none() == false {
        match Vis::load(response.unwrap()) {
            Ok(root) => {
                let results = root.find("ul.items").children("");

                for result in results {
                    let children = result.children();
                    let title = children.find("a").attr("title").map(|att| att.to_string());

                    if title.is_none() == true {
                        continue;
                    }
                    let image = children.find("img").attr("src").map(|att| att.to_string());
                    let mut id: Option<String> = children.find("a").attr("href").map(|att| {
                        let binding = att.to_string();
                        let parts: Vec<&str> = binding.split("/").collect();
                        if parts.len() > 0 {
                            parts[1].to_string()
                        } else {
                            "".to_owned()
                        }
                    });
                    let id_str = id.clone().unwrap_or_default();

                    let id_parts: Vec<&str> = id_str.split("-episode-").collect();
                    let mut ep_num: String = "".to_string();
                    if id_parts.len() == 2 {
                        id = Some(id_parts[0].to_string());
                        ep_num = id_parts[1].to_string();
                    }

                    let mut details = AnimeRelease::new();
                    details.title = title;
                    details.cover_url = image.unwrap_or_default();
                    details.id = id;
                    details.episode_num = Some(ep_num);

                    details.is_out = true;
                    details.is_sub = children.html().contains("ic-SUB");

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
