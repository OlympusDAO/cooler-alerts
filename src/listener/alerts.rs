use serenity::http::Http;
use serenity::model::{
    webhook::Webhook,
    channel::Embed,
};
use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

// Public method to send alerts via discord webhooks.
pub async fn send_webhook(webhook_url: &str, cooler: &str, loan_id: i64, days: u64) {
    let http = Http::new("");
    let webhook = Webhook::from_url(&http, &webhook_url).await.unwrap();
    let embed = Embed::fake(|e| {
        e.title(format!("New Alert!"))
            .description(format!("Cooler Contract: [{cooler}](https://www.etherscan.io/address/{cooler}) is about to expire!"))
            .field("Loan ID", loan_id, true)
            .field("Time Left", format!("{days} days"), true)
            .field(" ", " ", false)
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

// Public method to send alerts via an email.
pub async fn send_email(creds: Credentials, receiver: &str, cooler: &str, loan_id: i64, days: u64) {
    let email = Message::builder()
        .from("Cooler Monitoring <test@gmail.com>".parse().unwrap())
        .to(receiver.parse().unwrap())
        .subject("New Cooler Alert!")
        .header(ContentType::TEXT_PLAIN)
        .body(format!("Cooler: {cooler} is about to expire!\n- Loan ID: {loan_id}\n- Time Left: {days} days\n\nhttps://www.etherscan.io/address/{cooler}", cooler=cooler, loan_id=loan_id, days=days))
        .unwrap();

    let smtp = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match smtp.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}