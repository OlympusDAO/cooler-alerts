use serenity::builder::{
    CreateApplicationCommand,
    CreateEmbed,
};
use serenity::model::prelude::{
    command::CommandOptionType,
    application_command::CommandDataOption,
};
use sqlx;

pub async fn run(database: &sqlx::SqlitePool, user_id: i64, options: &[CommandDataOption]) -> CreateEmbed {
    let cooler = match &options.get(0).expect("Expected cooler").value {
        Some(cooler) => match cooler.as_str() {
            Some(cooler) => cooler,
            None => panic!("Expected cooler"),
        },
        None => panic!("Expected cooler"),    
    };

    match &options.get(1) {
        Some(loan_id) => {
            let loan_id = match &loan_id.value {
                Some(loan_id) => loan_id,
                None => panic!("Expected loan_id"),
            };
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
    };

    // format!("Successfully deleted all alerts for loan_id: {} of cooler: {}", loan_id, cooler)
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