use std::os::windows::thread;

use mongodb::{
    bson::{self, doc, Regex},
    options::IndexOptions,
    sync::{Client, Collection, Cursor},
    IndexModel,
};
use serde::{Deserialize, Serialize};

use super::types::*;
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

    pub fn search_anime(
        &self,
        title: &str,
        max_results: usize,
    ) -> mongodb::error::Result<Vec<Anime>> {
        let database = self.client.database("Kartof-Play");

        let col: Collection<Anime> = database.collection("Animes");

        let filter =
            doc! { "title": Regex { pattern: title.to_string(), options: "i".to_string() } };

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
    pub fn get_anime_id(&self, id: &str) -> Option<Anime> {
        let database = self.client.database("Kartof-Play");

        let col: Collection<Anime> = database.collection("Animes");

        let filter = doc! { "id": id };

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
    pub fn get_anime_mal_id(&self, id: &str) -> Option<Anime> {
        let database = self.client.database("Kartof-Play");

        let col: Collection<Anime> = database.collection("Animes");

        let filter = doc! { "mal_id": id };

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
    pub fn get_anime_animegg_id(&self, id: &str) -> Option<Anime> {
        let database = self.client.database("Kartof-Play");

        let col: Collection<Anime> = database.collection("Animes");

        let filter = doc! { "animegg_id": id };

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
    pub fn get_anime_schedule_id(&self, id: &str) -> Option<Anime> {
        let database = self.client.database("Kartof-Play");

        let col: Collection<Anime> = database.collection("Animes");

        let filter = doc! { "schedule_id": id };

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

    pub fn insert_new_anime(&self, anime: Anime) -> mongodb::error::Result<bool> {
        let database = self.client.database("Kartof-Play");

        let col: Collection<Anime> = database.collection("Animes");

        col.insert_one(anime, None).unwrap();
        Ok(true)
    }
    pub fn update_anime(
        &self,
        id: &str,
        details: Option<AnimeDetails>,
        episodes: Option<Vec<Episode>>,
    ) -> mongodb::error::Result<bool> {
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

        if update_doc.is_empty() {
            return Ok(false); // Nothing to update
        }
        update_doc.insert("last_updated", crate::utils::get_timestamp());
        
        let update = doc! { "$set": update_doc };
        col.update_one(filter, update, None).unwrap();
        Ok(true)
    }
}
