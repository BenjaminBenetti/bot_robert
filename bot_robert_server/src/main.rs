use actix_web::{App, HttpServer};

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::test::test)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
