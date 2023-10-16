use serenity::http::Http;
use serenity::model::{
    webhook::Webhook,
    channel::Embed,
};

pub async fn send_alert(webhook_url: &str, cooler: &str, loan_id: u64, days: u64) {
    // let webhook_url = std::env::var("DISCORD_WEBHOOK").expect("missing DISCORD_WEBHOOK");
    let http = Http::new("");
    let webhook = Webhook::from_url(&http, &webhook_url).await.unwrap();
    let embed = Embed::fake(|e| {
        e.title(format!("New Alert!"))
            .description(format!("Cooler Contract: [{cooler}](https://www.etherscan.io/address/{cooler}) is about to expire!"))
            .field("Loan ID", loan_id, false)
            .field("Time Left", format!("{days} days"), false)
            .footer(|f| f.text("Remember that you can check your current alerts by using the slash command /list_alerts."))
            .color(0xDB4B4B)
    });

    webhook
        .execute(&http, false, |w| {
            w.content("")
                .username("Webhook test")
                .embeds(vec![embed])
        })
        .await
        .expect("Could not execute webhook.");
}
