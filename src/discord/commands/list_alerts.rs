use serenity::builder::{
    CreateApplicationCommand,
    CreateEmbed,
};
use sqlx;

pub async fn run(database: &sqlx::SqlitePool, user_id: i64) -> CreateEmbed {
    let alerts = sqlx::query!("SELECT * FROM alerts WHERE user_id = ? ORDER BY rowid", user_id)
        .fetch_all(database)
        .await
        .unwrap();



    let mut embed = CreateEmbed::default()
        .color(0xC7D5E8)
        .field(" ", " ", false)
        .to_owned();
    for (i, alert) in alerts.iter().enumerate() {
        embed.field("Cooler Contract", &alert.cooler, true);
        embed.field("Loan ID", &alert.loan_id, true);
        embed.field("", "", true);
        embed.field("Alert threshold", format!("{} days", &alert.threshold), true);
        embed.field("Discord Notification?", "False", true);
        embed.field("Email Notification?", "False", true);
        embed.field(" ", " ", false);
        embed.field(" ", " ", false);
    }

    if (alerts.len() == 0) {
        embed.title("You don't have any alerts");
        embed.description("You can create a new alert by using the slash command /create_alert.");
    } else {
        embed.title(format!("You have {} alerts:\n", alerts.len()));
    }

    return embed;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("list_alerts").description("List all user alerts")
}