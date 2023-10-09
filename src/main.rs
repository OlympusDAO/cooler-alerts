mod hypernative;
mod discord;

use tokio;
use std::env;
use serenity::framework::standard::{StandardFramework, CommandResult};

use serenity::model::prelude::*;
use serenity::prelude::*;
use hypernative::{AccessToken, create_custom_agent, delete_custom_agent};
use discord::{GENERAL_GROUP, Bot};
use sqlx;


#[tokio::main]
async fn main() {
    // Load environment variables from .env file
   dotenv::dotenv().ok();
   let mut access_token = AccessToken::new(24 * 3600);// Configure the client with your Discord bot token in the environment.
   let bot_token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

   let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

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

   let bot = Bot::new(database);

   let intents = GatewayIntents::GUILD_MESSAGES
      | GatewayIntents::DIRECT_MESSAGES
      | GatewayIntents::MESSAGE_CONTENT;
   let mut client =
      Client::builder(&bot_token, intents)
         .event_handler(bot)
         .framework(framework)
         .await.expect("Err creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }

   // let input = vec![
   //    String::from("0x6f40DF8cC60F52125467838D15f9080748c2baea"),
   //    0.to_string()
   // ];
   // let threshold: u32 = 10062912;
   // let discord = "https://discord.com/api/webhooks/1234/slack".to_string();
   // let email: Option<String> = None; 

   // let response = create_custom_agent(&mut access_token, input, threshold, email, Some(discord)).await?;
   // match response {
   //    Some(data) => {
   //       println!("Agent {:#?} has been created", &data.agent_id());
   //       delete_custom_agent(&mut access_token, &data.agent_id()).await?
   //    }
   //    None => {
   //       println!("Alert already exists!");
   //    }
   // }
}
