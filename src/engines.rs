pub mod redthreemp3;
use crate::types::EngineTraits;

#[allow(dead_code)]
pub fn search_all() {
    let e = redthreemp3::RedThreeMP3;
    let engine = EngineTraits::search(&e, "test".to_string());
    println!("{:?}", engine)
}
