-- Add migration script here
CREATE TABLE alerts (
    user_id INTEGER NOT NULL,
    cooler TEXT NOT NULL,
    loan_id INTEGER NOT NULL,
    threshold INTEGER NOT NULL
)