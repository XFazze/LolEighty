use actix_web::{get, post, web, web::Redirect, App, HttpResponse, HttpServer, Responder};
use riot_api::{LargeRegion, LeagueV4, MatchV5Match, Region};
use strum::IntoEnumIterator;
use std::{env, str::FromStr};
extern crate dotenv;
use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;
use tera::Tera;
use std::sync::Arc;
use governor::{ DefaultDirectRateLimiter, Quota, RateLimiter};
use nonzero_ext::nonzero;

mod riot_api;
pub use riot_api::{AccountV1, SummonerV4};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "src/templates/**/*";
        let tera = Tera::new(source).unwrap();
        tera
    };
}
#[derive(Clone)]
pub struct RiotRatelimiters {
    short_ratelimit: Arc<DefaultDirectRateLimiter>,
    long_ratelimit:  Arc<DefaultDirectRateLimiter>

}
pub struct AppState {
    reqwest_client: Arc<reqwest::Client>,
    riot_ratelimiters:RiotRatelimiters
}

#[get("/")]
async fn index() -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("user", "Me moi");
    let mut regions = Vec::new();
    for r in Region::iter(){
        regions.push(format!("{:?}", r));
    };
    context.insert("regions", &regions );
    match TEMPLATES.render("index.html", &context){
        Ok(page_contents) =>{
            HttpResponse::Ok().body(page_contents)
        }
        Err(e)=>{ 
            println!("{:?}",e);
            HttpResponse::NotFound().finish()
        }
    }
}
#[derive(Deserialize)]
struct UserForm {
    region: String,
    full_name: String,
}
#[post("/user_lookup")]
async fn user_loopup(web::Form(form): web::Form<UserForm>) -> impl Responder {
    //println!("user_lookup: {} - {}", form.region, form.full_name);
    let full_name_split: Vec<&str> = form.full_name.split("#").collect();
    //assert_eq!(full_name_split.len(), 2);
    let name = full_name_split[0];
    let tag = full_name_split[1];
    Redirect::to(format!("/user/{}/{}/{}", form.region, name, tag)).see_other()
}

#[get("/user/{region}/{name}/{tag}")]
async fn user(path: web::Path<(String, String, String)>, data: web::Data<AppState>) -> impl Responder {
    let (region_as_str, name, tag) = path.into_inner();
    //println!("user: {} - {} - {}", region_as_str, name, tag);
    let region;
    match Region::from_str(&region_as_str) {
        Ok(success) => {
            region = success
        }
        Err(_) =>{
            let mut context = tera::Context::new();
            context.insert("error_message", "Region doesnt exists");
            let page_contents = TEMPLATES.render("error.html", &context).unwrap();
            return HttpResponse::Ok().body(page_contents);

        }
    }
    let large_region = match region {
        Region::Br1 => LargeRegion::Americas,
        Region::Eun1 => LargeRegion::Europe,
        Region::Euw1 => LargeRegion::Europe,
        Region::Jp1 => LargeRegion::Asia,
        Region::Kr => LargeRegion::Asia,
        Region::La1 => LargeRegion::Americas,
        Region::La2 => LargeRegion::Americas,
        Region::Na1 => LargeRegion::Americas,
        Region::Oc1 => LargeRegion::Sea,
        Region::Tr1 => LargeRegion::Europe,
        Region::Ru => LargeRegion::Europe,
        Region::Ph2 => LargeRegion::Sea,
        Region::Sg2 => LargeRegion::Sea,
        Region::Th2 => LargeRegion::Sea,
        Region::Tw2 => LargeRegion::Asia,
        Region::Vn2 => LargeRegion::Asia,
    };
    println!("user: {} - {} - {} - {}", large_region, region, name, tag);


    let account_v1: AccountV1;
    match riot_api::account_v1(data.reqwest_client.clone(), data.riot_ratelimiters.clone(),&large_region, &name, &tag).await {
        Ok(success) => {
            account_v1 = success;
        }
        Err(_err)=>{
            let mut context = tera::Context::new();
            context.insert("error_message", "Riot won't answer");
            let page_contents = TEMPLATES.render("error.html", &context).unwrap();
            return HttpResponse::Ok().body(page_contents);
        }
    }

    let summoner_v4: SummonerV4;
    match riot_api::summoner_v4(data.reqwest_client.clone(), data.riot_ratelimiters.clone(), &region,&account_v1.puuid).await {
        Ok(success) => {
            summoner_v4 = success;
        }
        Err(_err)=>{
            let mut context = tera::Context::new();
            context.insert("error_message", "Riot is confusing");
            let page_contents = TEMPLATES.render("error.html", &context).unwrap();
            return HttpResponse::Ok().body(page_contents);
        }
    }
    
    let league_v4s: Vec<LeagueV4>;
    match riot_api::league_v4(data.reqwest_client.clone(), data.riot_ratelimiters.clone(), &region, &account_v1.puuid).await {
        Ok(success) => {
            league_v4s = success;
        }
        Err(_err)=>{
            let mut context = tera::Context::new();
            context.insert("error_message", "Riot is confusing");
            let page_contents = TEMPLATES.render("error.html", &context).unwrap();
            return HttpResponse::Ok().body(page_contents);
        }
    }

    let matches: Vec<String>;
    match riot_api::match_v5_matchlist(data.reqwest_client.clone(), data.riot_ratelimiters.clone(), &large_region, &account_v1.puuid).await {
        Ok(success) => {
            matches = success;
        }
        Err(_err)=>{
            let mut context = tera::Context::new();
            context.insert("error_message", "Riot is confusing");
            let page_contents = TEMPLATES.render("error.html", &context).unwrap();
            return HttpResponse::Ok().body(page_contents);
        }
    }

    let mut context = tera::Context::new();
    context.insert("region", &region);
    context.insert("large_region", &large_region);
    context.insert("name", &name);
    context.insert("tag", &tag);
    context.insert("profile_icon_id", &summoner_v4.profile_icon_id);
    context.insert("lvl", &summoner_v4.summoner_level);
    context.insert("league_v4s", &league_v4s);
    context.insert("matches", &matches);
    

    match TEMPLATES.render("user.html", &context){
        Ok(page_contents) =>{
            HttpResponse::Ok().body(page_contents)
        }
        Err(e)=>{ 
            println!("{:?}",e);
            HttpResponse::NotFound().finish()
        }
    }
}


