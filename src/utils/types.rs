use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Anime {
    pub title: String,
    //Ids
    pub id: String,
    pub animegg_id: String,
    pub mal_id: String,
    pub schedule_id: String,
    //
    pub details: AnimeDetails,
    pub episodes: Vec<Episode>,
}
impl Anime {
    pub fn new() -> Anime {
        Anime {
            title: "".to_string(),
            id: "".to_string(),
            animegg_id: "".to_string(),
            mal_id: "".to_string(),
            schedule_id: "".to_string(),
            details: AnimeDetails::new(),
            episodes: Vec::new(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeDetails {
    pub title: Option<String>,

    pub other_names: Vec<String>,
    pub genres: Vec<String>,

    pub description: String,
    pub cover_url: String,
    pub rating: String,

    pub new_ep: u64,
}
impl AnimeDetails {
    pub fn new() -> AnimeDetails {
        AnimeDetails {
            title: None,
            other_names: Vec::new(),
            genres: Vec::new(),
            description: "".to_string(),
            cover_url: "".to_string(),
            rating: "".to_string(),
            new_ep: 0,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    pub num: u32,
    pub gogoanime_url: String,
    pub animegg_url: String,
}
impl Episode {
    pub fn new() -> Episode {
        Episode {
            num: 0,
            gogoanime_url: "".to_string(),
            animegg_url: "".to_string(),
        }
    }
}
#[derive(Debug)]
pub struct ScraperError {
    pub reason: String,
}
