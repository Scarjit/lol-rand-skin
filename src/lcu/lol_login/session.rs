use serde::{Serialize,Deserialize};
use league_client_connector::RiotLockFile;
use crate::lcu::lcu_get_request;

pub async fn get(rlf: &RiotLockFile) -> Option<RiotLoginSession>{
    let rs = lcu_get_request(rlf, "/lol-login/v1/session").await;
    if rs.status().is_success() {
        let rs_text = rs.text().await.unwrap();
        return Some(serde_json::from_str::<RiotLoginSession>(&rs_text).unwrap())
    }else{
        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RiotLoginSession {
    #[serde(rename = "accountId")]
    account_id: i64,
    connected: bool,
    error: Option<serde_json::Value>,
    #[serde(rename = "gasToken")]
    gas_token: Option<serde_json::Value>,
    #[serde(rename = "idToken")]
    id_token: String,
    #[serde(rename = "isInLoginQueue")]
    is_in_login_queue: bool,
    #[serde(rename = "isNewPlayer")]
    is_new_player: bool,
    puuid: String,
    state: String,
    #[serde(rename = "summonerId")]
    pub summoner_id: i64,
    #[serde(rename = "userAuthToken")]
    user_auth_token: String,
    username: String,
}
