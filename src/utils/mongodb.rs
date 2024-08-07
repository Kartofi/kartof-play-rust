use mongodb::{
    bson::{self, doc, Regex},
    options::IndexOptions,
    sync::{Client, Collection, Cursor},
    IndexModel,
};
use serde::{Deserialize, Serialize};

use super::{get_timestamp, types::*};
#[derive(Debug, Clone)]
pub struct Database {
    client: Client,
}
impl Database {
    pub fn new() -> mongodb::error::Result<Database> {
        let mongodb_conn = std::env::var("MONGODB").expect("MONGODB must be set.");
        let uri = &mongodb_conn;
        // Create a MongoDB client
        let client = Client::with_uri_str(uri)?;
        Ok(Database { client: client })
    }
    //Home
    pub fn update_home(
        &self,
        mut data: Home,
        search: Option<Home>,
    ) -> mongodb::error::Result<CacheResult> {
        let database = self.client.database("Kartof-Play");

        let col: Collection<Home> = database.collection("Home");

        if search.is_none() {
            data.last_updated = get_timestamp();
            col.insert_one(data, None)?;
        } else {
            let filter = doc! {"date": &data.date};

            let mut update = doc! {};

            let popular = bson::to_bson(&data.popular);
            if popular.is_ok() {
                update.insert("popular", popular.unwrap());
            }

            let recent = bson::to_bson(&data.recent);
            if recent.is_ok() {
                update.insert("recent", recent.unwrap());
            }

            let schedule = bson::to_bson(&data.schedule);
            if schedule.is_ok() {
                update.insert("schedule", schedule.unwrap());
            }

            update.insert("last_updated", get_timestamp());
            col.update_one(filter, doc! { "$set": update}, None)?;
        }
        Ok(CacheResult::new("", false))
    }
    pub fn get_home(&self, date: &str) -> mongodb::error::Result<Option<Home>> {
        let database = self.client.database("Kartof-Play");

        let col: Collection<Home> = database.collection("Home");

        let filter = doc! {"date": date};

        let cursor = col.find(filter, None)?;
        let mut cursor_peekable = cursor.peekable();

        if cursor_peekable.peek().is_none() {
            return Ok(None);
        }

        Ok(Some(cursor_peekable.next().unwrap()?))
    }
    //Animes
    pub fn search_anime(
        &self,
        title: &str,
        max_results: usize,
    ) -> mongodb::error::Result<Vec<Anime>> {
        let database = self.client.database("Kartof-Play");

        let col: Collection<Anime> = database.collection("Animes");

        let title = &title.replace("+", " ");

        let filter = doc! {
            "$or": [
                { "title": Regex { pattern: "(?i)".to_string() +title, options: "i".to_string() } },
                { "details.other_names": { "$regex": Regex { pattern: "(?i)".to_string() +title, options: "i".to_string() } } }
            ]
        };

        // Search for Anime documents matching the filter
        let cursor = col.find(filter, None)?;

        // Iterate over the results and print each document
        let mut results: Vec<Anime> = Vec::new();

        for result in cursor {
            match result {
                Ok(anime) => {
                    results.push(anime);
                    if results.len() >= max_results {
                        break;
                    }
                }
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
        Ok(results)
    }
    pub fn get_anime_id(&self, id: &str, id_type: &IdType) -> Option<Anime> {
        if id_type == &IdType::KartofPlay {
            self.get_anime(id, "id")
        } else if id_type == &IdType::AnimeGG {
            self.get_anime(id, "animegg_id")
        } else if id_type == &IdType::Gogoanime {
            self.get_anime(id, "gogo_id")
        } else if id_type == &IdType::AnimeSchedule {
            self.get_anime(id, "schedule_id")
        } else if id_type == &IdType::MAL {
            self.get_anime(id, "mal_id")
        } else {
            None
        }
    }

    fn get_anime(&self, id: &str, name_id: &str) -> Option<Anime> {
        let database = self.client.database("Kartof-Play");

        let col: Collection<Anime> = database.collection("Animes");

        let filter = doc! {name_id: id };

        // Search for Anime documents matching the filter
        let cursor: Cursor<Anime> = col.find(filter, None).unwrap();
        for result in cursor {
            match result {
                Ok(anime) => return Some(anime),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
        None
    }
    pub fn insert_new_anime(&self, anime: Anime) -> mongodb::error::Result<CacheResult> {
        let database = self.client.database("Kartof-Play");

        let col: Collection<Anime> = database.collection("Animes");

        col.insert_one(anime, None).unwrap();
        Ok(CacheResult::new("No errors", false))
    }
    pub fn update_anime(
        &self,
        id: &str,
        details: Option<AnimeDetails>,
        episodes: Option<Vec<Episode>>,
        animegg_id: Option<&str>,
        mal_id: Option<&str>,
        schedule_id: Option<&str>,
    ) -> mongodb::error::Result<CacheResult> {
        let database = self.client.database("Kartof-Play");

        let col: Collection<Anime> = database.collection("Animes");

        let filter = doc! { "id": id };

        let mut update_doc = doc! {};
        if let Some(details) = details {
            let res = bson::to_bson(&details);
            if res.is_ok() {
                update_doc.insert("details", res.unwrap());
            }
        }

        if let Some(episodes) = episodes {
            let res = bson::to_bson(&episodes);
            if res.is_ok() {
                update_doc.insert("episodes", res.unwrap());
            }
        }
        if let Some(animegg_id) = animegg_id {
            update_doc.insert("animegg_id", animegg_id);
        }
        if let Some(mal_id) = mal_id {
            update_doc.insert("mal_id", mal_id);
        }
        if let Some(schedule_id) = schedule_id {
            update_doc.insert("schedule_id", schedule_id);
        }
        update_doc.insert("last_updated", crate::utils::get_timestamp());

        let update = doc! { "$set": update_doc };
        col.update_one(filter, update, None).unwrap();
        Ok(CacheResult::new("No errors", false))
    }
}
