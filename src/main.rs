use actix_web::{
    get,
    web, App, HttpResponse, HttpServer, Result,
};

#[get("/{source}/{name}")]
async fn index(path: web::Path<(String, String)>) -> Result<HttpResponse> {
    let (action, file) = path.into_inner();
    let f = async_std::fs::read(format!("./assets/{}/{}.gif", action, file)).await?;
    Ok(HttpResponse::Ok().content_type("image/gif").body(f))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run()
    .await
}
