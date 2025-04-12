use governor::Jitter;
use reqwest::header::USER_AGENT;
use serde::{
    de::{self},
    Deserialize, Serialize,
};
use std::sync::Arc;
use std::{
    env::{self, VarError},
    fmt,
    time::Duration,
};
use strum::{EnumIter, EnumString};

use crate::RiotRatelimiters;

#[derive(Debug, EnumString, Serialize, EnumIter)]
pub enum Region {
    Br1,
    Eun1,
    Euw1,
    Jp1,
    Kr,
    La1,
    La2,
    Na1,
    Oc1,
    Tr1,
    Ru,
    Ph2,
    Sg2,
    Th2,
    Tw2,
    Vn2,
}
impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, EnumString, Serialize)]
pub enum LargeRegion {
    Americas,
    Asia,
    Europe,
    Sea,
}
impl fmt::Display for LargeRegion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RiotApiError {
    #[error("Enviromental variable RIOT_API_KEY")]
    EnviromentalVariableError(#[from] VarError),
    #[error("Riot api error")]
    RiotApiError(#[from] reqwest::Error),
}

async fn riot_request<T: de::DeserializeOwned>(
    reqwest_client: Arc<reqwest::Client>,
    riot_ratelimiters: RiotRatelimiters,
    request_url: &str,
) -> Result<T, reqwest::Error> {
    while riot_ratelimiters.long_ratelimit.check().is_err() {
        riot_ratelimiters
            .long_ratelimit
            .until_ready_with_jitter(Jitter::new(Duration::from_secs(1), Duration::from_secs(1)))
            .await;
    }
    while riot_ratelimiters.short_ratelimit.check().is_err() {
        riot_ratelimiters
            .short_ratelimit
            .until_ready_with_jitter(Jitter::new(Duration::from_secs(1), Duration::from_secs(1)))
            .await;
    }
    return reqwest_client
        .get(request_url)
        .header(USER_AGENT, "rust-web-api-client") // gh api requires a user-agent header
        .send()
        .await?
        .json()
        .await;
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct AccountV1 {
    pub puuid: String,
    pub game_name: String,
    pub tag_line: String,
}

pub async fn account_v1(
    reqwest_client: Arc<reqwest::Client>,
    riot_ratelimiters: RiotRatelimiters,
    large_region: &LargeRegion,
    gamename: &str,
    tagline: &str,
) -> Result<AccountV1, RiotApiError> {
    let _riot_api_key = env::var("RIOT_API_KEY")?;
    let request_url = format!(
        "https://{}.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}?api_key={}",
        large_region, gamename, tagline, _riot_api_key
    );
    //println!("account_v1 request_url: {}", request_url);
    let account_v1 =
        riot_request::<AccountV1>(reqwest_client, riot_ratelimiters, &request_url).await;

    //println!("{:?}", account_v1);
    Ok(account_v1?)
}

//pub async fn champion_mastery_v4_puuid(puuid: &str) {}

//pub async fn champion_v3() {}

//pub enum Queue {
//    RankedSolo5x5,
//    RankedTft,
//    RankedFlexSr,
//    RankedFlexTt,
//}

//pub struct Rank {
//    tier: Tier,
//    divison: Division,
//}
//
//pub enum Tier {
//    CHALLENGER,
//    GRANDMASTER,
//    MASTER,
//    DIAMOND,
//    EMERALD,
//    PLATINUM,
//    GOLD,
//    SILVER,
//    BRONZE,
//    IRON,
//}
//
//pub enum Division {
//    I,
//    II,
//    III,
//    IV,
//}

//pub async fn league_exp_v4(queue: &Queue, rank: &Rank) {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LeagueV4 {
    pub league_id: String,
    pub queue_type: String,
    pub tier: String,
    pub rank: String,
    pub summoner_id: String,
    pub puuid: String,
    pub league_points: i32,
    pub wins: i32,
    pub losses: i32,
    pub veteran: bool,
    pub inactive: bool,
    pub fresh_blood: bool,
    pub hot_streak: bool,
}

pub async fn league_v4(
    reqwest_client: Arc<reqwest::Client>,
    riot_ratelimiters: RiotRatelimiters,
    region: &Region,
    puuid: &str,
) -> Result<Vec<LeagueV4>, RiotApiError> {
    let _riot_api_key = env::var("RIOT_API_KEY")?;
    let request_url = format!(
        "https://{}.api.riotgames.com/lol/league/v4/entries/by-puuid/{}?api_key={}",
        region, puuid, _riot_api_key
    );
    //println!("account_v1 request_url: {}", request_url);
    let league_v4s =
        riot_request::<Vec<LeagueV4>>(reqwest_client, riot_ratelimiters, &request_url).await;

    //println!("{:?}", league_v4s);
    Ok(league_v4s?)
}

//pub async fn league_v4_challengerleagues(queue: &Queue) {}

//pub async fn league_v4_masterleagues(queue: &Queue) {}

//pub async fn lol_status_v4() {}

pub async fn match_v5_matchlist(
    reqwest_client: Arc<reqwest::Client>,
    riot_ratelimiters: RiotRatelimiters,
    large_region: &LargeRegion,
    puuid: &str,
) -> Result<Vec<String>, RiotApiError> {
    let _riot_api_key = env::var("RIOT_API_KEY")?;
    let request_url = format!(
        "https://{}.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids?start=0&count=5&api_key={}",
        large_region, puuid, _riot_api_key
    );
    //println!("account_v1 request_url: {}", request_url);
    let match_v5_matchlist =
        riot_request::<Vec<String>>(reqwest_client, riot_ratelimiters, &request_url).await;

    //println!("{:?}", match_v5_matchlist);
    Ok(match_v5_matchlist?)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchV5Match {
    pub metadata: MetadataDto,
    pub info: InfoDto,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataDto {
    pub data_version: String,
    pub match_id: String,
    pub participants: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoDto {
    pub end_of_game_result: String,
    pub game_creation: i64,
    pub game_duration: i64,
    pub game_end_timestamp: Option<i64>,
    pub game_id: i64,
    pub game_mode: String,
    pub game_name: String,
    pub game_start_timestamp: i64,
    pub game_type: String,
    pub game_version: String,
    pub map_id: i32,
    pub participants: Vec<ParticipantDto>,
    pub platform_id: String,
    pub queue_id: i32,
    pub teams: Vec<TeamDto>,
    pub tournament_code: Option<String>,
}

// ChallengesDto is ignored
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantDto {
    pub all_in_pings: i32,
    pub assist_me_pings: i32,
    pub assists: i32,
    pub baron_kills: i32,
    pub bounty_level: i32,
    pub champ_experience: i32,
    pub champ_level: i32,
    pub champion_id: i32,
    pub champion_name: String,
    pub command_pings: i32,
    pub champion_transform: i32,
    pub consumables_purchased: i32,
    pub damage_dealt_to_buildings: i32,
    pub damage_dealt_to_objectives: i32,
    pub damage_dealt_to_turrets: i32,
    pub damage_self_mitigated: i32,
    pub deaths: i32,
    pub detector_wards_placed: i32,
    pub double_kills: i32,
    pub dragon_kills: i32,
    pub eligible_for_progression: bool,
    pub enemy_missing_pings: i32,
    pub enemy_vision_pings: i32,
    pub first_blood_assist: bool,
    pub first_blood_kill: bool,
    pub first_tower_assist: bool,
    pub first_tower_kill: bool,
    pub game_ended_in_early_surrender: bool,
    pub game_ended_in_surrender: bool,
    pub hold_pings: i32,
    pub get_back_pings: i32,
    pub gold_earned: i32,
    pub gold_spent: i32,
    pub individual_position: String,
    pub inhibitor_kills: i32,
    pub inhibitor_takedowns: i32,
    pub inhibitors_lost: i32,
    pub item0: i32,
    pub item1: i32,
    pub item2: i32,
    pub item3: i32,
    pub item4: i32,
    pub item5: i32,
    pub item6: i32,
    pub items_purchased: i32,
    pub killing_sprees: i32,
    pub kills: i32,
    pub lane: String,
    pub largest_critical_strike: i32,
    pub largest_killing_spree: i32,
    pub largest_multi_kill: i32,
    pub longest_time_spent_living: i32,
    pub magic_damage_dealt: i32,
    pub magic_damage_dealt_to_champions: i32,
    pub magic_damage_taken: i32,
    pub neutral_minions_killed: i32,
    pub need_vision_pings: i32,
    pub nexus_kills: i32,
    pub nexus_takedowns: i32,
    pub nexus_lost: i32,
    pub objectives_stolen: i32,
    pub objectives_stolen_assists: i32,
    pub on_my_way_pings: i32,
    pub participant_id: i32,
    //pub player_score0: i32,
    //pub player_score1: i32,
    //pub player_score2: i32,
    //pub player_score3: i32,
    //pub player_score4: i32,
    //pub player_score5: i32,
    //pub player_score6: i32,
    //pub player_score7: i32,
    //pub player_score8: i32,
    //pub player_score9: i32,
    //pub player_score10: i32,
    //pub player_score11: i32,
    pub penta_kills: i32,
    pub perks: PerksDto,
    pub physical_damage_dealt: i32,
    pub physical_damage_dealt_to_champions: i32,
    pub physical_damage_taken: i32,
    pub placement: i32,
    pub player_augment1: i32,
    pub player_augment2: i32,
    pub player_augment3: i32,
    pub player_augment4: i32,
    pub player_subteam_id: i32,
    pub push_pings: i32,
    pub profile_icon: i32,
    pub puuid: String,
    pub quadra_kills: i32,
    pub riot_id_game_name: String,
    pub riot_id_tagline: String,
    pub role: String,
    pub sight_wards_bought_in_game: i32,
    pub spell1_casts: i32,
    pub spell2_casts: i32,
    pub spell3_casts: i32,
    pub spell4_casts: i32,
    pub subteam_placement: i32,
    pub summoner1_casts: i32,
    pub summoner1_id: i32,
    pub summoner2_casts: i32,
    pub summoner2_id: i32,
    pub summoner_id: String,
    pub summoner_level: i32,
    pub summoner_name: String,
    pub team_early_surrendered: bool,
    pub team_id: i32,
    pub team_position: String,
    //pub time_ccing_others: i32,
    pub time_played: i32,
    pub total_ally_jungle_minions_killed: i32,
    pub total_damage_dealt: i32,
    pub total_damage_dealt_to_champions: i32,
    pub total_damage_shielded_on_teammates: i32,
    pub total_damage_taken: i32,
    pub total_enemy_jungle_minions_killed: i32,
    pub total_heal: i32,
    pub total_heals_on_teammates: i32,
    pub total_minions_killed: i32,
    //pub total_time_cc_dealt: i32,
    pub total_time_spent_dead: i32,
    pub total_units_healed: i32,
    pub triple_kills: i32,
    pub true_damage_dealt: i32,
    pub true_damage_dealt_to_champions: i32,
    pub true_damage_taken: i32,
    pub turret_kills: i32,
    pub turret_takedowns: i32,
    pub turrets_lost: i32,
    pub unreal_kills: i32,
    pub vision_score: i32,
    pub vision_cleared_pings: i32,
    pub vision_wards_bought_in_game: i32,
    pub wards_killed: i32,
    pub wards_placed: i32,
    pub win: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerksDto {
    pub stat_perks: PerkStatsDto,
    pub styles: Vec<PerkStyleDto>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerkStatsDto {
    pub defense: i32,
    pub flex: i32,
    pub offense: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerkStyleDto {
    pub description: String,
    pub selections: Vec<PerkStyleSelectionDto>,
    pub style: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerkStyleSelectionDto {
    pub perk: i32,
    pub var1: i32,
    pub var2: i32,
    pub var3: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamDto {
    pub bans: Vec<BanDto>,
    pub objectives: ObjectivesDto,
    pub team_id: i32,
    pub win: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BanDto {
    pub champion_id: i32,
    pub pick_turn: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectivesDto {
    pub baron: ObjectiveDto,
    pub champion: ObjectiveDto,
    pub dragon: ObjectiveDto,
    pub horde: ObjectiveDto,
    pub inhibitor: ObjectiveDto,
    pub rift_herald: ObjectiveDto,
    pub tower: ObjectiveDto,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveDto {
    pub first: bool,
    pub kills: i32,
}

pub async fn match_v5_match(
    reqwest_client: Arc<reqwest::Client>,
    riot_ratelimiters: RiotRatelimiters,
    large_region: &LargeRegion,
    match_id: &str,
) -> Result<MatchV5Match, RiotApiError> {
    let _riot_api_key = env::var("RIOT_API_KEY")?;
    let request_url = format!(
        "https://{}.api.riotgames.com/lol/match/v5/matches/{}?api_key={}",
        large_region, match_id, _riot_api_key
    );
    //println!("account_v1 request_url: {}", request_url);
    let match_v5_match =
        riot_request::<MatchV5Match>(reqwest_client, riot_ratelimiters, &request_url).await;

    //println!("{:?}", match_v5_match);
    Ok(match_v5_match?)
}

//pub async fn match_v5_timeline(match_id: &str) {}

//pub async fn spectator_v5(puuid: &str) {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct SummonerV4 {
    pub account_id: String,
    pub profile_icon_id: u32,
    pub revision_date: u64,
    pub id: String,
    pub puuid: String,
    pub summoner_level: u64,
}
pub async fn summoner_v4(
    reqwest_client: Arc<reqwest::Client>,
    riot_ratelimiters: RiotRatelimiters,
    region: &Region,
    puuid: &str,
) -> Result<SummonerV4, RiotApiError> {
    let _riot_api_key = env::var("RIOT_API_KEY")?;
    let request_url = format!(
        "https://{}.api.riotgames.com/lol/summoner/v4/summoners/by-puuid/{}?api_key={}",
        region, puuid, _riot_api_key
    );
    //println!("summoner_v4 request_url: {}", request_url);
    let summoner_v4 =
        riot_request::<SummonerV4>(reqwest_client, riot_ratelimiters, &request_url).await;

    //println!("{:?}", summoner_v4);
    Ok(summoner_v4?)
}

//pub async fn summoner_v4(summoner_id: &str) {}
