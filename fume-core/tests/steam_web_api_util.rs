use fume_core::util::{GetServerInfoResponse, GetSupportedApiListResponse};

#[test]
fn get_supported_apis_decode() {
    let content = std::fs::read_to_string("./tests/responses/get_supported_apis.json").unwrap();
    let response: GetSupportedApiListResponse = serde_json::from_str(&content).unwrap();
    println!("{:#?}", response);
}

#[test]
fn get_server_info_decode() {
    let content = std::fs::read_to_string("./tests/responses/get_server_info.json").unwrap();
    let response: GetServerInfoResponse = serde_json::from_str(&content).unwrap();
    println!("{:#?}", response);
}
