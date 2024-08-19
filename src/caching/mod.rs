use std::{rc::Rc, sync::Arc, thread, time::Duration};

use chrono::Utc;
use threadpool::ThreadPool;

use crate::{
    utils::{
        get_date_string, get_timestamp,
        mongodb::Database,
        types::{AnimeRelease, Home, IdType},
    },
    CACHE_HOME_FREQUENCY, CACHE_HOME_FREQUENCY_NUM,
};

pub mod cache_anime;

pub mod cache_home;

pub fn start(database: Database) {
    let database_clone = database.clone();
    println!(
        "Starting home caching task. Every {} seconds.",
        CACHE_HOME_FREQUENCY.as_secs()
    );
    cache_home_task(database);
}
fn cache_home_task(database: Database) {
    thread::spawn(move || {
        let arc = Arc::from(database);
        let mut last_updated = arc
            .get_home(&get_date_string())
            .unwrap_or(None)
            .unwrap_or(Home::new())
            .last_updated;

        loop {
            if get_timestamp() - last_updated >= CACHE_HOME_FREQUENCY_NUM {
                println!("Caching home.");
                let new_home = arc.cache_home().unwrap().1;

                fn get_ids(animes: Vec<AnimeRelease>) -> Vec<String> {
                    animes
                        .into_iter()
                        .filter_map(|anime| anime.id) // Filters out None and extracts Some(id)
                        .collect()
                }
                cache_animes(get_ids(new_home.schedule), Arc::clone(&arc));
                cache_animes(get_ids(new_home.recent), Arc::clone(&arc));

                last_updated = new_home.last_updated;
                println!("Done caching home.");
            }

            thread::sleep(CACHE_HOME_FREQUENCY);
        }
    });
}

fn cache_animes(anime_ids: Vec<String>, database: Arc<Database>) {
    let pool = ThreadPool::new(anime_ids.len() / 2 + 1);

    for anime_id in anime_ids {
        let db_clone = Arc::clone(&database);

        pool.execute(move || {
            db_clone.cache_anime(&anime_id, IdType::KartofPlay).unwrap();
            println!("Done {}", &anime_id);
        });
    }

    pool.join();
}
