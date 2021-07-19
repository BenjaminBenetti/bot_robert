mod api;
mod server;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::start_server("0.0.0.0", "8080").await
}
