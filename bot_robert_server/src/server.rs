use actix_web::{web, App, HttpServer};
use crate::api;

pub async fn start_server(address: &str, port: &str) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(configure_app)
    })
    .bind(format!("{}:{}", address, port))?
    .run()
    .await
}

/// Configure the server. Put routes here. (probably better way to organize this)
fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.service(api::slack::command_ws::slash_command);
}

