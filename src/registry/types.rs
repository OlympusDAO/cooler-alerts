use std::error::Error;
use std::fmt;

// Custom errors
#[derive(Debug)]
pub struct ErrorDB(String);

impl ErrorDB {
    pub fn new(message: &str) -> Self {
        ErrorDB(message.to_string())
    }
}

impl Error for ErrorDB {}

impl fmt::Display for ErrorDB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Alert Struct for the DB entries.
// Struct with non-public attributes + getter methods so that it can be safely used.
#[derive(Clone, Debug)]
pub struct AlertDB {
    alert_id: i64,
    user_id: i64,
    cooler: String,
    loan_id: i64,
    threshold: i64,
    webhook_url: Option<String>,
    email: Option<String>,
    active: bool,
}

impl AlertDB {
    pub fn get_alert_id(&self) -> i64 {
        self.alert_id
    }

    pub fn get_user_id(&self) -> i64 {
        self.user_id
    }

    pub fn get_cooler(&self) -> &str {
        &self.cooler
    }

    pub fn get_loan_id(&self) -> i64 {
        self.loan_id
    }

    pub fn get_threshold(&self) -> i64 {
        self.threshold
    }

    pub fn get_webhook_url(&self) -> Option<&str> {
        match &self.webhook_url {
            Some(webhook_url) => Some(webhook_url),
            None => None,
        }
    }

    pub fn get_email(&self) -> Option<&str> {
        match &self.email {
            Some(email) => Some(email),
            None => None,
        }
    }

    pub fn is_active(&self) -> bool {
        println!("Active: {}", self.active);
        self.active
    }
}

// Alert Struct for the DB entries.
// Only used when directly reading from the DB.
// All its attributes are public so that new entities can be created by sqlx.
#[derive(Clone, Debug)]
pub struct SqlxAlertDB {
    pub alert_id: i64,
    pub user_id: i64,
    pub cooler: String,
    pub loan_id: i64,
    pub threshold: i64,
    pub webhook_url: Option<String>,
    pub email: Option<String>,
    pub active: bool,
}

impl From<SqlxAlertDB> for AlertDB {
    fn from(item: SqlxAlertDB) -> Self {
        AlertDB {
            alert_id: item.alert_id,
            user_id: item.user_id,
            cooler: item.cooler,
            loan_id: item.loan_id,
            threshold: item.threshold,
            webhook_url: item.webhook_url,
            email: item.email,
            active: item.active,
        }
    }
}