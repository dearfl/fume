use fume_core::{
    Response,
    user::{GetFriendListResponse, GetUserGroupListResponseInner},
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
