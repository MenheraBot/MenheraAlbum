use actix_web::{
    get, web,
    App, HttpServer, Result, Responder
};
use serde::Serialize;
use actix_files::Files;
use std::time::Instant;


#[derive(Serialize)]
struct Ping {
    uptime: u128,
}

#[derive(Clone)]
struct Uptime {
    start_at: std::time::Instant,
}

impl Uptime {
    fn new() -> Self {
        Uptime {
            start_at: Instant::now(),
        }
    }
}

#[derive(Serialize)]
struct AvailableAssets {
    angry: u8,
    bicuda: u8,
    bite: u8,
    cheek: u8,
    cry: u8,
    disgusted: u8,
    fear: u8,
    fodase: u8,
    grumble: u8,
    hug: u8,
    humor: u8,
    kill: u8,
    kiss: u8,
    laugh: u8,
    mamar: u8,
    pat: u8,
    poke: u8,
    punch: u8,
    resurrect: u8,
    sarrar: u8,
    sarrar_sozinho: u8,
    shot: u8,
    shy: u8,
    slap: u8,
    sniff: u8,
    think: u8,
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    let obj = AvailableAssets {
        angry: 39,
        bicuda: 34,
        bite: 33,
        cheek: 23,
        cry: 26,
        disgusted: 17,
        fear: 21,
        fodase: 37,
        grumble: 18,
        hug: 26,
        humor: 50,
        kill: 23,
        kiss: 20,
        laugh: 23,
        mamar: 23,
        pat: 26,
        poke: 24,
        punch: 22,
        resurrect: 7,
        sarrar: 5,
        sarrar_sozinho: 2,
        shot: 14,
        shy: 43,
        slap: 36,
        sniff: 22,
        think: 31,
    };
    Ok(web::Json(obj))
}

#[get("/ping")]
async fn ping(data: web::Data<Uptime>) -> Result<impl Responder> {
    let obj = Ping {
        uptime: data.start_at.elapsed().as_millis()
    };

    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started at port 8080");

    let uptime = Uptime::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(uptime.clone()))
            .service(index)
            .service(ping)
            .service(Files::new("/images", "assets"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
