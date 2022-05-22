use actix_web::{
    get, web,
    App, HttpResponse, HttpServer, Result, Responder
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
    kiss: u32,
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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
