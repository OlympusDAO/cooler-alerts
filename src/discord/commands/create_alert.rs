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

    let threshold = match &options.get(2).expect("Expected threshold").value {
        Some(threshold) => threshold,
        None => panic!("Expected threshold"),
    };

    sqlx::query!(
        "INSERT INTO alerts (user_id, cooler, loan_id, threshold) VALUES (?, ?, ?, ?)",
        user_id,
        cooler,
        loan_id,
        threshold
    )
    .execute(database)
    .await
    .unwrap();
    
    ("Alert successfully added!").to_string()
}

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
                .description("The ID of the loan to monitor")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("threshold")
                .description("The days in advance to notify the user before expiration")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
}