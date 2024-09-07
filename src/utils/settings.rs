use std::{ fs, path::Path, time::Duration };

use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
pub struct Settings {
    // Anime Schedule
    pub ANIMESCHEDULE: String,
    // AnimeGG
    pub ANIMEGG: String,
    // Mal
    pub MALURL: String,
    // Gogoanime
    pub GOGOANIMEURL: String,
    pub GOGOANIMEURL_AJAX: String,

    pub UPDATE_ALL_ANIME_THREADS: usize,

    pub CACHE_COUNTDOWN: i64, // 5 MINS

    pub CACHE_HOME_FREQUENCY_NUM: i64, // 5 MINS
    pub CACHE_HOME_FREQUENCY: Duration, // 5 MINS

    pub CACHE_ALL_ANIME_FREQUENCY: Duration, // 7 Days
    pub CACHE_ALL_IMAGES_FREQUENCY: Duration, // 7 Days

    pub HTTP_REQUEST_TIMEOUT: Duration,
    pub HTTP_FREQUENCY_TIMEOUT: Duration,

    pub IMAGES_PATH: String,
    pub CACHE_SLEEP: Duration,
    pub REPORT_COUNT: usize,
}

impl Settings {
    pub fn default() -> Settings {
        Settings {
            ANIMESCHEDULE: "https://animeschedule.net/".to_string(),
            ANIMEGG: "https://www.animegg.org/".to_string(),
            MALURL: "https://myanimelist.net/".to_string(),
            GOGOANIMEURL: "https://gogoanime3.co/".to_string(),
            GOGOANIMEURL_AJAX: "https://ajax.gogocdn.net/ajax/".to_string(),

            UPDATE_ALL_ANIME_THREADS: 100,

            CACHE_COUNTDOWN: 300, // 5 MINS

            CACHE_HOME_FREQUENCY_NUM: 300, // 5 MINS
            CACHE_HOME_FREQUENCY: Duration::from_secs(300), // 5 MINS

            CACHE_ALL_ANIME_FREQUENCY: Duration::from_secs(604800), // 7 Days
            CACHE_ALL_IMAGES_FREQUENCY: Duration::from_secs(259200), // 3 Days

            HTTP_REQUEST_TIMEOUT: Duration::from_secs(4),
            HTTP_FREQUENCY_TIMEOUT: Duration::from_secs(2),

            IMAGES_PATH: "./images".to_string(),
            CACHE_SLEEP: Duration::from_millis(100),
            REPORT_COUNT: 100,
        }
    }
    pub fn from_file(path: &str) -> Settings {
        let string_data = fs::read_to_string(path).unwrap();
        serde_json::from_str(&string_data).unwrap()
    }
    pub fn save_to_file(&self) {
        let string_data = serde_json::to_string_pretty(self).unwrap();
        fs::write("./settings.json", string_data).unwrap();
    }
    fn exits() -> bool {
        Path::new("./settings.json").exists()
    }
    pub fn init() -> Settings {
        if Self::exits() {
            return Self::from_file("./settings.json");
        } else {
            let settings = Settings::default();
            settings.save_to_file();
            return settings;
        }
    }
}
