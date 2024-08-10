use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Home {
    pub date: String, // in format day:month:year

    pub last_updated: i64,

    pub schedule: Vec<AnimeRelease>,
    pub recent: Vec<AnimeRelease>,
    pub popular: Vec<AnimeDetails>,
}
impl Home {
    pub fn new() -> Home {
        Home {
            date: "".to_string(),
            last_updated: 0,

            schedule: Vec::new(),
            recent: Vec::new(),
            popular: Vec::new(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Anime {
    pub title: String,
    //Ids
    pub id: String,

    pub gogo_id: String,
    pub animegg_id: String,
    pub mal_id: String,
    pub schedule_id: String,
    //
    pub details: AnimeDetails,
    pub episodes: Vec<Episode>,
    //
    pub last_updated: i64,
}
impl Anime {
    pub fn new() -> Anime {
        Anime {
            title: "".to_string(),
            id: "".to_string(),
            gogo_id: "".to_string(),
            animegg_id: "".to_string(),
            mal_id: "".to_string(),
            schedule_id: "".to_string(),
            details: AnimeDetails::new(),
            episodes: Vec::new(),
            last_updated: 0,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnimeDetails {
    pub id: Option<String>,
    pub title: Option<String>,
    pub released: Option<String>,
    pub movie_id: Option<String>, // Gogoanime ajax thingy
    pub episodes: u32,

    pub other_names: Vec<String>,
    pub genres: Vec<String>,

    pub description: String,
    pub cover_url: String,
    pub rating: String,

    pub new_ep: i64,
}
impl AnimeDetails {
    pub fn new() -> AnimeDetails {
        AnimeDetails {
            title: None,
            id: None,
            released: None,
            other_names: Vec::new(),
            genres: Vec::new(),
            movie_id: None,
            episodes: 0,
            description: "".to_string(),
            cover_url: "".to_string(),
            rating: "".to_string(),
            new_ep: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnimeRelease {
    pub id: Option<String>,
    pub title: Option<String>,
    pub episode_num: Option<String>,

    pub is_sub: bool,
    pub is_out: bool,

    pub cover_url: String,
    pub release_time: Option<i64>,
}
impl AnimeRelease {
    pub fn new() -> AnimeRelease {
        AnimeRelease {
            title: None,
            id: None,
            episode_num: None,
            is_sub: false,
            is_out: false,
            cover_url: "".to_string(),
            release_time: None,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Episode {
    pub num: String,
    pub gogoanime_url: String,
    pub animegg_url: String,
}
impl Episode {
    pub fn new() -> Episode {
        Episode {
            num: "".to_string(),
            gogoanime_url: "/error".to_string(),
            animegg_url: "/error".to_string(),
        }
    }
}
#[derive(Debug)]
pub struct ScraperError {
    pub reason: String,
}
#[derive(Debug)]
pub struct CacheResult {
    pub reason: String,
    pub is_error: bool,
}
impl CacheResult {
    pub fn new(reason: &str, error: bool) -> CacheResult {
        CacheResult {
            reason: reason.to_string(),
            is_error: error,
        }
    }
}
#[derive(PartialEq)]
pub enum IdType {
    KartofPlay,
    Gogoanime,
    AnimeGG,
    AnimeSchedule,
    MAL,
}
