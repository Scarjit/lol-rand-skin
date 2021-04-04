use league_client_connector::RiotLockFile;
use crate::lcu::lcu_get_request;
use serde::{Serialize,Deserialize};

pub async fn get(rlf: &RiotLockFile) -> Option<RiotSkinCarousel>{
    let rs = lcu_get_request(rlf, "/lol-champ-select/v1/skin-carousel-skins").await;
    if rs.status().is_success() {
        let rs_text = rs.text().await.unwrap();
        return Some(serde_json::from_str::<RiotSkinCarousel>(&rs_text).unwrap())
    }else{
        None
    }
}

pub type RiotSkinCarousel = Vec<RiotSkinCarouselElement>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RiotSkinCarouselElement {
    #[serde(rename = "championId")]
    pub champion_id: Option<i64>,
    #[serde(rename = "childSkins")]
    pub child_skins: Option<Vec<RiotSkinCarouselElement>>,
    #[serde(rename = "chromaPreviewPath")]
    pub chroma_preview_path: Option<String>,
    pub disabled: Option<bool>,
    pub emblems: Option<Vec<Option<serde_json::Value>>>,
    #[serde(rename = "groupSplash")]
    pub group_splash: Option<String>,
    pub id: Option<i64>,
    #[serde(rename = "isBase")]
    pub is_base: Option<bool>,
    #[serde(rename = "isChampionUnlocked")]
    pub is_champion_unlocked: Option<bool>,
    #[serde(rename = "isUnlockedFromEntitledFeature")]
    pub is_unlocked_from_entitled_feature: Option<bool>,
    pub name: Option<String>,
    pub ownership: Option<Ownership>,
    #[serde(rename = "rarityGemPath")]
    pub rarity_gem_path: Option<String>,
    #[serde(rename = "splashPath")]
    pub splash_path: Option<String>,
    #[serde(rename = "splashVideoPath")]
    pub splash_video_path: Option<String>,
    #[serde(rename = "stillObtainable")]
    pub still_obtainable: Option<bool>,
    #[serde(rename = "tilePath")]
    pub tile_path: Option<String>,
    pub unlocked: Option<bool>,
    pub colors: Option<Vec<String>>,
    #[serde(rename = "parentSkinId")]
    pub parent_skin_id: Option<i64>,
    #[serde(rename = "shortName")]
    pub short_name: Option<String>,
    pub stage: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ownership {
    #[serde(rename = "freeToPlayReward")]
    pub free_to_play_reward: Option<bool>,
    pub owned: Option<bool>,
    pub rental: Option<Rental>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rental {
    pub rented: Option<bool>,
}
