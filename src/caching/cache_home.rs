use crate::{
    scrapers::{animegg, gogoanime},
    utils::{
        mongodb::Database,
        types::{Anime, AnimeRelease, CacheResult, IdType},
    },
};

impl Database {
    pub fn cache_recent(&self) -> mongodb::error::Result<(CacheResult, Vec<AnimeRelease>)> {
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
    pub fn cache_popular() {}
    pub fn cache_schedule_today() {}
}
