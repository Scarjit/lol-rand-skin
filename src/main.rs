use league_client_connector::{RiotLockFile};
use log::LevelFilter;
use sysinfo::{RefreshKind, SystemExt, ProcessExt};
use std::thread;
use std::time::Duration;
use crate::lcu::{lol_champ_select, lol_login};
use rand::seq::SliceRandom;
use crate::lcu::lol_champ_select::skin_carousel_skins::{RiotSkinCarouselElement};
use crate::lcu::lol_champ_select::session::my_selection::MySelection;

pub mod lcu;

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Error)
        .with_module_level("lol_rand_skin", LevelFilter::Trace)
        .init()
        .unwrap();

    loop {

        await_lol();

        let lockfile = match league_client_connector::LeagueClientConnector::parse_lockfile(){
            Ok(v) => {v}
            Err(e) => {
                log::error!("{:?}", e);
                return
            }
        };
        log::debug!("{:?}", &lockfile);

        await_login(&lockfile).await;
        await_lobby(&lockfile).await;

        await_phase(&lockfile,"FINALIZATION").await;
        apply_random_skin(&lockfile).await;


        thread::sleep(Duration::from_secs(10));
    }
}

#[derive(Debug)]
struct Skin{
    id: i64,
    name: String,
    color: Vec<String>
}

async fn apply_random_skin(rlf: &RiotLockFile){
    log::debug!("Getting skin list");
    let skins = lcu::lol_champ_select::skin_carousel_skins::get(rlf).await;
    match skins {
        None => {
            log::error!("Error retrieving skins");
            return;
        }
        Some(riot_skin_carousel) => {
            if riot_skin_carousel.is_empty(){
                log::error!("No skins for champ");
                return;
            }

            let ava = riot_skin_carousel.iter().filter_map(filter_valid_skin).collect::<Vec<&RiotSkinCarouselElement>>();
            let skin =  ava.choose(&mut rand::thread_rng()).unwrap();

            let mut skinx = Skin{
                id: skin.id.clone().unwrap(),
                name: skin.name.clone().unwrap(),
                color: vec![]
            };

            match &skin.child_skins{
                None => {
                }
                Some(ck) => {
                    let subskins = ck.iter().filter_map(filter_valid_skin).collect::<Vec<&RiotSkinCarouselElement>>();
                    if !subskins.is_empty() {
                        let avasub = subskins.choose(&mut rand::thread_rng()).unwrap();

                        skinx.id = avasub.id.clone().unwrap();
                        skinx.name = avasub.name.clone().unwrap();
                        skinx.color = avasub.colors.clone().unwrap();
                    }

                }
            }

            log::debug!("Setting skin:");
            log::info!("{:#?}", skinx);

            let res = lol_champ_select::session::my_selection::patch(rlf, &MySelection{
                selectedSkinId: skinx.id,
                ..Default::default()
            }).await;

            match res{
                Ok(_) => {}
                Err(e) => {
                    log::error!("Failed to set skin: {:?}", e);
                }
            }
        }
    }
}

fn filter_valid_skin(skin: &RiotSkinCarouselElement) -> Option<&RiotSkinCarouselElement> {
    let skin_name = skin.name.clone().unwrap();
    if skin.disabled.unwrap(){
        log::trace!("{:?} disabled", skin_name);
        None
    }else {
        match &skin.ownership {
            None => {
                None
            }
            Some(ow) => {
                if ow.owned.unwrap() || ow.free_to_play_reward.unwrap() {
                    Some(skin)
                }else{
                    match &ow.rental{
                        None => {
                            None
                        }
                        Some(owo) => {
                            if owo.rented.unwrap(){
                                Some(skin)
                            }else{
                                None
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn await_login(rlf: &RiotLockFile){
    log::debug!("Waiting for login");
    loop{
        match lol_login::session::get(rlf).await{
            None => {
                thread::sleep(Duration::from_secs(1))
            }
            Some(_) => {
                return;
            }
        }
    }
}

async fn await_phase(rlf: &RiotLockFile, phase: &str){
    log::debug!("Waiting until everybody has picked");
    loop {
        match lol_champ_select::session::get(rlf).await {
            None => {
                thread::sleep(Duration::from_secs(1));
            }
            Some(v) => {
                if v.timer.phase == phase {
                    return
                }
            }
        }
    }
}

async fn await_lobby(rlf: &RiotLockFile) {
    log::debug!("Waiting for game session");
    loop {
        match lol_champ_select::session::get(rlf).await {
            None => {
                thread::sleep(Duration::from_secs(1));
            }
            Some(_) => {
                log::info!("Found game session");
                return;
            }
        }
    }
}

fn await_lol() {
    let mut sys = sysinfo::System::new_with_specifics(
        RefreshKind::everything()
            .without_components()
            .without_components_list()
            .without_cpu()
            .without_disks()
            .with_disks()
            .without_disks_list()
            .without_memory()
            .without_networks()
            .without_networks_list()
            .without_users_list(),
    );

    loop {
        let lol_p = sys.get_process_by_name("LeagueClientUx.exe");
        if !lol_p.is_empty() {
            log::debug!("Found League client {}", lol_p.first().unwrap().pid());
            return
        }
        log::info!("Waiting for League client");
        thread::sleep(Duration::from_secs(1));
        sys.refresh_processes()
    }
}
