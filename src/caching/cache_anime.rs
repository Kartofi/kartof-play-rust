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
    pub fn get(id: &str) -> mongodb::error::Result<bool> {
        Ok(true)
    }
}
