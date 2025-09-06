use fume_core::user::GetFriendListResponse;

#[test]
fn get_friend_list_decode() {
    let content = std::fs::read_to_string("./tests/responses/get_friend_list.json").unwrap();
    let response: GetFriendListResponse = serde_json::from_str(&content).unwrap();
    println!("{:#?}", response);
}
