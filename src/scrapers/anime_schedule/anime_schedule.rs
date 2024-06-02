use crate::utils::http;
use crate::utils::types::*;
use chrono::DurationRound;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

use chrono::{DateTime, TimeZone, Utc};

pub fn get() -> Result<Vec<AnimeRelease>, ScraperError> {
    let mut data: Vec<AnimeRelease> = Vec::new();
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
                            let children = child.children().parent("");

                            let time_bar = children.find("h3.time-bar");
                            let episode = time_bar.find("span.show-episode").text();

                            let time = time_bar
                                .find("time.show-air-time")
                                .attr("datetime")
                                .unwrap()
                                .to_string();

                            let show = children.find("a.show-link");
                            let image_el = show.find("img");
                            let mut image = "".to_string();

                            if image_el.has_attr("src") {
                                image = image_el.attr("src").unwrap().to_string();
                            }

                            let id = children.attr("route").unwrap().to_string();
                            let title = show.find("h2.show-title-bar").text();
                            if episode.len() <= 0 {
                                continue;
                            }
                            let mut out = false;
                            let now = Utc::now();
                            let release_time =
                                DateTime::parse_from_rfc3339(&time).unwrap_or_default();

                            if release_time < now {
                                out = true;
                            }

                            let mut release_data = AnimeRelease::new();
                            release_data.is_out = out;
                            release_data.cover_url = image;
                            release_data.episode_num = Some(episode);
                            release_data.id = Some(id);
                            release_data.title = Some(title);
                            release_data.release_time = Some(time);
                            release_data.is_sub = children.html().contains("SUB</span>");

                            data.push(release_data);
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
