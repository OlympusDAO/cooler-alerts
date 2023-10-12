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
    let alerts = sqlx::query!("SELECT * FROM alerts WHERE user_id = ? ORDER BY rowid", user_id)
        .fetch_all(database)
        .await
        .unwrap();
    if (alerts.len() == 3) {
        return CreateEmbed::default()
            .title("Each user is limited to 3 alerts")
            .description("You can delete some alerts by using the slash command /delete_alerts.")
            .color(0x6AE5B3)
            .to_owned();
    }
    
    let cooler = match &options.get(0).expect("Expected cooler").value {
        Some(cooler) => match cooler.as_str() {
            Some(cooler) => cooler,
            None => panic!("Expected cooler"),
        },
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

    CreateEmbed::default()
        .title("Alert successfully added")
        .description(format!("Cooler Contract: [{cooler}](https://www.etherscan.io/address/{cooler})"))
        .field("Loan ID", loan_id, true)
        .field("", "", true)
        .field("Alert threshold", format!("{threshold} days"), true)
        .field("Discord Notification?", "False", true)
        .field("", "", true)
        .field("Email Notification?", "False", true)
        .field("", "", false)
        .footer(|f| f.text("Remember that you can check your current alerts by using the slash command /list_alerts."))
        .color(0x2AC3DE)
        .to_owned()
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