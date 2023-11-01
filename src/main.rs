use actix_web::http::header::ContentType;
use actix_web::{get, web, App, HttpResponse, HttpServer };

use actix_files::Files;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
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

fn get_categories() -> HashMap<String, usize> {
    let mut assets: HashMap<String, usize> = HashMap::new();
    let paths = fs::read_dir("assets/").unwrap();

    for dir in paths {
        let path = dir.unwrap().path();
        if path.is_dir() {
            let dir_name = (&path.file_name()).unwrap().to_str().unwrap();

            let images = fs::read_dir(&path.to_str().unwrap()).unwrap();
            let count = images.count();

            assets.insert(dir_name.to_string(), count);
        }
    }

    return assets;
}

#[get("/")]
async fn index() -> HttpResponse {
    let categories = get_categories();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(categories)
}

#[get("/ping")]
async fn ping(data: web::Data<Uptime>) -> HttpResponse {
    let obj = Ping {
        uptime: data.start_at.elapsed().as_millis(),
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(obj)
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
