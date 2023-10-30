use sqlx;
use std::error::Error;
use crate::registry::types::{SqlxAlertDB, AlertDB, ErrorDB};

pub async fn create_alert(database: &sqlx::SqlitePool, user_id: i64, cooler: &str, loan_id: i64, threshold: i64, webhook_url: Option<String>, email: Option<String>) -> Result<[String; 2], Box<dyn Error + Send>>{
    let mut email_check = ":x:";
    let mut webhook_check = ":x:";
    match (webhook_url, email) {
        (Some(webhook_url), Some(email)) => {
            email_check = ":white_check_mark:";
            webhook_check = ":white_check_mark:";
            println!("Webhook: {:#?}, Email: {:#?}", webhook_url, email);
            sqlx::query!(
                "INSERT INTO alerts (user_id, cooler, loan_id, threshold, webhook_url, email, active) VALUES (?, ?, ?, ?, ?, ?, 1)",
                user_id,
                cooler,
                loan_id,
                threshold,
                webhook_url,
                email
            )
            .execute(database)
            .await
            .unwrap();
        }
        (Some(webhook_url), None) => {
            webhook_check = ":white_check_mark:";
            println!("Webhook: {:#?}", webhook_url);
            sqlx::query!(
                "INSERT INTO alerts (user_id, cooler, loan_id, threshold, webhook_url, active) VALUES (?, ?, ?, ?, ?, 1)",
                user_id,
                cooler,
                loan_id,
                threshold,
                webhook_url
            )
            .execute(database)
            .await
            .unwrap();
        }
        (None, Some(email)) => {
            email_check = ":white_check_mark:";
            println!("Email: {:#?}", email);
            sqlx::query!(
                "INSERT INTO alerts (user_id, cooler, loan_id, threshold, email, active) VALUES (?, ?, ?, ?, ?, 1)",
                user_id,
                cooler,
                loan_id,
                threshold,
                email
            )
            .execute(database)
            .await
            .unwrap();
        }
        (None, None) => {
            return Err(Box::new(ErrorDB::new("Error when trying to create the alert. Please try again!".into())));
        }
    }
    Ok([email_check.to_owned(), webhook_check.to_owned()])
}

pub async fn deactivate_alert(database: &sqlx::SqlitePool, alert_id: i64) -> Result<(), Box<dyn Error + Send>>{
    match sqlx::query!(
        "UPDATE alerts SET active = 0 WHERE alert_id = ?",
        alert_id
    )
    .execute(database)
    .await {
        Ok(_) => Ok(()),
        Err(error) => Err(Box::new(error))
    }
}

pub async fn delete_user_alerts_by_cooler(database: &sqlx::SqlitePool, user_id: i64, cooler: &str, loan_id: Option<i64>) -> Result<(), Box<dyn Error + Send>>{
    match loan_id {
        Some(loan_id) => {
            match sqlx::query!("DELETE FROM alerts WHERE user_id = ? AND cooler = ? AND loan_id = ?", user_id, cooler, loan_id)
            .fetch_all(database)
            .await {
                Ok(_) => Ok(()),
                Err(error) => Err(Box::new(error))
            }
        },
        None => {
            match sqlx::query!("DELETE FROM alerts WHERE user_id = ? AND cooler = ?", user_id, cooler)
            .fetch_all(database)
            .await {
                Ok(_) => Ok(()),
                Err(error) => Err(Box::new(error))
            }
        }
    }
}

pub async fn get_user_alerts(database: &sqlx::SqlitePool, user_id: i64) -> Result<Vec<AlertDB>, Box<dyn Error + Send>>{
    match sqlx::query_as!(SqlxAlertDB, "SELECT * FROM alerts WHERE user_id = ? ORDER BY rowid", user_id)
    .fetch_all(database)
    .await {
        Ok(alerts) => Ok(alerts.into_iter().map(|alert| alert.into()).collect()),
        Err(error) => Err(Box::new(error))
    }
}

pub async fn get_active_alerts(database: &sqlx::SqlitePool) -> Result<Vec<AlertDB>, Box<dyn Error + Send>>{
    match sqlx::query_as!(SqlxAlertDB, "SELECT * FROM alerts WHERE active = 1 ORDER BY rowid")
    .fetch_all(database)
    .await {
        Ok(alerts) => Ok(alerts.into_iter().map(|alert| alert.into()).collect()),
        Err(error) => Err(Box::new(error))
    }
}

pub async fn count_user_alerts(database: &sqlx::SqlitePool, user_id: i64) -> Result<i32, Box<dyn Error + Send>>{
    match sqlx::query!("SELECT COUNT(*) as count FROM alerts WHERE user_id = ?", user_id)
    .fetch_one(database)
    .await {
        Ok(query) => Ok(query.count),
        Err(error) => Err(Box::new(error))
    }
}

pub async fn count_user_alerts_by_cooler(database: &sqlx::SqlitePool, user_id: i64, cooler: &str, loan_id: Option<i64>) -> Result<i32, Box<dyn Error + Send>>{
    match loan_id {
        Some(loan_id) => {
            match sqlx::query!("SELECT COUNT(*) as count FROM alerts WHERE user_id = ? AND cooler = ? AND loan_id = ? ORDER BY rowid", user_id, cooler, loan_id)
            .fetch_one(database)
            .await {
                Ok(query) => Ok(query.count),
                Err(error) => Err(Box::new(error))
            }
        },
        None => {
            match sqlx::query!("SELECT COUNT(*) as count FROM alerts WHERE user_id = ? AND cooler = ? ORDER BY rowid", user_id, cooler)
            .fetch_one(database)
            .await {
                Ok(query) => Ok(query.count),
                Err(error) => Err(Box::new(error))
            }
        }
    }
}