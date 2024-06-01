use std::u32;

use crate::utils::http;
use crate::utils::types::*;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

pub fn get(id: &str) -> Result<AnimeDetails, ScraperError> {
    let mut data: AnimeDetails = AnimeDetails::new();
    let url = crate::MALURL.to_owned() + "anime/" + id;
    let response: Option<String> = http::get(&url);

    if response.is_none() == false {
        match Vis::load(response.unwrap()) {
            Ok(root) => {
                //Title
                let title_untrimmed = root.find(".title-name > strong:nth-child(1)").text();
                let title = title_untrimmed.trim();
                //Cover
                let mut image: String = "".to_string();
                let image_el = root
                    .find(".leftside > div:nth-child(1) > a:nth-child(1) > img.lazyload")
                    .first();
                if image_el.is_empty() == false {
                    if image_el.has_attr("src") {
                        image = image_el.attr("src").unwrap().to_string();
                    } else if image_el.has_attr("data-src") {
                        image = image_el.attr("data-src").unwrap().to_string();
                    }
                }

                data.cover_url = image;
                data.title = Some(title.to_owned());
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
