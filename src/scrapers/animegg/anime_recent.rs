use crate::utils::http;
use crate::utils::types::*;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

pub fn get() -> Result<Vec<AnimeRelease>, ScraperError> {
    let mut data: Vec<AnimeRelease> = Vec::new();
    let url = crate::ANIMEGG.to_owned() + "releases/";
    let response: Option<String> = http::get(&url);
    if response.is_none() == false {
        match Vis::load(response.unwrap()) {
            Ok(root) => {
                let results = root.find("ul[class='popanime cats']").children("li");

                for result in results {
                    let children = result.children();

                    let id = children.find("a.releaseLink");

                    if (id.is_empty()) {
                        continue;
                    }
                    let mut details = AnimeRelease::new();
                    details.title = Some(id.text());
                    if id.has_attr("href") {
                        details.id =
                            Some(id.attr("href").unwrap().to_string().replace("/series/", ""))
                    }

                    let image = children.find("div.releaseImg").find("img");

                    if image.is_empty() == false && image.has_attr("src") {
                        details.cover_url = image.attr("src").unwrap().to_string();
                    }

                    details.episode_num = Some(
                        children
                            .find("strong")
                            .text()
                            .split(" Episode ")
                            .nth(1)
                            .unwrap_or_default()
                            .to_string(),
                    );

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
