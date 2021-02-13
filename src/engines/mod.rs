pub mod mp3s;
pub mod myfreemp3;
use crate::types::{
    EngineTraits,
    MythraError,
    MockEngineTraits
};
use crate::utils::render_select_music;
use log::error;

pub fn get_engine(engine: &str) -> Result<Box<dyn EngineTraits>, MythraError>{
    match engine {
        "mp3s" => {
            Ok(Box::new(mp3s::MP3S{}))
        },
        "myfreemp3" => {
            Ok(Box::new(myfreemp3::MyFreeMP3{}))
        }
        engine_match => {
            error!("Engine {} is unsupported", engine_match);
            Err(MythraError::UnsupportedEngine)
        }
    }
}

pub async fn cli(engine_name: &str, query: &str) {
    let title: &str = &(format!("Searching {} for {}", engine_name, query))[..];
    let engine = get_engine(engine_name);
    match engine {
        Ok(actual) => {
            let results = actual.search(
                String::from(query)
                ).await.unwrap();
            render_select_music(results, title);
        },
        Err(_) => {
            error!("Error {} is unsupported", engine_name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[actix_rt::test]
    async fn test_cli_with_fake_engine_returns_not_found() {
        let mut mock = MockEngineTraits::new();
        mock.expect_search().times(0);
        cli("fake", "query").await;
    }

}
