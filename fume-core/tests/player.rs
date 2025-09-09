use fume_core::{Response, player::get_steam_level::SteamLevel};

#[test]
fn get_steam_level_decode() {
    let content = std::fs::read_to_string("./tests/responses/get_steam_level.json").unwrap();
    let response: Response<SteamLevel> = serde_json::from_str(&content).unwrap();
    println!("{:#?}", response);
}
