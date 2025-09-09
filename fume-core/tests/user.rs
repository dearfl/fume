use fume_core::{
    Response,
    user::{
        get_friend_list::GetFriendListResponse, get_player_summaries::PlayerSummaries,
        get_user_group_list::GetUserGroupListResponseInner,
        resolve_vanity_url::ResolveVanityUrlResponseInner,
    },
};

#[test]
fn get_friend_list_decode() {
    let content = std::fs::read_to_string("./tests/responses/get_friend_list.json").unwrap();
    let response: GetFriendListResponse = serde_json::from_str(&content).unwrap();
    println!("{:#?}", response);
}

#[test]
fn get_user_group_list_decode() {
    let content = std::fs::read_to_string("./tests/responses/get_user_group_list.json").unwrap();
    let response: Response<GetUserGroupListResponseInner> = serde_json::from_str(&content).unwrap();
    println!("{:#?}", response);
}

#[test]
fn resolve_vanity_url_success_decode() {
    let content =
        std::fs::read_to_string("./tests/responses/resolve_vanity_url_success.json").unwrap();
    let response: Response<ResolveVanityUrlResponseInner> = serde_json::from_str(&content).unwrap();
    println!("{:#?}", response);
}

#[test]
fn resolve_vanity_url_failure_decode() {
    let content =
        std::fs::read_to_string("./tests/responses/resolve_vanity_url_failure.json").unwrap();
    let response: Response<ResolveVanityUrlResponseInner> = serde_json::from_str(&content).unwrap();
    println!("{:#?}", response);
}

#[test]
fn get_player_summaries_decode() {
    let content = std::fs::read_to_string("./tests/responses/get_player_summaries.json").unwrap();
    let response: Response<PlayerSummaries> = serde_json::from_str(&content).unwrap();
    println!("{:#?}", response);
}
