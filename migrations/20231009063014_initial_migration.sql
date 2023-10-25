-- Add migration script here
CREATE TABLE alerts (
    alert_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    cooler TEXT NOT NULL,
    loan_id INTEGER NOT NULL,
    threshold INTEGER NOT NULL,
    webhook_url TEXT,
    email TEXT,
    active BOOLEAN NOT NULL
)