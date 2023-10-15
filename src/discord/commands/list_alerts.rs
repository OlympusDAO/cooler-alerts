use serenity::builder::{
    CreateApplicationCommand,
    CreateEmbed,
};
use sqlx;

pub async fn run(database: &sqlx::SqlitePool, user_id: i64) -> CreateEmbed {
    let alerts = sqlx::query!("SELECT * FROM alerts WHERE user_id = ?", user_id)
        .fetch_all(database)
        .await
        .unwrap();

    let mut embed = CreateEmbed::default()
        .color(0xC7D5E8)
        .field(" ", " ", false)
        .to_owned();


    if (alerts.len() == 0) {
        embed.title("You don't have any alerts");
        embed.description("You can create a new alert by using the slash command /create_alert.");
    } else {
        embed.title(format!("You have {} alerts:\n", alerts.len()));
        for (i, alert) in alerts.iter().enumerate() {
            let email_check = match &alert.email {
                Some(_) => ":white_check_mark:",
                None => ":x:",
            };
            let webhook_check = match &alert.webhook_url {
                Some(_) => ":white_check_mark:",
                None => ":x:",
            };
            embed.field(" ", " ", false);
            embed.field(" ", " ", false);
            embed.field("Cooler Contract", &alert.cooler, true);
            embed.field("Loan ID", &alert.loan_id, true);
            embed.field("Alert threshold", format!("{} days", &alert.threshold), true);
            embed.field("Webhook Notification?", webhook_check, true);
            embed.field("Email Notification?", email_check, true);
            embed.field("", "", true);
        }
    }

    return embed;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("list_alerts").description("List all user alerts")
}