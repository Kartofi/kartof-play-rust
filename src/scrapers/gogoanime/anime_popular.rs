use crate::utils::http;
use crate::utils::types::*;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

pub fn get(page: &str) -> Result<Vec<AnimeDetails>, ScraperError> {
    let mut data: Vec<AnimeDetails> = Vec::new();
    let url = crate::GOGOANIMEURL.to_owned() + "popular.html?page=" + page;
    let response: Option<String> = http::get(&url);
    if response.is_none() == false {
        match Vis::load(response.unwrap()) {
            Ok(root) => {
                let results = root.find("div.last_episodes > ul > li");

                for result in results {
                    let children = result.children();

                    let name_el = children.find("a");

                    let title = name_el.attr("title").map(|att| att.to_string());

                    let id: String = name_el
                        .attr("href")
                        .map(|att| att.to_string())
                        .unwrap_or_default()
                        .split("/category/")
                        .nth(1)
                        .unwrap_or_default() // Provide a default if there is no second element
                        .to_string();

                    let image = children.find("img").attr("src").map(|att| att.to_string());
                    let released: String = result
                        .text()
                        .trim()
                        .replace(" ", "")
                        .split("Released:")
                        .nth(1)
                        .unwrap_or_default()
                        .to_string();

                    let mut details = AnimeDetails::new();
                    details.title = title;
                    details.cover_url = image.unwrap_or_default();
                    if id.len() > 0 {
                        details.id = Some(id);
                    }
                    if released.len() > 0 {
                        details.released = Some(released);
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
