use league_client_connector::RiotLockFile;
use crate::lcu::lcu_get_request;
use serde::{Serialize,Deserialize};
pub mod my_selection;

pub async fn get(rlf: &RiotLockFile) -> Option<RiotChampSelectSession>{
    let rs = lcu_get_request(rlf, "/lol-champ-select/v1/session").await;
    if rs.status().is_success() {
        let rs_text = rs.text().await.unwrap();
        return Some(serde_json::from_str::<RiotChampSelectSession>(&rs_text).unwrap())
    }else{
        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RiotChampSelectSession {
    actions: Vec<Vec<Action>>,
    #[serde(rename = "allowBattleBoost")]
    allow_battle_boost: bool,
    #[serde(rename = "allowDuplicatePicks")]
    allow_duplicate_picks: bool,
    #[serde(rename = "allowLockedEvents")]
    allow_locked_events: bool,
    #[serde(rename = "allowRerolling")]
    allow_rerolling: bool,
    #[serde(rename = "allowSkinSelection")]
    allow_skin_selection: bool,
    bans: Bans,
    #[serde(rename = "benchChampionIds")]
    bench_champion_ids: Vec<Option<serde_json::Value>>,
    #[serde(rename = "benchEnabled")]
    bench_enabled: bool,
    #[serde(rename = "boostableSkinCount")]
    boostable_skin_count: i64,
    #[serde(rename = "chatDetails")]
    chat_details: ChatDetails,
    counter: i64,
    #[serde(rename = "entitledFeatureState")]
    entitled_feature_state: EntitledFeatureState,
    #[serde(rename = "gameId")]
    game_id: i64,
    #[serde(rename = "hasSimultaneousBans")]
    has_simultaneous_bans: bool,
    #[serde(rename = "hasSimultaneousPicks")]
    has_simultaneous_picks: bool,
    #[serde(rename = "isCustomGame")]
    is_custom_game: bool,
    #[serde(rename = "isSpectating")]
    is_spectating: bool,
    #[serde(rename = "localPlayerCellId")]
    local_player_cell_id: i64,
    #[serde(rename = "lockedEventIndex")]
    locked_event_index: i64,
    #[serde(rename = "myTeam")]
    my_team: Vec<Team>,
    #[serde(rename = "rerollsRemaining")]
    rerolls_remaining: i64,
    #[serde(rename = "skipChampionSelect")]
    skip_champion_select: bool,
    #[serde(rename = "theirTeam")]
    their_team: Vec<Team>,
    pub(crate) timer: Timer,
    trades: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "actorCellId")]
    actor_cell_id: i64,
    #[serde(rename = "championId")]
    champion_id: i64,
    completed: bool,
    id: i64,
    #[serde(rename = "isAllyAction")]
    is_ally_action: bool,
    #[serde(rename = "isInProgress")]
    is_in_progress: bool,
    #[serde(rename = "type")]
    action_type: Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bans {
    #[serde(rename = "myTeamBans")]
    my_team_bans: Vec<Option<serde_json::Value>>,
    #[serde(rename = "numBans")]
    num_bans: i64,
    #[serde(rename = "theirTeamBans")]
    their_team_bans: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatDetails {
    #[serde(rename = "chatRoomName")]
    chat_room_name: String,
    #[serde(rename = "chatRoomPassword")]
    chat_room_password: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntitledFeatureState {
    #[serde(rename = "additionalRerolls")]
    additional_rerolls: i64,
    #[serde(rename = "unlockedSkinIds")]
    unlocked_skin_ids: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    #[serde(rename = "assignedPosition")]
    assigned_position: String,
    #[serde(rename = "cellId")]
    cell_id: i64,
    #[serde(rename = "championId")]
    champion_id: i64,
    #[serde(rename = "championPickIntent")]
    champion_pick_intent: i64,
    #[serde(rename = "entitledFeatureType")]
    entitled_feature_type: EntitledFeatureType,
    #[serde(rename = "selectedSkinId")]
    selected_skin_id: i64,
    #[serde(rename = "spell1Id")]
    spell1_id: i64,
    #[serde(rename = "spell2Id")]
    spell2_id: i64,
    #[serde(rename = "summonerId")]
    summoner_id: i64,
    team: i64,
    #[serde(rename = "wardSkinId")]
    ward_skin_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Timer {
    #[serde(rename = "adjustedTimeLeftInPhase")]
    adjusted_time_left_in_phase: i64,
    #[serde(rename = "internalNowInEpochMs")]
    internal_now_in_epoch_ms: i64,
    #[serde(rename = "isInfinite")]
    is_infinite: bool,
    pub(crate) phase: String,
    #[serde(rename = "totalTimeInPhase")]
    total_time_in_phase: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ban")]
    Ban,
    #[serde(rename = "pick")]
    Pick,
    #[serde(rename = "ten_bans_reveal")]
    TenBansReveal,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EntitledFeatureType {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "NONE")]
    None,
}
