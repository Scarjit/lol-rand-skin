use league_client_connector::RiotLockFile;
use serde::Serialize;
use crate::lcu::lcu_patch_request;
use reqwest::Response;

#[allow(non_snake_case)]
#[derive(Debug,Serialize, Default)]
pub struct MySelection{
    pub(crate) selectedSkinId: i64,
    pub(crate) spell1Id: i64,
    pub(crate) spell2Id: i64,
    pub(crate) wardSkinId: i64,
}

pub async fn patch(rlf: &RiotLockFile, selection: &MySelection) -> reqwest::Result<Response> {
    let rs = lcu_patch_request(rlf, "/lol-champ-select/v1/session/my-selection",selection).await;
    rs.error_for_status()
}