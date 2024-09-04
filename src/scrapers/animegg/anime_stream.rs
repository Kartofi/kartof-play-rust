use crate::utils::http;
use crate::utils::types::*;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

pub fn get(ep_id: &str) -> Result<String, ScraperError> {
    let mut data: String = "/error".to_string();
    let url = crate::SETTINGS.ANIMEGG.to_owned() + ep_id;

    let response: Option<String> = http::get(&url);
    if response.is_none() == false {
        match Vis::load(response.unwrap()) {
            Ok(root) => {
                let source = root.find("iframe.video");
                if source.is_empty() == false {
                    if source.has_attr("src") {
                        data = crate::SETTINGS.ANIMEGG.to_owned()
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
