use std::cmp::Ordering;
use std::sync::Arc;
use std::sync::Mutex;

use crate::scrapers::mal;
use crate::utils::get_timestamp;
use crate::utils::http;

use crate::utils::mongodb::Database;
use crate::utils::types::*;

use choki::structs::ContentType;
use chrono::DurationRound;
use chrono::FixedOffset;
use threadpool::ThreadPool;
use visdom::types::BoxDynError;
use visdom::types::Elements;
use visdom::Vis;

use chrono::{DateTime, TimeZone, Utc};

use crate::scrapers;
use crate::utils;

impl Database {
    pub fn cache_anime(&self, id: &str, id_type: IdType) -> mongodb::error::Result<CacheResult> {
        let found = self.get_anime_id(id, id_type);
        if found.is_none() {
            self.create_new(id)
        } else {
            let mut anime = found.unwrap();
            if get_timestamp() - anime.last_updated > crate::CACHE_ANIME_COUNTDOWN {
                self.update_existing(id, &mut anime)
            } else {
                Ok(CacheResult::new("On cooldown.", true))
            }
        }
    }
    fn create_new(&self, gogo_id: &str) -> mongodb::error::Result<CacheResult> {
        let mut anime = Anime::new();

        let gogoanime_data_result = scrapers::gogoanime::anime_details::get(gogo_id);
        if gogoanime_data_result.is_err() {
            return Ok(CacheResult::new("Invalid id!", true));
        }
        let gogoanime_details = gogoanime_data_result.unwrap();

        anime.id = utils::generate_id().to_string();

        anime.gogo_id = gogo_id.to_string();

        anime.details = gogoanime_details.clone();
        anime.details.id = Some(gogo_id.to_string());

        anime.title = gogoanime_details.title.unwrap_or_default();

        let mut episodes: Vec<Episode> =
            cache_episodes_gogo(&gogoanime_details.movie_id.unwrap_or_default());

        anime.details.episodes = episodes.len() as u32;

        let episodes = Arc::new(Mutex::new(episodes));

        let title = anime.title.clone();

        //Search animeGG and get id
        let animegg_search = scrapers::animegg::anime_search::get(&title).unwrap_or_default();

        if animegg_search.len() > 0 {
            let mut result_anime = AnimeDetails::new();
            let mut found = false;
            for anime_res in animegg_search {
                if anime_res.episodes.abs_diff(anime.details.episodes) < anime.details.episodes / 2
                {
                    found = true;
                    result_anime = anime_res;
                    break;
                }
            }

            if found == true {
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

                let mut thread_count = episodes_animegg.len();
                if thread_count == 0 {
                    thread_count = 1;
                }
                let pool = ThreadPool::new(thread_count);

                episodes
                    .lock()
                    .unwrap()
                    .sort_by(|a, b| compare(&a.num, &b.num));
                for i in 0..episodes_animegg.len() {
                    let clone = episodes.clone();
                    let episodes_animegg = episodes_animegg.clone();
                    pool.execute(move || {
                        let mut episodes = clone.lock().unwrap();
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
                            ep.unwrap().animegg_url = ep_url;
                        }
                    });
                }
                pool.join();
            }
        }

        anime.episodes = episodes.lock().unwrap().to_vec();
        anime.episodes.sort_by(|a, b| compare(&a.num, &b.num));
        // Mal data
        let mal_search = scrapers::mal::anime_search::get(&title).unwrap_or_default();
        if mal_search.len() > 0 {
            let result_mal = mal_search[0].clone();
            anime.mal_id = result_mal.id.unwrap_or_default();
            anime.details.rating = result_mal.rating;

            let details_mal = scrapers::mal::anime_details::get(&anime.mal_id);
            if details_mal.is_ok() {
                let data_details = details_mal.unwrap();
                anime.details.released = data_details.released;
                if anime.details.cover_url.len() == 0 {
                    anime.details.cover_url = data_details.cover_url;
                }
                if anime.details.title.clone().unwrap_or_default().len() == 0 {
                    anime.details.title = data_details.title.clone();
                    anime.title = data_details.title.unwrap_or_default();
                }
            }
        }

        //Anime Schedule

        let schedule_search =
            scrapers::anime_schedule::anime_search::get(&title).unwrap_or_default();
        if schedule_search.len() > 0 {
            let result_schedule = schedule_search[0].clone();

            anime.details.new_ep = result_schedule.new_ep;
            anime.schedule_id = result_schedule.id.unwrap_or_default();
        }
        anime.last_updated = utils::get_timestamp();

