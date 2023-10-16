mod discord;
// mod blockchain_scanner;
mod hypernative;

use tokio;
use std::env;
use serenity::framework::standard::{StandardFramework, CommandResult};

use serenity::model::prelude::*;
use serenity::prelude::*;
use hypernative::AccessToken;
use discord::{
    GENERAL_GROUP,
    Bot,
    // alerts::send_alert,
};
use sqlx;

use std::error::Error;
// use std::{sync::Arc, time::Duration};
// use ethers::{
//     prelude::abigen,
//     providers::{Http, Provider},
//     contract::Contract,
//     types::Address,
// };

// abigen!(
//     IUniswapV2Pair,
//     "[function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)]"
// );

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load env variables
   dotenv::dotenv().ok();
   let mut access_token = AccessToken::new(24 * 3600);
   let bot_token = env::var("DISCORD_TOKEN").expect("Expected a token in the .env file");

   // Initiate a connection to the database file, creating the file if required.
   let database = sqlx::sqlite::SqlitePoolOptions::new()
   .max_connections(5)
   .connect_with(
       sqlx::sqlite::SqliteConnectOptions::new()
           .filename("cooler-alerts.sqlite")
           .create_if_missing(true),
   )
   .await
   .expect("Couldn't connect to database");

   // Run migrations, which updates the database's schema to the latest version.
   sqlx::migrate!("./migrations").run(&database).await.expect("Couldn't run database migrations");

    // Configure and initialize the Discord Bot to manage alerts.
   let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);
   let bot = Bot::new(database, access_token);

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

    // // Replace with the contract address and ABI of the contract you want to interact with
    // let contract_address: Address = "CONTRACT_ADDRESS".parse().unwrap();
 
    // let network = std::env::var("NETWORK_RPC").expect("missing NETWORK_RPC");
    // let provider: Arc<Provider<Http>> = Arc::new(Provider::try_from(network).expect("invalid NETWORK_RPC"));

    //  // Thread for checking what block we're on.
    //  tokio::spawn(async move {
    //     blockchain_scanner::monitor(contract_address, Arc::clone(&provider)).await;
    // });

    Ok(())
}
