mod database_connector;
use lazy_static::lazy_static;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref DATABASE_CONNECTION: Mutex<database_connector::DatabaseConnector> = {
        Mutex::new(database_connector::DatabaseConnector::new())
    };
}
