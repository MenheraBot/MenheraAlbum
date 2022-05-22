use actix_web::{
    get,
    web, App, HttpResponse, HttpServer, Result,
};

#[get("/{source}/{name}")]
async fn index(path: web::Path<(String, String)>) -> Result<HttpResponse> {
    let (action, file) = path.into_inner();
    let extension = if action == "fodase" || action == "humor" {"png"} else {"gif"};
    let f = async_std::fs::read(format!("./assets/{}/{}.{}", action, file, extension)).await?;
    Ok(HttpResponse::Ok().content_type(format!("image/{}", extension)).body(f))
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
