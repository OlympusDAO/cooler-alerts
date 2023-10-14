pub mod commands;

use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::builder::CreateEmbed;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::framework::standard::macros::{command, group};
use serenity::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::hypernative::AccessToken;
use sqlx;

#[group]
pub struct General;

pub struct Bot {
    database: sqlx::SqlitePool,
    access_token: Arc<Mutex<AccessToken>>,
}

impl Bot {
    pub fn new(database: sqlx::SqlitePool, access_token: AccessToken) -> Self {
        Self { database, access_token: Arc::new(Mutex::new(access_token)) }
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let user_id = command.user.id.0 as i64;

            let embed = match command.data.name.as_str() {
                "create_alert" => commands::create_alert::run(&self.database, &self.access_token, user_id, &command.data.options).await,
                "list_alerts" => commands::list_alerts::run(&self.database, user_id).await,
                "delete_alerts" => commands::delete_alerts::run(&self.database, user_id, &command.data.options).await,
                _ => CreateEmbed::default().title("not implemented :(").to_owned(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.add_embed(embed))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let bot_commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    commands::create_alert::register(command)
                })
                .create_application_command(|command| {
                    commands::list_alerts::register(command)
                })
                .create_application_command(|command| {
                    commands::delete_alerts::register(command)
                })
        }).await;

        println!("I created the following global slash command: {:#?}", bot_commands);
    }
}