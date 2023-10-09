use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::{
    command::CommandOptionType,
    application_command::CommandDataOption,
};
use sqlx;

pub async fn run(database: &sqlx::SqlitePool, user_id: i64, options: &[CommandDataOption]) -> String {
    let cooler = match &options.get(0).expect("Expected cooler").value {
        Some(cooler) => cooler,
        None => panic!("Expected cooler"),
    };

    let loan_id = match &options.get(1).expect("Expected loan_id").value {
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

    format!("Successfully deleted all alerts for loan_id: {} of cooler: {}", loan_id, cooler)
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
                .description("The ID of the monitored loan")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
}