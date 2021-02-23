use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Entry {
    pub name: String,
    pub weight: i32,
    pub min_depth: i32,
    pub max_depth: i32,
    pub scales_to_depth: bool,
}
