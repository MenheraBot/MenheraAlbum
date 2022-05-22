use actix_web::{
    get,
    web, App, HttpResponse, HttpServer, Result, Responder
};
use serde::Serialize;

#[derive(Serialize)]
pub struct AvailableAssets {
    angry: u32,
    bicuda: u32,
    bite: u32,
    cheek: u32,
    cry: u32,
    disgusted: u32,
    fear: u32,
    fodase: u32,
    grumble: u32,
    hug: u32,
    humor: u32,
    kill: u32,
    laugh: u32,
    mamar: u32,
    pat: u32,
    poke: u32,
    punch: u32,
    resurrect: u32,
    sarrar: u32,
    sarrar_sozinho: u32,
    shot: u32,
    shy: u32,
    slap: u32,
    sniff: u32,
    think: u32,
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    let obj = AvailableAssets {
        angry: 14,
        bicuda: 14,
        bite: 14,
        cheek: 17,
        cry: 25,
        disgusted: 17,
        fear: 20,
        fodase: 37,
        grumble: 17,
        hug: 26,
        humor: 50,
        kill: 23,
        laugh: 22,
        mamar: 21,
        pat: 26,
        poke: 24,
        punch: 21,
        resurrect: 5,
        sarrar: 5,
        sarrar_sozinho: 2,
        shot: 13,
        shy: 14,
        slap: 6,
        sniff: 4,
        think: 11,
    };
    Ok(web::Json(obj))
}

#[get("/{source}/{name}")]
async fn images(path: web::Path<(String, String)>) -> Result<HttpResponse> {
    let (action, file) = path.into_inner();
    let extension = if action == "fodase" || action == "humor" {"png"} else {"gif"};
    let f = async_std::fs::read(format!("./assets/{}/{}.{}", action, file, extension)).await?;
    Ok(HttpResponse::Ok().content_type(format!("image/{}", extension)).body(f))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started at port 8080");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(images)
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run()
    .await
}