#[get("/match/{large_region}/{match_id}")]
async fn lol_match(path: web::Path<(String, String)>, data: web::Data<AppState>) -> impl Responder {
    let (large_region_as_str, match_id) = path.into_inner();
    let large_region; 
    match LargeRegion::from_str(&large_region_as_str) {
        Ok(success) => {
            large_region = success
        }
        Err(_) =>{
            let mut context = tera::Context::new();
            context.insert("error_message", "Large region doesnt exists");
            let page_contents = TEMPLATES.render("error.html", &context).unwrap();
            return HttpResponse::Ok().body(page_contents);

        }
    }
    println!("{:?}", match_id);


    let lol_match: MatchV5Match;
    match riot_api::match_v5_match(data.reqwest_client.clone(), data.riot_ratelimiters.clone(), &large_region, &match_id).await {
        Ok(success) => {
            lol_match = success;
        }
        Err(_err)=>{
            let mut context = tera::Context::new();
            context.insert("error_message", "Riot is confusing");
            let page_contents = TEMPLATES.render("error.html", &context).unwrap();
            return HttpResponse::Ok().body(page_contents);
        }
    }

    let mut context = tera::Context::new();
    context.insert("match_id", &match_id);
    context.insert("lol_match", &lol_match);
    

    match TEMPLATES.render("match.html", &context){
        Ok(page_contents) =>{
             HttpResponse::Ok().body(page_contents)
        }
        Err(e)=>{ 
            println!("{:?}",e);
             HttpResponse::NotFound().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env.secret").ok();
    dotenv().ok();
    let _riot_api_key = env::var("RIOT_API_KEY").expect("RIOT_API_KEY not set in .env");
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                reqwest_client: Arc::new(reqwest::Client::new()),
                riot_ratelimiters:RiotRatelimiters{
                    short_ratelimit: Arc::new(RateLimiter::direct(Quota::per_second(nonzero!(20u32)))),
                    long_ratelimit: Arc::new(RateLimiter::direct(Quota::per_minute(nonzero!(120u32))))
                }
            }))
            .service(
                actix_files::Files::new("/static", "./static")
                    //.use_last_modified(true)
                    .show_files_listing(),
            )
            .service(index)
            .service(user_loopup)
            .service(user)
            .service(lol_match)
        })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
