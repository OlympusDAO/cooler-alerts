mod discord;
mod registry;
mod listener;

use sqlx;
use tokio;
use std::env;
use std::error::Error;
use std::sync::Arc;

use lettre::transport::smtp::authentication::Credentials;
use serenity::framework::standard::StandardFramework;
use serenity::model::prelude::*;
use serenity::prelude::*;
use ethers::{
    providers::{Http, Provider},
    types::Address,
};

use discord::{GENERAL_GROUP, Bot};

struct Config {
    provider: Arc<Provider<Http>>,
    database: sqlx::SqlitePool,
    email_creds: Credentials,
}

impl Config {
    pub async fn new(database: sqlx::SqlitePool) -> Self {
        // Configure RPC provider
        let network = std::env::var("NETWORK_RPC").expect("missing NETWORK_RPC");
        let provider: Arc<Provider<Http>> = Arc::new(Provider::try_from(network).expect("invalid NETWORK_RPC"));

        // Configure email credentials
        let email_usr = std::env::var("EMAIL_USER").expect("missing EMAIL_USER");
        let email_pwd = std::env::var("EMAIL_PASSWORD").expect("missing EMAIL_PASSWORD");
        let email_creds = Credentials::new(email_usr, email_pwd);

        Self { provider, database, email_creds }
    }

    pub fn get_db(&self) -> &sqlx::SqlitePool {
        &self.database
    }

    pub fn get_provider(&self) -> Arc<Provider<Http>> {
        self.provider.clone()
    }

    pub fn get_email_creds(&self) -> Credentials {
        self.email_creds.clone()
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load env variables
    dotenv::dotenv().ok();
    let bot_token = env::var("DISCORD_TOKEN").expect("Expected a token in the .env file");
    let monitoring_address: Address = "0xA00F4b7c57a4995796D6E2ae4A6D5dEc8a557367".parse().unwrap();

    // Initiate a connection to the database file, creating the file if required.
    let database = sqlx::sqlite::SqlitePoolOptions::new()
    .max_connections(5)
    .connect_with(sqlx::sqlite::SqliteConnectOptions::new()
            .filename("cooler-alerts.sqlite")
            .create_if_missing(true),
    )
    .await
    .expect("Couldn't connect to database");

    // Update DB schema to the latest version.
    sqlx::migrate!("./migrations").run(&database).await.expect("Couldn't run database migrations");

    // Initialize the required system configuration.
    let config: Config = Config::new(database.clone()).await;

    // New thread to monitor the chain.
    tokio::spawn(async move {
        listener::monitor(monitoring_address, config.get_provider(), config.get_db(), config.get_email_creds()).await;
    });

    // Configure and initialize the Discord bot to manage alerts.
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);
    let bot = Bot::new(database.clone());

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    
    let mut discord_client =
        Client::builder(&bot_token, intents)
            .event_handler(bot)
            .framework(framework)
            .await.expect("Err creating client");

    // Start listening for discord events
    if let Err(why) = discord_client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }

    Ok(())
}
