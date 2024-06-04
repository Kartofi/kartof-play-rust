use crate::utils::http;
use crate::utils::types::*;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

pub fn get(id: &str, episode: &str) -> Result<String, ScraperError> {
    let mut data: String = "/error".to_string();
    let url = crate::ANIMEGG.to_owned() + id + "-episode-" + episode;
    let response: Option<String> = http::get(&url);
    if response.is_none() == false {
        match Vis::load(response.unwrap()) {
            Ok(root) => {
                let source = root.find("iframe.video");
                if source.is_empty() == false {
                    if source.has_attr("src") {
                        data = crate::ANIMEGG.to_owned()
                            + &source
                                .attr("src")
                                .unwrap()
                                .to_string()
                                .replace("/embed", "embed");
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
