use mockall::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

// Result type.
pub type MythraResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum MythraError {
    UnsupportedEngine
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Music {
    // Option<> elements are struct elements
    // that are allowed to be None
    pub index: usize,
    pub artiste: Option<String>,
    pub title: String,
    pub collection: Option<String>,
    pub download_link: String,
    pub picture_link: Option<String>,
    pub size: Option<String>,
    pub duration: Option<String>,
    pub source: String,
}

// Enables borrow due to cursive cli
impl<'a> Borrow<Music> for &mut &Music {
    fn borrow(&self) -> &Music {
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Engine {
    pub name: &'static str,
    pub base_url: &'static str,
    pub search_url: &'static str,
}

#[automock]
#[async_trait]
pub trait EngineTraits {
    async fn search(&self, query: String) -> MythraResult<Vec<Music>>;
}

// Music request parser for API
#[derive(Deserialize)]
pub struct MusicRequest {
    pub query: String,
    pub engine: String,
}
// and mock!
mock! {
    pub Mock {}
    #[async_trait]
    trait EngineTraits {
        async fn search(&self, query: String) -> MythraResult<Vec<Music>>;
    }
}
