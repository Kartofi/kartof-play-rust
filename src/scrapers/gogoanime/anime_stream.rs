use crate::utils::http;
use crate::utils::types::*;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

pub fn get(ep_id: &str) -> Result<String, ScraperError> {
    let url = crate::GOGOANIMEURL.to_owned() + ep_id;
    let response: Option<String> = http::get(&url);
    if response.is_none() == false {
        match Vis::load(response.unwrap()) {
            Ok(root) => {
                let iframe = root.find("#load_anime > div > div > iframe").first();
                if iframe.has_attr("src") {
                    Ok(iframe.attr("src").unwrap().to_string())
                } else {
                    return Err(ScraperError {
                        reason: "Error getting the url".to_owned(),
                    });
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
}
