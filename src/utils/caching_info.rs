use std::{ fs, path::Path, time::Duration };

use serde::{ Deserialize, Serialize };

use super::get_timestamp;

#[derive(Serialize, Deserialize)]
pub struct CachingInfo {
    last_all_anime_cache: i64, 
    last_all_images_cache: i64, 
}

impl CachingInfo {
    pub fn default() -> CachingInfo {
        CachingInfo {
            last_all_anime_cache: get_timestamp(),
            last_all_images_cache: get_timestamp(),
        }
    }
    pub fn from_file(path: &str) -> CachingInfo {
        let string_data = fs::read_to_string(path).unwrap();
        serde_json::from_str(&string_data).unwrap()
    }
    pub fn save_to_file(&self) {
        let string_data = serde_json::to_string_pretty(self).unwrap();
        fs::write("./caching_info.json", string_data).unwrap();
    }
    pub fn update_time() {
        let string_data = serde_json::to_string_pretty(&CachingInfo::default()).unwrap();
        fs::write("./caching_info.json", string_data).unwrap();
    }
    pub fn get_all_anime_time() -> i64 {
        return Self::from_file("./caching_info.json").last_all_anime_cache;
    }
    pub fn get_all_images_time() -> i64 {
        return Self::from_file("./caching_info.json").last_all_images_cache;
    }
    fn exits() -> bool {
        Path::new("./caching_info.json").exists()
    }
    pub fn init() -> CachingInfo {
        if Self::exits() {
            return Self::from_file("./caching_info.json");
        } else {
            let caching_info = CachingInfo::default();
            caching_info.save_to_file();
            return caching_info;
        }
    }
}
