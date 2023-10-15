use serenity::builder::{
    CreateApplicationCommand,
    CreateEmbed,
};
use serenity::model::prelude::{
    command::CommandOptionType,
    application_command::CommandDataOption,
};
use serenity::model::webhook::Webhook;
use sqlx;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::hypernative::{AccessToken, create_custom_agent, delete_custom_agent};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("create_alert").description("Create a new alert")
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
                .description("The ID of the loan to be monitored.")
                .kind(CommandOptionType::Integer)
                .required(true)
                .min_int_value(0)
        })
        .create_option(|option| {
            option
                .name("threshold")
                .description("The days before expiration that the user should be notified in advance.")
                .kind(CommandOptionType::Integer)
                .required(true)
                .min_int_value(0)
        })
        .create_option(|option| {
            option
                .name("webhook_url")
                .description("The webhook URL where the alerts should be sent. Example: https://discord.com/api/webhooks/123/XXX")
                .kind(CommandOptionType::String)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("email")
                .description("The email address where the alerts should be sent. Example: cooler_alerts@yxz.com")
                .kind(CommandOptionType::String)
                .required(false)
        })
}

pub async fn run(database: &sqlx::SqlitePool, access_token: &Arc<Mutex<AccessToken>>, user_id: i64, options: &[CommandDataOption]) -> CreateEmbed {
    let mut access_token = access_token.lock().await;
    // Ensure user hasn't hit the limit of 3 alerts.
    let alerts = sqlx::query!("SELECT * FROM alerts WHERE user_id = ? ORDER BY rowid", user_id)
        .fetch_all(database)
        .await
        .unwrap();
    if (alerts.len() == 3) {
        return CreateEmbed::default()
            .title("Each user is limited to 3 alerts")
            .description("You can delete some alerts by using the slash command /delete_alerts.")
            .color(0xDB4B4B)
            .to_owned();
    }

    // Process the alert parameters.
    let cooler = match &options.get(0).expect("Expected cooler").value {
        Some(cooler) => match cooler.as_str() {
            Some(cooler) => cooler,
            None => panic!("Expected cooler"),
        },
        None => panic!("Expected cooler"),    
    };

    let loan_id = match &options.get(1).expect("Expected loan_id").value {
        Some(loan_id) => match loan_id.as_i64() {
            Some(loan_id) => loan_id,
            None => panic!("Expected loan_id"),
        },
        None => panic!("Expected loan_id"),
    };

    let threshold = match &options.get(2).expect("Expected threshold").value {
        Some(threshold) => match threshold.as_i64() {
            Some(threshold) => threshold,
            None => panic!("Expected threshold"),
        },
        None => panic!("Expected threshold"),
    };

    let mut email: Option<String> = None;
    let mut webhook_url: Option<String> = None;
    match &options.get(3) {
        Some(receiver) => {
            match &receiver.value {
                Some(receiver) => match receiver.as_str() {
                    Some(receiver) => {
                        if receiver.contains("https://") || receiver.contains("http://") {
                            if receiver.contains("discord") {
                                webhook_url = Some(format!("{receiver}/slack"));
                            } else {
                                webhook_url = Some(receiver.to_string());
                            }
                        } else if receiver.contains("@") {
                            email = Some(receiver.to_string());
                        } else {
                            return CreateEmbed::default()
                            .title("Something went wrong!")
                            .description("Invalid webhoook or email. Please try again with a valid input format.")
                            .color(0xDB4B4B)
                            .to_owned();
                        }
                    },
                    None => (),
                },
                None => (),
            };
        },
        None => return CreateEmbed::default()
        .title("Missing alert receiver!")
        .description("Please try again informing a webhook URL or an email.")
        .color(0xDB4B4B)
        .to_owned()
    };

    match &options.get(4) {
        Some(receiver) => {
            match &receiver.value {
                Some(receiver) => match receiver.as_str() {
                    Some(receiver) => {
                        if receiver.contains("https://") || receiver.contains("http://") {
                            if receiver.contains("discord") {
                                webhook_url = Some(format!("{receiver}/slack"));
                            } else {
                                webhook_url = Some(receiver.to_string());
                            }
                        } else if receiver.contains("@") {
                            email = Some(receiver.to_string());
                        } else {
                            return CreateEmbed::default()
                            .title("Something went wrong!")
                            .description("Invalid webhoook or email. Please try again with a valid input format.")
                            .color(0xDB4B4B)
                            .to_owned();
                        }
                    },
                    None => (),
                },
                None => (),
            };
        },
        None => ()
    };
    // Register the alert in the Hypernative platform.
    let response = create_custom_agent(&mut *access_token, vec![cooler.to_string(), loan_id.to_string()], threshold, email.clone(), webhook_url.clone()).await.unwrap();
    println!("{:#?}", response);
    let agent_id = match response {
        Some(custom_agent) => custom_agent.agent_id().clone(),
        None => return CreateEmbed::default()
            .title("Something went wrong!")
            .description("Error when trying to create the alert in the Hypernative platform.")
            .color(0xDB4B4B)
            .to_owned(),
    };


    println!("option3: {:#?}", &options.get(3));
    println!("option4: {:#?}", &options.get(4));

    println!("webhook_url: {:#?}", &webhook_url);
    println!("email: {:#?}", &email);
    // Store the alert in the database.
    let mut email_check = ":x:";
    let mut webhook_check = ":x:";
    match (webhook_url, email) {
        (Some(webhook_url), Some(email)) => {
            email_check = ":white_check_mark:";
            webhook_check = ":white_check_mark:";
            println!("Webhook: {:#?}, Email: {:#?}", webhook_url, email);
            sqlx::query!(
                "INSERT INTO alerts (user_id, agent_id, cooler, loan_id, threshold, webhook_url, email) VALUES (?, ?, ?, ?, ?, ?, ?)",
                user_id,
                agent_id,
                cooler,
                loan_id,
                threshold,
                webhook_url,
                email
            )
            .execute(database)
            .await
            .unwrap()
        }
        (Some(webhook_url), None) => {
            webhook_check = ":white_check_mark:";
            println!("Webhook: {:#?}", webhook_url);
            sqlx::query!(
                "INSERT INTO alerts (user_id, agent_id, cooler, loan_id, threshold, webhook_url) VALUES (?, ?, ?, ?, ?, ?)",
                user_id,
                agent_id,
                cooler,
                loan_id,
                threshold,
                webhook_url
            )
            .execute(database)
            .await
            .unwrap()
        }
        (None, Some(email)) => {
            email_check = ":white_check_mark:";
            println!("Email: {:#?}", email);
            sqlx::query!(
                "INSERT INTO alerts (user_id, agent_id, cooler, loan_id, threshold, email) VALUES (?, ?, ?, ?, ?, ?)",
                user_id,
                agent_id,
                cooler,
                loan_id,
                threshold,
                email
            )
            .execute(database)
            .await
            .unwrap()
        }
        (None, None) => {
            return CreateEmbed::default()
            .title("Something went wrong!")
            .description("Error when trying to create the alert in the Hypernative platform.")
            .color(0xDB4B4B)
            .to_owned();
        }
    };

    let alerts_after = sqlx::query!("SELECT * FROM alerts WHERE user_id = ?", user_id)
        .fetch_all(database)
        .await
        .unwrap();

    if (alerts_after.len() == alerts.len()) {
        delete_custom_agent(&mut access_token, &agent_id).await.unwrap();
        return CreateEmbed::default()
            .title("Something went wrong!")
            .description("Error when trying to register the alerts in the database.")
            .color(0xDB4B4B)
            .to_owned();
    }

    // Return the success embed.
    CreateEmbed::default()
        .title("Alert successfully added")
        .description(format!("Cooler Contract: [{cooler}](https://www.etherscan.io/address/{cooler})"))
        .field("Loan ID", loan_id, true)
        .field("", "", true)
        .field("Alert threshold", format!("{threshold} days"), true)
        .field("Webhook Notification?", webhook_check, true)
        .field("", "", true)
        .field("Email Notification?", email_check, true)
        .field("", "", false)
        .footer(|f| f.text("Remember that you can check your current alerts by using the slash command /list_alerts."))
        .color(0x2AC3DE)
        .to_owned()
}