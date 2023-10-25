use sqlx;
use serenity::builder::{
    CreateApplicationCommand,
    CreateEmbed,
};
use crate::registry::utils::get_user_alerts;


pub async fn run(database: &sqlx::SqlitePool, user_id: i64) -> CreateEmbed {
    let alerts = match get_user_alerts(database, user_id).await {
        Ok(alerts) => alerts,
        Err(error) => {
            return CreateEmbed::default()
            .title("Something went wrong!")
            .description("Unable to retrieve alerts form the DB. Please try again.")
            .field("Error", error.to_string(), false)
            .color(0xDB4B4B)
            .to_owned();
        }
    };        

    let mut embed = CreateEmbed::default()
        .color(0xC7D5E8)
        .field(" ", " ", false)
        .to_owned();


    let num_alerts = alerts.len();
    if num_alerts == 0 {
        embed.title("You don't have any alerts");
        embed.description("You can create a new alert by using the slash command /create_alert.");
    } else {
        if num_alerts == 1 {
            embed.title(format!("You have 1 alert:\n"));
        } else {
            embed.title(format!("You have {} alerts:\n", num_alerts));
        }
        for (_, alert) in alerts.iter().enumerate() {
            let email_check = match alert.get_email() {
                Some(_) => ":white_check_mark:",
                None => ":x:",
            };
            let webhook_check = match alert.get_webhook_url() {
                Some(_) => ":white_check_mark:",
                None => ":x:",
            };
            let trigger_check = match alert.is_active() {
                true => ":x:",
                false => ":white_check_mark:",
            };
            embed.field(" ", " ", false);
            embed.field(" ", " ", false);
            embed.field("Cooler Contract", alert.get_cooler(), true);
            embed.field("Loan ID", alert.get_loan_id(), true);
            embed.field("Alert threshold", format!("{} days", alert.get_threshold()), true);
            embed.field("Webhook Notification?", webhook_check, true);
            embed.field("Email Notification?", email_check, true);
            embed.field("Already Triggered?", trigger_check, true);
        }
    }

    return embed;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("list_alerts").description("List all user alerts")
}