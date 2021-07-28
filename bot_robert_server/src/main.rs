use std::env;
use std::env::VarError;

mod api;
mod server;

static DB_PASSWORD_ENV: &str = "BOT_ROBERT_DB_PASSWORD";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pwd_opt: Result<String, VarError> = env::var(String::from(DB_PASSWORD_ENV));

    if let Ok(db_pwd) = db_pwd_opt {
        bot_robert_lib::database::DATABASE_CONNECTION.lock().await.initialize_connection("root", &db_pwd).await.unwrap();
        server::start_server("0.0.0.0", "8080").await
    }
    else {
        panic!("Database password wasn't set. Environment variable, {} should define it", DB_PASSWORD_ENV)
    }
}
