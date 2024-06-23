use std::env;
use std::io::Read;
use std::fs::File;
use actix_web::{Responder, HttpRequest, HttpResponse, post, web};
use actix_http::body::to_bytes;
use uuid::Uuid;
use chrono::prelude::*;
use serde::Deserialize;

mod simsim;

#[allow(dead_code)]
#[derive(Deserialize)]
struct Config {
    rule1: String,
    exception1: String,
    rule2: String,
    rule3: String,
    rule4: String,
    rule5: String,
    rule6: String,
    resp: String,
    url: String,
    api_key: String,
}

pub async fn runrule(linput: String) -> String {
    let file = File::open("./pregate-rules.toml");
    let mut contents = String::new();
    let _ = file.expect("failed to read ./pregate-rules.toml").read_to_string(&mut contents);
    let config: Config = toml::from_str(&contents).unwrap();

    match linput.to_lowercase().as_str() {
        s if s.contains(&config.exception1) => {
            match linput {
                s if s.contains(&config.rule2) => config.resp,
                s if s.contains(&config.rule3) => config.resp,
                s if s.contains(&config.rule4) => config.resp,
                s if s.contains(&config.rule5) => config.resp,
                s if s.contains(&config.rule6) => config.resp,
                _ => {
                    let callb = ifetch(config.url, config.api_key, linput).await.unwrap_or_else(|_| "Failed to fetch from API".to_string()).to_string();
                    callb
                }
            }
        }
        s if s.contains(&config.rule1) => config.resp,
        s if s.contains(&config.rule2) => config.resp,
        s if s.contains(&config.rule3) => config.resp,
        s if s.contains(&config.rule4) => config.resp,
        s if s.contains(&config.rule5) => config.resp,
        s if s.contains(&config.rule6) => config.resp,
        _s if simsim::checkit(&config.rule1, &linput).contains("match") => config.resp,
        _ => {
            let callb = ifetch(config.url, config.api_key, linput).await.unwrap_or_else(|_| "Failed to fetch from API".to_string()).to_string();
            callb
        }
    }
}

#[allow(unused)]
#[post("/api/SOMETHING/v1")]
pub async fn readit(req: HttpRequest, body: web::Bytes) -> impl Responder {
    let txid = Uuid::new_v4().to_string();
    env::set_var("txid", &txid);
    let peer = req.peer_addr();
    let requ = req.headers();
    let readi: DateTime<Utc> = Utc::now();
    log::info!("{} - {} - /api/SOMETHING/v1 POST - from {:?} - {:?}", readi, &txid, peer, &requ);
    let bbod = to_bytes(body).await.unwrap();
    let sbod: Result<String, _> = match std::str::from_utf8(&bbod) {
        Ok(_string) => {
            let seasnails = std::str::from_utf8(&bbod).unwrap().to_string();
            Ok(seasnails)
        },
        _ => {
            let seasnails = "ERROR: non-utf8 data received.".to_string();
            Err(seasnails)
        }
    };

    let mut return_me = String::new();
    match sbod {
        Err(_) => return_me = "ERROR: non-utf8 data received.".to_string(),
        _ => return_me = sbod.unwrap(),
    }

    let rbod = return_me;
    let nid = env::var("txid").unwrap();

    let returnbod: String = runrule(rbod).await;
    if returnbod == "Failed to fetch from API" {
        let reada: DateTime<Utc> = Utc::now();
        log::error!("{} - {} - /api/SOMETHING/v1 BACKEND DOWN sending response for: {:?}", reada, &nid, requ);
    } else {
        let reada: DateTime<Utc> = Utc::now();
        log::info!("{} - {} - /api/SOMETHING/v1 sending response for:  {:?}", reada, &nid, requ);
    }
    HttpResponse::Ok().body(returnbod)
}

pub async fn ifetch(url: String, key: String, sendbod: String) -> Result<String, reqwest::Error> {
    let nid = env::var("txid").unwrap();
    let timed = Utc::now();
    log::info!("{} - {} - Fetching {:?}...", timed, &nid, url);
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", key)
        .body(sendbod)
        .send()
        .await?;
    let timeo = Utc::now();
    log::info!("{} - {} - Response: {:?} {}", &timeo, &nid, res.version(), res.status());
    let body = res.text().await;
    let backbod = body?.to_string();
    let timeb = Utc::now();
    log::info!("{} - {} - sending response", timeb, &nid);
    Ok(backbod)
}
