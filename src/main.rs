use actix_web::{get, post, web, web::Redirect, App, HttpResponse, HttpServer, Responder};
use std::env;
extern crate dotenv;
use actix_files::NamedFile;
use dotenv::dotenv;
use lazy_static::lazy_static;
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use tera::Tera;
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "src/templates/**/*";
        let tera = Tera::new(source).unwrap();
        tera
    };
}

#[get("/")]
async fn index() -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("user", "Me moi");
    let page_contents = TEMPLATES.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(page_contents)
}

#[derive(Deserialize)]
struct UserForm {
    region: String,
    full_name: String,
}

#[post("/user_lookup")]
async fn user_loopup(web::Form(form): web::Form<UserForm>) -> impl Responder {
    println!("user_lookup: {} - {}", form.region, form.full_name);
    let full_name_split: Vec<&str> = form.full_name.split("#").collect();
    assert_eq!(full_name_split.len(), 2);
    let name = full_name_split[0];
    let tag = full_name_split[1];
    Redirect::to(format!("/user/{}/{}/{}", form.region, name, tag)).see_other()
}

#[derive(Deserialize, Debug)]
struct AccountV1 {
    puuid: String,
    gameName: String,
    tagLine: String,
}

#[derive(Deserialize, Debug)]
struct SummonerV4 {
    accountId: String,
    profileIconId: u32,
    revisionDate: u64,
    id: String,
    puuid: String,
    summonerLevel: u64,
}

#[get("/user/{region}/{name}/{tag}")]
async fn user(path: web::Path<(String, String, String)>) -> impl Responder {
    let (region, name, tag) = path.into_inner();
    println!("user: {} - {} - {}", region, name, tag);
    // /riot/account/v1/accounts/by-riot-id/{gameName}/{tagLine}

    let _riot_api_key = env::var("RIOT_API_KEY").expect("RIOT_API_KEY not set in .env");
    let request_url = format!(
        "https://europe.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}?api_key={}",
        name, tag, _riot_api_key
    );
    println!("user request_url: {}", request_url);
    let client = reqwest::Client::new();
    let response = client
        .get(request_url)
        .header(USER_AGENT, "rust-web-api-client") // gh api requires a user-agent header
        .send()
        .await;
    let accountv1: AccountV1;
    match response {
        Ok(success) => {
            println!("Successful api call");
            accountv1 = success.json().await.expect("user riot api doesnt work");
        }
        Err(_error) => {
            println!("Unsuccessful api call");
            let mut context = tera::Context::new();
            context.insert("error_message", "riot api call is Unsuccessful");
            let page_contents = TEMPLATES.render("error.html", &context).unwrap();
            return HttpResponse::Ok().body(page_contents);
        }
    }
    println!("{:?}", accountv1);

    let request_url = format!(
        "https://euw1.api.riotgames.com/lol/summoner/v4/summoners/by-puuid/{}?api_key={}",
        accountv1.puuid, _riot_api_key
    );
    let response = client
        .get(request_url)
        .header(USER_AGENT, "rust-web-api-client") // gh api requires a user-agent header
        .send()
        .await;

    let summonerv4: SummonerV4;
    match response {
        Ok(success) => {
            println!("Successful api call");
            summonerv4 = success.json().await.expect("user riot api doesnt work");
        }
        Err(_error) => {
            println!("Unsuccessful api call");
            let mut context = tera::Context::new();
            context.insert("error_message", "riot api call is Unsuccessful");
            let page_contents = TEMPLATES.render("error.html", &context).unwrap();
            return HttpResponse::Ok().body(page_contents);
        }
    }
    println!("{:?}", summonerv4);

    let mut context = tera::Context::new();
    context.insert("region", &region);
    context.insert("name", &name);
    context.insert("tag", &tag);
    context.insert("profile_icon_id", &summonerv4.profileIconId);
    let page_contents = TEMPLATES.render("user.html", &context).unwrap();
    HttpResponse::Ok().body(page_contents)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env.secret").ok();
    dotenv().ok();
    let _riot_api_key = env::var("RIOT_API_KEY").expect("RIOT_API_KEY not set in .env");
    HttpServer::new(|| {
        App::new()
            .service(
                actix_files::Files::new("/static", "./static")
                    //.use_last_modified(true)
                    .show_files_listing(),
            )
            .service(index)
            .service(user_loopup)
            .service(user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
