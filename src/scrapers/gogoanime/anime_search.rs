use crate::utils::http;
use crate::utils::types::*;
use urlencoding::encode;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

pub fn get(query: &str) -> Result<Vec<AnimeDetails>, ScraperError> {
    let mut data: Vec<AnimeDetails> = Vec::new();
    let url = crate::SETTINGS.GOGOANIMEURL.to_owned() + "search.html?keyword=" + &encode(query.trim());
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
                    let id: Option<String> = children.find("a").attr("href").map(|att| {
                        let binding = att.to_string();
                        let parts: Vec<&str> = binding.split("/").collect();
                        if parts.len() > 0 {
                            parts[1].to_string()
                        } else {
                            "".to_owned()
                        }
                    });
                    let released: String = children
                        .find("p.released")
                        .html()
                        .trim()
                        .split(" ")
                        .nth(1)
                        .unwrap_or_default()
                        .to_string();

                    let mut details = AnimeDetails::new();
                    details.title = title;
                    details.cover_url = image.unwrap_or_default();
                    details.id = id;

                    if released.len() > 0 {
                        details.released = Some(released)
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
