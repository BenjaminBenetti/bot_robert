use mongodb::{Client, Database, Collection};
use mongodb::options::ClientOptions;
use std::error::Error;
use crate::error::DatabaseError;

static DB_NAME: &str = "bot_robert_db";

pub struct DatabaseConnector {
    client: Option<Client>,
    database: Option<Database>,
    initialized: bool,
}

impl DatabaseConnector {
    pub fn new() -> DatabaseConnector {
        DatabaseConnector {
            client: None,
            database: None,
            initialized: false,
        }
    }

    /// connect to the database.
    pub async fn initialize_connection(&mut self, user_name: &str, password: &str) -> Result<(), Box<dyn Error>> {
        // db connection string cannot be in static because format! macro only works with string literals :(
        let mut client_options = ClientOptions::parse(&format!("mongodb://{}:{}@mongodb:27017", user_name, password)).await?;
        client_options.app_name = Some(String::from("bot_robert"));

        self.client = match Client::with_options(client_options) {
            Ok(client) => Some(client),
            Err(_) => None
        };

        if let Some(client) = &self.client {
            self.database = Some(client.database(DB_NAME));
        }

        self.initialized = self.database.is_some() && self.client.is_some();

        return if self.initialized {
            Ok(())
        }
        else {
            Err(Box::new(DatabaseError::new("Failed to connect ot database")))
        }
    }

    /// get a collection from the database
    pub fn get_collection(&self, name: &str) -> Result<Collection, Box<dyn Error>> {
        if let Some(database) = &self.database {
            Ok(database.collection(name))
        }
        else {
            Err(Box::new(DatabaseError::new(&format!("Failed to fetch collection [{}] from the database.\
             Error database not initialized.", name))))
        }
    }
}