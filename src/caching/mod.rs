use std::{ rc::Rc, sync::Arc, thread, time::Duration };

use chrono::Utc;
use threadpool::ThreadPool;

use crate::{
    scrapers,
    utils::{
        caching_info::CachingInfo,
        get_date_string,
        get_timestamp,
        mongodb::Database,
        types::{ AnimeRelease, Home, IdType },
    },
    SETTINGS,
};

pub mod cache_anime;

pub mod cache_home;

pub fn start(database: Database) {
    let database_clone = database.clone();
    let database_clone2 = database.clone();

    println!(
        "Starting home caching task. Every {} seconds.",
        SETTINGS.CACHE_HOME_FREQUENCY.as_secs()
    );
    cache_home_task(database);
    println!(
        "Starting all animes caching task. Every {} days.",
        SETTINGS.CACHE_ALL_ANIME_FREQUENCY.as_secs() / (24 * 60 * 60)
    );
    update_all_animes_task(database_clone);

    println!(
        "Statring all images caching task. Every {} days.",
        SETTINGS.CACHE_ALL_IMAGES_FREQUENCY.as_secs() / (24 * 60 * 60)
    );
    cache_all_images_task(database_clone2);
}
fn cache_all_images_task(database: Database) {
    thread::spawn(move || {
        loop {
            if
                get_timestamp() - CachingInfo::get_all_images_time() >=
                (SETTINGS.CACHE_ALL_IMAGES_FREQUENCY.as_secs() as i64)
            {
                CachingInfo::update_time();
                println!("Started caching all images!");
                database.cache_all_images().unwrap();
                println!("Done caching all images!");
            }
            thread::sleep(Duration::from_secs(100));
        }
    });
}
fn update_all_animes_task(database: Database) {
    thread::spawn(move || {
        let arc = Arc::from(database);

        loop {
            if
                get_timestamp() - CachingInfo::get_all_anime_time() >=
                (SETTINGS.CACHE_ALL_ANIME_FREQUENCY.as_secs() as i64)
            {
                CachingInfo::update_time();
                update_all_animes(&arc);
            }

            thread::sleep(Duration::from_secs(100));
        }
    });
}

fn cache_home_task(database: Database) {
    thread::spawn(move || {
        let arc = Arc::from(database);
        let mut last_updated = arc
            .get_home(&get_date_string())
            .unwrap_or(None)
            .unwrap_or(Home::new()).last_updated;

        loop {
            if get_timestamp() - last_updated >= SETTINGS.CACHE_HOME_FREQUENCY_NUM {
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

            thread::sleep(Duration::from_secs(100));
        }
    });
}

fn update_all_animes(database: &Arc<Database>) {
    let po = ThreadPool::new(SETTINGS.UPDATE_ALL_ANIME_THREADS);
    let arc: Arc<Database> = Arc::clone(database);
    for i in 0..100 {
        let arc_clone = arc.clone();
        let page1 = scrapers::gogoanime::anime_list::get(&i.to_string()).unwrap_or_default();
        println!("Caching all anime {}%", i);

        let mut current_anime = 0;

        for anime in page1 {
            let clone = arc_clone.clone();

            po.execute(move || {
                clone.cache_anime(&anime, IdType::Gogoanime).unwrap();
            });
            current_anime += 1;
            if current_anime >= SETTINGS.UPDATE_ALL_ANIME_THREADS {
                po.join();
                thread::sleep(SETTINGS.CACHE_SLEEP);
            }
        }
        po.join();
    }
}

fn cache_animes(anime_ids: Vec<String>, database: Arc<Database>) {
    let pool = ThreadPool::new(anime_ids.len() / 2 + 1);

    for anime_id in anime_ids {
        let db_clone = Arc::clone(&database);

        pool.execute(move || {
            db_clone.cache_anime(&anime_id, IdType::KartofPlay).unwrap();
        });
    }

    pool.join();
}
