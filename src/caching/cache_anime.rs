use crate::utils::http;
use crate::utils::mongodb::Database;
use crate::utils::types::*;
use chrono::DurationRound;
use chrono::FixedOffset;
use mongodb::results;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

use chrono::{DateTime, TimeZone, Utc};

use crate::scrapers;
use crate::utils;

impl Database {
    pub fn run(&self, id: &str) -> mongodb::error::Result<bool> {
        self.create_new(id)
    }
    fn create_new(&self, id: &str) -> mongodb::error::Result<bool> {
        let mut anime = Anime::new();

        let gogoanime_data_result = scrapers::gogoanime::anime_details::get(id);
        if gogoanime_data_result.is_err() {
            return Ok(false);
        }
        let gogoanime_details = gogoanime_data_result.unwrap();

        anime.id = id.to_string();

        anime.details = gogoanime_details.clone();
        anime.details.id = Some(id.to_string());

        anime.title = gogoanime_details.title.unwrap_or_default();

        let mut episodes: Vec<Episode> = Vec::new();

        let mut episodes_gogoanime = scrapers::gogoanime::anime_details::get_episodes(
            &gogoanime_details.movie_id.unwrap_or_default(),
        );
        episodes_gogoanime.reverse();
        for ep_id in episodes_gogoanime {
            if ep_id.len() == 0 {
                continue;
            }
            let mut episode = Episode::new();
            episode.num = ep_id
                .split("-episode-")
                .last()
                .unwrap_or_default()
                .to_string();
            let stream_url = scrapers::gogoanime::anime_stream::get(&ep_id).unwrap_or_default();
            if stream_url.is_empty() == false {
                episode.gogoanime_url = stream_url;
            }
            episodes.push(episode);
        }

        let title = anime.title.clone();
        //Search animeGG and get id
        let animegg_search = scrapers::animegg::anime_search::get(&title).unwrap_or_default();
        if animegg_search.len() > 0 {
            let result_anime = animegg_search.get(0).unwrap().to_owned();
            anime.animegg_id = result_anime.id.unwrap_or_default();

            if anime.details.other_names.len() == 0 {
                anime.details.other_names = result_anime.other_names;
            }
            if result_anime.episodes > anime.details.episodes
                && result_anime.episodes - anime.details.episodes < 5
            {
                anime.details.episodes = result_anime.episodes;
            }

            anime.details.released = result_anime.released;

            let mut episodes_animegg =
                scrapers::animegg::anime_details::get_episodes(&anime.animegg_id);
            episodes_animegg.reverse();
            for i in 0..episodes_animegg.len() {
                let ep = episodes.get_mut(i);
                if ep.is_none() {
                    let mut episode = Episode::new();

                    let ep_url = scrapers::animegg::anime_stream::get(&episodes_animegg[i])
                        .unwrap_or_default();

                    episode.animegg_url = ep_url;
                    episode.num = episodes_animegg[i].clone();

                    episodes.push(episode)
                } else {
                    let ep_url = scrapers::animegg::anime_stream::get(&episodes_animegg[i])
                        .unwrap_or_default();
                    println!("{}", episodes_animegg[i]);
                    ep.unwrap().animegg_url = ep_url;
                }
            }
        }

        anime.episodes = episodes;

        // Mal data
        let mal_search = scrapers::mal::anime_search::get(&title).unwrap_or_default();
        if mal_search.len() > 0 {
            let result_mal = mal_search[0].clone();
            anime.mal_id = result_mal.id.unwrap_or_default();
            anime.details.rating = result_mal.rating;
        }
        self.insert_new_anime(anime)
    }
}
