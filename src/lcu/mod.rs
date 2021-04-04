use league_client_connector::RiotLockFile;
use reqwest::{Response};
use serde::Serialize;

pub mod lol_champ_select;
pub mod lol_login;

pub async fn lcu_get_request(rlf: &RiotLockFile, uri: &str) -> Response {
    let client = reqwest::ClientBuilder::new().danger_accept_invalid_certs(true).build().unwrap();
    return client.get(format!("{}://{}:{}{}", rlf.protocol, rlf.address, rlf.port, uri))
        .basic_auth(rlf.username.clone(), Some(rlf.password.clone()))
        .send()
        .await
        .unwrap();
}


pub async fn lcu_patch_request(rlf: &RiotLockFile, uri: &str, body: impl Serialize) -> Response {
    let client = reqwest::ClientBuilder::new().danger_accept_invalid_certs(true).build().unwrap();
    return client.patch(format!("{}://{}:{}{}", rlf.protocol, rlf.address, rlf.port, uri))
        .basic_auth(rlf.username.clone(), Some(rlf.password.clone()))
        .body(serde_json::to_string(&body).unwrap())
        .send()
        .await
        .unwrap();
}