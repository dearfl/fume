use fume_core::app::GetAppListResponse;

#[test]
fn get_app_list_decode() {
    let content = std::fs::read_to_string("./tests/responses/get_app_list.json").unwrap();
    let response: GetAppListResponse = serde_json::from_str(&content).unwrap();
    println!("{:#?}", response);
}
