use crate::utils::http;
use crate::utils::types::*;
use chrono::DurationRound;
use chrono::FixedOffset;
use mongodb::results;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

use chrono::{DateTime, TimeZone, Utc};

pub fn get(id: &str) -> Result<AnimeDetails, ScraperError> {
    let mut data: AnimeDetails = AnimeDetails::new();
    let url = crate::ANIMESCHEDULE.to_owned() + "anime/" + id;
    let response: Option<String> = http::get(&url);

    if response.is_none() == false {
        match Vis::load(response.unwrap()) {
            Ok(root) => {
                let time_res = root.find("time[id='release-time-subs']").attr("datetime");
                let mut timestamp: i64 = 0;

                if time_res.is_none() == false {
                    let datetime: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(
                        &time_res.unwrap().to_string().replace("&#43;", ":00+"),
                    )
                    .unwrap();
                    timestamp = datetime.timestamp();
                }

                //let time_zone = root.find("div.release-time-timezone-text").text();
                let title = root.find("#anime-header-main-title").text();
                let description = root.find("#description").text().replace("\n", "");

                let image_el = root.find("#anime-poster");

                if image_el.has_attr("src") {
                    data.cover_url = image_el.attr("src").unwrap().to_string();
                }
                data.id = Some(id.to_string());
                data.new_ep = timestamp;
                data.title = Some(title);
                data.description = description;
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
