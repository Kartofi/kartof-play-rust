use crate::utils::http;
use crate::utils::types::*;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

pub fn get(query: &str) -> Result<Vec<AnimeDetails>, ScraperError> {
    let mut data: Vec<AnimeDetails> = Vec::new();
    let url = crate::ANIMEGG.to_owned() + "search/?q=" + query;
    let response: Option<String> = http::get(&url);
    if response.is_none() == false {
        match Vis::load(response.unwrap()) {
            Ok(root) => {
                let results = root.find("div[class='moose page']").children("a");
                for result in results {
                    if result.has_attribute("href") == false {
                        continue;
                    }
                    let children = result.children();

                    let mut details = AnimeDetails::new();
                    details.id = Some(
                        result
                            .get_attribute("href")
                            .unwrap()
                            .to_string()
                            .replace("/series/", ""),
                    );
                    let image_el = children.find("img.media-object");
                    if image_el.has_attr("src") {
                        details.cover_url = image_el.attr("src").unwrap().to_string();
                    }
                    details.title = Some(children.find("h2").text());

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
