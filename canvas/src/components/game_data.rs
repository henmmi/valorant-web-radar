use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GameInfo {
    pub t_score: Vec<i32>,
    pub ct_score: Vec<i32>,
}

impl GameInfo {
    pub fn get_map_url(name: &str) -> String {
        "http://127.0.0.1:8080/images/".to_owned() + name + ".png"
    }
}