        if title.len() > 0 {
            self.insert_new_anime(anime)
        } else {
            Ok(CacheResult::new("No data collected", true))
        }
    }
    fn update_existing(
        &self,
        id: &str,
        current: &mut Anime,
    ) -> mongodb::error::Result<CacheResult> {
        //rating episodes count and episodes
        let mut details = current.details.clone();
        let mut episodes = current.episodes.clone();
        //Mal
        if current.mal_id.len() > 0 {
            let mal_data = scrapers::mal::anime_details::get(&current.mal_id);
            if mal_data.is_ok() {
                details.rating = mal_data.unwrap().rating;
            }
        } else {
            let mal_search = scrapers::mal::anime_search::get(&current.title).unwrap_or_default();
            if mal_search.len() > 0 {
                let result_mal = mal_search[0].clone();
                current.mal_id = result_mal.id.unwrap_or_default();
                details.rating = result_mal.rating;
            }
        }
        //Schedule
        if current.schedule_id.len() > 0 {
            let schedule_data = scrapers::anime_schedule::anime_details::get(&current.schedule_id);
            if schedule_data.is_ok() {
                details.new_ep = schedule_data.unwrap().new_ep;
            }
        } else {
            let schedule_search =
                scrapers::anime_schedule::anime_search::get(&current.title).unwrap_or_default();
            if schedule_search.len() > 0 {
                let result_schedule = schedule_search[0].clone();

                details.new_ep = result_schedule.new_ep;
                current.schedule_id = result_schedule.id.unwrap_or_default();
            }
        }
        //Anime GG
        if current.animegg_id.len() == 0 {
            let animegg_search =
                scrapers::animegg::anime_search::get(&current.title).unwrap_or_default();
            if animegg_search.len() > 0 {
                for anime in animegg_search {
                    if anime.episodes.abs_diff(details.episodes) < details.episodes / 2 {
                        current.animegg_id = anime.id.unwrap_or_default();
                        break;
                    }
                }
            }
        }
        //Episodes
        let gogoanime_details_res = scrapers::gogoanime::anime_details::get(&current.gogo_id);
        if gogoanime_details_res.is_ok() {
            let details_gogo = gogoanime_details_res.unwrap();
            details.episodes = details_gogo.episodes;
        }
        let details_clone = details.clone();

        let pool = ThreadPool::new(episodes.len() + 1);

        let episodes_mt = Arc::from(Mutex::from(episodes));
        let end_iter_eps = if pool.max_count() == 2 {
            1
        } else {
            pool.max_count() - 2
        };
        for i in 0..end_iter_eps {
            let gogo_id = current.gogo_id.to_owned();
            let animegg_id = current.animegg_id.clone();

            let arc_clone = Arc::clone(&episodes_mt);
            pool.execute(move || {
                let mut episodes_locked = arc_clone.lock().unwrap().to_vec();
                let episode_res = episodes_locked.get(i);
                if episode_res.is_none() == false {
                    let mut episode = episode_res.unwrap().clone();

                    if episode.gogoanime_url.len() <= 6 {
                        let result = scrapers::gogoanime::anime_stream::get(
                            &(gogo_id + "-episode-" + &episode.num),
                        );

                        let url = result.unwrap_or_default();
                        if url.len() > 6 {
                            episode.gogoanime_url = url;
                        }
                    }
                    if episode.animegg_url.len() <= 6 {
                        let result = scrapers::animegg::anime_stream::get(
                            &(animegg_id + "-episode-" + &episode.num),
                        );

                        let url = result.unwrap_or_default();
                        if url.len() > 6 {
                            episode.animegg_url = url;
                        }
                    }
                    episodes_locked[i] = episode;
                }
            });
        }
        pool.join();
        if current.details.episodes < details.episodes {
            let mut ep_list_gogo = scrapers::gogoanime::anime_details::get_episodes(
                &details.movie_id.unwrap_or_default(),
            );
            let mut ep_list_animegg =
                scrapers::animegg::anime_details::get_episodes(&&current.animegg_id);

            let missing_eps = (details.episodes - current.details.episodes) as usize;

            if missing_eps < ep_list_gogo.len() {
                ep_list_gogo.splice(0..(ep_list_gogo.len() - missing_eps), std::iter::empty());
            }
            if missing_eps < ep_list_animegg.len() {
                ep_list_animegg
                    .splice(0..(ep_list_animegg.len() - missing_eps), std::iter::empty());
            }

            let pool = ThreadPool::new(missing_eps);

            let episodes_new_mt: Arc<Mutex<Vec<Episode>>> = Arc::new(Mutex::new(Vec::new()));
            if ep_list_gogo.len() >= ep_list_animegg.len() {
                let end_iter = if ep_list_gogo.len() == 1 {
                    1
                } else {
                    ep_list_gogo.len() - 1
                };
                for i in 0..end_iter {
                    let ep_gogo = ep_list_gogo[i].clone();
                    let ep_anime = ep_list_animegg.get(i).unwrap_or(&"".to_string()).clone();
                    let animegg_len = ep_list_animegg.len();

                    let cl = Arc::clone(&episodes_new_mt);
                    pool.execute(move || {
                        let gogo_url =
                            scrapers::gogoanime::anime_stream::get(&ep_gogo).unwrap_or_default();
                        let mut animegg_url = "".to_string();
                        if animegg_len > i {
                            animegg_url =
                                scrapers::animegg::anime_stream::get(&ep_anime).unwrap_or_default()
                        }

                        let mut ep = Episode::new();
                        ep.num = ep_gogo
                            .split("-episode-")
                            .last()
                            .unwrap_or_default()
                            .to_string();
                        ep.animegg_url = animegg_url;
                        ep.gogoanime_url = gogo_url;
                        cl.lock().unwrap().push(ep);
                    });
                    if end_iter == 1 {
                        break;
                    }
                }
            } else {
                let end_iter = if ep_list_animegg.len() == 1 {
                    1
                } else {
                    ep_list_animegg.len() - 1
                };
                for i in 0..end_iter {
                    let ep_animegg = ep_list_animegg[i].clone();
                    let ep_gogo = ep_list_animegg.get(i).unwrap_or(&"".to_string()).clone();
                    let gogo_len = ep_list_gogo.len();

                    let cl = Arc::clone(&episodes_new_mt);
                    pool.execute(move || {
                        let animegg_url =
                            scrapers::animegg::anime_stream::get(&ep_animegg).unwrap_or_default();
                        let mut gogo_url = "".to_string();
                        if gogo_len > i {
                            gogo_url =
                                scrapers::gogoanime::anime_stream::get(&ep_gogo).unwrap_or_default()
                        }

                        let mut ep = Episode::new();
                        ep.num = ep_animegg
                            .split("-episode-")
                            .last()
                            .unwrap_or_default()
                            .to_string();
                        ep.animegg_url = animegg_url;
                        ep.gogoanime_url = gogo_url;
                        cl.lock().unwrap().push(ep);
                    });

                    if end_iter == 1 {
                        break;
                    }
                }
            }
            pool.join();
            let mut eps = episodes_mt.lock().unwrap().to_vec();
            let new_eps = episodes_new_mt.lock().unwrap().to_vec();

            eps.extend(new_eps);
            self.update_anime(
                id,
                Some(details_clone),
                Some(eps),
                Some(&current.animegg_id),
                Some(&current.mal_id),
                Some(&current.schedule_id),
            )
        } else {
            let eps = episodes_mt.lock().unwrap().to_vec();
            self.update_anime(
                id,
                Some(details_clone),
                Some(eps),
                Some(&current.animegg_id),
                Some(&current.mal_id),
                Some(&current.schedule_id),
            )
        }
    }
}
fn compare(a: &str, b: &str) -> Ordering {
    match (a.parse::<u32>(), b.parse::<u32>()) {
        (Ok(a_num), Ok(b_num)) => a_num.cmp(&b_num),
        _ => a.cmp(b),
    }
}

fn cache_episodes_gogo(movie_id: &str) -> Vec<Episode> {
    let mut episodes: Vec<Episode> = Vec::new();

    let mut episodes_gogoanime = scrapers::gogoanime::anime_details::get_episodes(&movie_id);

    let mut thread_count = episodes_gogoanime.len();

    if thread_count == 0 {
        thread_count = 1;
    }
    let pool = ThreadPool::new(thread_count);

    let episodes = Arc::new(Mutex::new(episodes));

    for ep_id in episodes_gogoanime {
        let clone = episodes.clone();
        pool.execute(move || {
            if ep_id.len() != 0 {
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

                let mut episodes = clone.lock().unwrap();
                episodes.push(episode);
            }
        });
    }
    pool.join();
    return episodes.lock().unwrap().to_vec();
}
