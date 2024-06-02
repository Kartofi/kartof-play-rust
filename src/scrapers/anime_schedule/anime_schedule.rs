use crate::utils::http;
use crate::utils::types::*;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

pub fn get() -> Result<Vec<AnimeDetails>, ScraperError> {
    let mut data: Vec<AnimeDetails> = Vec::new();
    let url = crate::ANIMESCHEDULE.to_owned();
    let response: Option<String> = http::get(&url);

    if response.is_none() == false {
        match Vis::load(response.unwrap()) {
            Ok(root) => {
                let timezone = root.find("#timetable-timezone-text-mobile").text();

                let today = root.find("#active-day").first().children("");
                for child in today {
                    if child.has_attribute("class") {
                        let class = child.get_attribute("class").unwrap().to_string();
                        if class == "timetable-column-show aired expanded"
                            || class == "timetable-column-show unaired expanded"
                        {
                            // let episode
                        }
                    }
                }
                println!("{}", timezone);
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
