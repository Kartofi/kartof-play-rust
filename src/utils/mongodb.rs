use std::os::windows::thread;

use mongodb::{
    bson::{doc, Regex},
    options::IndexOptions,
    sync::{Client, Collection},
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
}

pub fn insert(anime: Anime, client: Client) -> mongodb::error::Result<()> {
    // List the names of the databases in that deployment
    let database = client.database("Kartof-Play");

    let col: Collection<Anime> = database.collection("Animes");
    let cursor = col.find(doc! {"episode": "12"}, None).unwrap();
    for result in cursor {
        println!("title: {}", result?.title);
    }
    let index_model = IndexModel::builder()
        .keys(doc! { "title": 1 }) // 1 for ascending order, -1 for descending
        .options(IndexOptions::builder().unique(false).build())
        .build();

    // Create the index
    col.create_index(index_model, None)?;

    let mut da: Anime = Anime::new();
    da.title = "Bleach".to_string();

    //col.insert_one(da, None).unwrap();
    let filter = doc! { "title": Regex { pattern: "naru".to_string(), options: "i".to_string() } };

    // Search for Anime documents matching the filter
    let cursor = col.find(filter, None)?;

    // Iterate over the results and print each document
    for result in cursor {
        match result {
            Ok(anime) => println!("{:?}", anime),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    client.shutdown();
    Ok(())
}
