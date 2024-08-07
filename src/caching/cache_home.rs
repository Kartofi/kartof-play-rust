use chrono::{Datelike, Utc};

use crate::{
    scrapers::{animegg, gogoanime},
    utils::{
        get_timestamp,
        mongodb::Database,
        types::{Anime, AnimeDetails, AnimeRelease, CacheResult, Home, IdType},
    },
};

impl Database {
    pub fn cache_home(&self) -> mongodb::error::Result<(CacheResult, Home)> {
        let mut home = Home::new();

        let time = Utc::now();
        home.date = time.day().to_string()
            + ":"
            + &time.month().to_string()
            + ":"
            + &time.year().to_string();

        let search = self.get_home(&home.date).unwrap_or_default();
        if search.is_some()
            && get_timestamp() - search.as_ref().unwrap().last_updated < crate::CACHE_COUNTDOWN
        {
            return Ok((CacheResult::new("On cooldown!", true), Home::new()));
        }
        let recent = self.get_recent();
        if recent.is_ok() {
            home.recent = recent.unwrap().1;
        }
        let popular = self.get_popular();
        if popular.is_ok() {
            home.popular = popular.unwrap().1;
        }
        Ok((self.update_home(home.clone(), search).unwrap(), home))
    }

    fn get_recent(&self) -> mongodb::error::Result<(CacheResult, Vec<AnimeRelease>)> {
        let mut recent: Vec<AnimeRelease> = Vec::new();
        let mut recent_type = IdType::Gogoanime;

        recent = gogoanime::anime_recent::get("1").unwrap_or_default();
        if recent.len() == 0 {
            recent = animegg::anime_recent::get().unwrap_or_default();
            recent_type = IdType::AnimeGG;
        }
        let a = AnimeRelease::new();

        if recent.len() == 0 {
            return Ok((CacheResult::new("Empty sources", true), Vec::new()));
        }
        let mut result: Vec<AnimeRelease> = Vec::new();

        for anime in recent {
            let anime_data_res = self.get_anime_id(&anime.id.unwrap_or_default(), &recent_type);
            if anime_data_res.is_none() == true {
                continue;
            }
            let anime_data = anime_data_res.unwrap();

            result.push(AnimeRelease {
                id: Some(anime_data.id),
                title: Some(anime_data.title),
                episode_num: anime.episode_num,
                is_sub: anime.is_sub,
                is_out: true,
                cover_url: anime_data.details.cover_url,
                release_time: anime.release_time,
            })
        }

        Ok((CacheResult::new("No errors", false), result))
    }
    fn get_popular(&self) -> mongodb::error::Result<(CacheResult, Vec<AnimeDetails>)> {
        let mut popular: Vec<AnimeDetails> = Vec::new();
        let mut popular_type = IdType::Gogoanime;

        popular = gogoanime::anime_popular::get("1").unwrap_or_default();

        if popular.len() == 0 {
            popular = Vec::new();
            popular_type = IdType::AnimeGG;
        }
        let a = AnimeRelease::new();

        if popular.len() == 0 {
            return Ok((CacheResult::new("Empty sources", true), Vec::new()));
        }
        let mut result: Vec<AnimeDetails> = Vec::new();

        for anime in popular {
            let anime_data_res = self.get_anime_id(&anime.id.unwrap_or_default(), &popular_type);
            if anime_data_res.is_none() == true {
                continue;
            }
            let anime_data = anime_data_res.unwrap();

            let mut details = AnimeDetails::new();

            details.id = Some(anime_data.id);
            details.title = Some(anime_data.title);
            details.rating = anime_data.details.rating;
            details.episodes = anime_data.details.episodes;
            details.new_ep = anime_data.details.new_ep;
            details.cover_url = anime_data.details.cover_url;
            details.released = if anime_data.details.released.is_none() == true {
                anime.released
            } else {
                anime_data.details.released
            };

            result.push(details)
        }

        Ok((CacheResult::new("No errors", false), result))
    }
    fn get_schedule_today() {}
}
