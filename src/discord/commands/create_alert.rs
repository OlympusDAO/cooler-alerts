use serenity::builder::{
    CreateApplicationCommand,
    CreateEmbed,
};
use serenity::model::prelude::{
    command::CommandOptionType,
    application_command::CommandDataOption,
};
use crate::registry::utils::{
    count_user_alerts,
    create_alert,
};

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

pub async fn run(database: &sqlx::SqlitePool, user_id: i64, options: &[CommandDataOption]) -> CreateEmbed {
    let alerts_prev = match count_user_alerts(database, user_id).await {
        Ok(alerts) => alerts,
        Err(error) => {
            return CreateEmbed::default()
            .title("Something went wrong!")
            .description("Unable to retrieve current alerts form the DB. Please try again.")
            .field("Error", error.to_string(), false)
            .color(0xDB4B4B)
            .to_owned();
        }
    };

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
                            webhook_url = Some(receiver.to_string());
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
                            webhook_url = Some(receiver.to_string());
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

    let [email_check, webhook_check] = match create_alert(database, user_id, cooler, loan_id, threshold, webhook_url, email).await {
        Ok([email_check, webhook_check]) => [email_check, webhook_check],
        Err(error) => {
            return CreateEmbed::default()
            .title("Something went wrong!")
            .description("Error when trying to register the alerts in the database. Please try again.")
            .field("Error", error.to_string(), false)
            .color(0xDB4B4B)
            .to_owned();
        }
    };

    let alerts_after = match count_user_alerts(database, user_id).await {
        Ok(alerts) => alerts,
        Err(error) => {
            return CreateEmbed::default()
            .title("Something went wrong!")
            .description("Unable to retrieve current alerts form the DB. Please try again.")
            .field("Error", error.to_string(), false)
            .color(0xDB4B4B)
            .to_owned();
        }
    };

    if alerts_after == alerts_prev {
        return CreateEmbed::default()
            .title("Something went wrong!")
            .description("Error when trying to register the alerts in the database. Please try again.")
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