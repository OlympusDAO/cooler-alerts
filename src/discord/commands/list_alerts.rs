
use serenity::builder::CreateApplicationCommand;
use sqlx;

pub async fn run(database: &sqlx::SqlitePool, user_id: i64) -> String {
    let alerts = sqlx::query!("SELECT * FROM alerts WHERE user_id = ? ORDER BY rowid", user_id)
        .fetch_all(database)
        .await
        .unwrap();

    let mut response = format!("You have {} alerts:\n", alerts.len());
    for (i, alert) in alerts.iter().enumerate() {
        response.push_str(&format!(
            "Alert #{}:\n - cooler: {}\n - loan_id: {}\n - threshold: {} days",
            i + 1,
            alert.cooler,
            alert.loan_id,
            alert.threshold
        ));
    }
    response
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("list_alerts").description("List all user alerts")
}