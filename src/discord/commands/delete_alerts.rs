use serenity::builder::{
    CreateApplicationCommand,
    CreateEmbed,
};
use serenity::model::prelude::{
    command::CommandOptionType,
    application_command::CommandDataOption,
};
use sqlx;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use crate::hypernative::{AccessToken, delete_custom_agent};

#[derive(Clone)]
struct AlertDB {
    agent_id: i64,
    user_id: i64,
    cooler: String,
    loan_id: i64,
    threshold: i64,
    webhook_url: Option<String>,
    email: Option<String>,
}

pub async fn run(database: &sqlx::SqlitePool, access_token: &Arc<Mutex<AccessToken>>, user_id: i64, options: &[CommandDataOption]) -> CreateEmbed {
    let access_token = access_token.lock().await;
    
    let cooler = match &options.get(0).expect("Expected cooler").value {
        Some(cooler) => match cooler.as_str() {
            Some(cooler) => cooler,
            None => panic!("Expected cooler"),
        },
        None => panic!("Expected cooler"),    
    };

    let alerts: Vec<AlertDB>;
    let deleted_loan_id: Option<i64>;
    match &options.get(1) {
        Some(loan_id) => {
            let loan_id = match &loan_id.value {
                Some(loan_id) => {
                    deleted_loan_id = loan_id.as_i64();
                    loan_id
                },
                None => panic!("Expected loan_id"),
            };
            alerts = sqlx::query_as!(AlertDB, "
            SELECT * FROM alerts
            WHERE user_id = ? AND cooler = ? AND loan_id = ?
            ", user_id, cooler, loan_id)
            .fetch_all(database)
            .await
            .unwrap();
        },
        None => {
            deleted_loan_id = None;
            alerts = sqlx::query_as!(AlertDB, "
            SELECT * FROM alerts
            WHERE user_id = ? AND cooler = ?
            ", user_id, cooler)
            .fetch_all(database)
            .await
            .unwrap();
        }
    };

    let alerts = Arc::new(alerts);
    if (alerts.len() == 0){
        return CreateEmbed::default()
        .title("No alerts found")
        .description(format!("Cooler Contract: [{cooler}](https://www.etherscan.io/address/{cooler})"))
        .field("", "", false)
        .footer(|f| f.text("Remember that you can check your current alerts by using the slash command /list_alerts."))
        .color(0xDB4B4B)
        .to_owned();
    }
    // Create a channel to collect the results of async calls.
    let (tx, mut rx) = mpsc::channel(alerts.len());
    // Iterate through alerts and spawn async tasks.
    for i in 0..alerts.len() {
        let mut access_token = access_token.clone();
        let alert = alerts[i].clone();

        let tx = tx.clone();
        tokio::spawn(async move {
            let result = delete_custom_agent(&mut access_token, &alert.agent_id).await;
            tx.send(result).await.expect("Failed to send result");
        });
    }

    // Collect and await the results.
    for _ in 0..alerts.len() {
        let result = rx.recv().await.expect("Failed to receive result");
        match result {
            Ok(_) => {},
            Err(_) => {
                return CreateEmbed::default()
                .title("Something went wrong!")
                .description("Error when trying to delete the alert in the Hypernative platform.")
                .color(0xDB4B4B)
                .to_owned();}
        }
    }

    match deleted_loan_id {
        Some(loan_id) => {
            // Delete the alert from the DB.
            sqlx::query!("
            DELETE FROM alerts
            WHERE user_id = ? AND cooler = ? AND loan_id = ?
            ", user_id, cooler, loan_id)
            .fetch_all(database)
            .await
            .unwrap();

            return CreateEmbed::default()
            .title("All Alerts Deleted")
            .description(format!("Cooler Contract: [{cooler}](https://www.etherscan.io/address/{cooler})"))
            .field("Loan ID", loan_id, false)
            .field("", "", false)
            .footer(|f| f.text("Remember that you can check your current alerts by using the slash command /list_alerts."))
            .color(0xDB4B4B)
            .to_owned();
        },
        None => {
            // Delete the alert from the DB.
            sqlx::query!("
            DELETE FROM alerts
            WHERE user_id = ? AND cooler = ?
            ", user_id, cooler)
            .fetch_all(database)
            .await
            .unwrap();

            return CreateEmbed::default()
            .title("All Alerts Deleted")
            .description(format!("Cooler Contract: [{cooler}](https://www.etherscan.io/address/{cooler})"))
            .field("", "", false)
            .footer(|f| f.text("Remember that you can check your current alerts by using the slash command /list_alerts."))
            .color(0xDB4B4B)
            .to_owned();
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("delete_alerts").description("Delete all existing alerts for a given loan.")
        .create_option(|option| {
            option
                .name("cooler")
                .description("The address of the Cooler contract. Must starts with `0x`.")
                .kind(CommandOptionType::String)
                .required(true)
                .min_length(42) // enforce length of EVM address.
                .max_length(42) // enforce length of EVM address.
        })
        .create_option(|option| {
            option
                .name("loan_id")
                .description("If no ID is informed, all alerts for the given Cooler will be deleted.")
                .kind(CommandOptionType::Integer)
                .required(false)
        })
}