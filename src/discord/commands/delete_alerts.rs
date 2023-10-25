use sqlx;
use serenity::builder::{
    CreateApplicationCommand,
    CreateEmbed,
};
use serenity::model::prelude::{
    command::CommandOptionType,
    application_command::CommandDataOption,
};
use crate::registry::utils::{
    count_user_alerts_by_cooler,
    delete_user_alerts_by_cooler
};

pub async fn run(database: &sqlx::SqlitePool, user_id: i64, options: &[CommandDataOption]) -> CreateEmbed {    
    let cooler = match &options.get(0).expect("Expected cooler").value {
        Some(input) => match input.as_str() {
            Some(cooler_address) => cooler_address,
            None => panic!("Expected cooler"),
        },
        None => panic!("Expected cooler"),    
    };

    let deleted_loan_id: Option<i64> = match &options.get(1) {
        Some(input) => {
            match &input.value {
                Some(loan_id) => loan_id.as_i64(),
                None => panic!("Expected loan_id"),
            }
        },
        None => None
    };

    let alerts = match count_user_alerts_by_cooler(database, user_id, cooler, deleted_loan_id).await {
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

    if alerts == 0 {
        return CreateEmbed::default()
        .title("No alerts found")
        .description(format!("Cooler Contract: [{cooler}](https://www.etherscan.io/address/{cooler})"))
        .field("", "", false)
        .footer(|f| f.text("Remember that you can check your current alerts by using the slash command /list_alerts."))
        .color(0xDB4B4B)
        .to_owned();
    }

    match delete_user_alerts_by_cooler(database, user_id, cooler, deleted_loan_id).await {
        Ok(alerts) => alerts,
        Err(error) => {
            return CreateEmbed::default()
            .title("Something went wrong!")
            .description("Unable to delete alerts form the DB. Please try again.")
            .field("Error", error.to_string(), false)
            .color(0xDB4B4B)
            .to_owned();
        }
    }
    match deleted_loan_id {
        Some(loan_id) => {
            return CreateEmbed::default()
            .title("Alerts successfully deleted")
            .description(format!("Cooler Contract: [{cooler}](https://www.etherscan.io/address/{cooler})"))
            .field("Loan ID", loan_id, false)
            .field("", "", false)
            .footer(|f| f.text("Remember that you can check your current alerts by using the slash command /list_alerts."))
            .color(0x2AC3DE)
            .to_owned();
        },
        None => {
            return CreateEmbed::default()
            .title("Alerts successfully deleted")
            .description(format!("Cooler Contract: [{cooler}](https://www.etherscan.io/address/{cooler})"))
            .field("", "", false)
            .footer(|f| f.text("Remember that you can check your current alerts by using the slash command /list_alerts."))
            .color(0x2AC3DE)
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