mod alerts;
use alerts::send_webhook;
use lettre::transport::smtp::authentication::Credentials;
use crate::{registry::utils::{get_active_alerts, deactivate_alert}, listener::alerts::send_email};

use sqlx;
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use ethers::{
    prelude::abigen,
    providers::{Http, Provider},
    types::{Address, U256}
};

abigen!(
    ICoolerMonitoring,
    "[function timeToExpiry(address cooler_, uint256 loanID_) external view returns (uint256 secondsToExpiry)]"
);

pub async fn monitor(contract_address: Address, provider: Arc<Provider<Http>>, database: &sqlx::SqlitePool, email_creds: Credentials) {
    // Initialize a new instance of the Weth/Dai Uniswap V2 pair contract
    let contract = ICoolerMonitoring::new(contract_address, provider);
    println!("\n\nMonitoring contract: {:?}", contract);
    loop {
        let alerts = match get_active_alerts(database).await {
            Ok(alerts) => alerts,
            Err(error) => {
                println!("Error: {:?}", error);
                Vec::new()
            }
        };

        for i in 0..alerts.len() {
            let cooler: Address = alerts[i].get_cooler().parse().unwrap();
            let loan_id: U256 = U256::from(alerts[i].get_loan_id());
            let threshold: U256 = U256::from(alerts[i].get_threshold() * 24 * 3600);
            if let Ok(time_left) = contract.time_to_expiry(
                cooler,
                loan_id
            ).call().await {
                if time_left <= threshold {
                    // Send webhook alerts
                    match alerts[i].get_webhook_url() {
                        Some(webhook_url) => {
                            send_webhook(
                                webhook_url,
                                alerts[i].get_cooler(),
                                alerts[i].get_loan_id(),
                                time_left.as_u64() / 24 / 3600
                            ).await;
                            match deactivate_alert(database, alerts[i].get_alert_id()).await {
                                Ok(_) => (),
                                Err(_) => {
                                    // Try again after 1 seconds
                                    sleep(Duration::from_secs(1)).await;
                                    match deactivate_alert(database, alerts[i].get_alert_id()).await {
                                        Ok(_) => (),
                                        Err(error) => println!("Error: {:?}", error),
                                    }
                                }
                            }
                        },
                        None => (),
                    };
                    // Send email alerts
                    match alerts[i].get_email() {
                        Some(receiver) => {
                            send_email(
                                email_creds.clone(),
                                receiver,
                                alerts[i].get_cooler(),
                                alerts[i].get_loan_id(),
                                time_left.as_u64() / 24 / 3600
                            ).await;
                            match deactivate_alert(database, alerts[i].get_alert_id()).await {
                                Ok(_) => (),
                                Err(_) => {
                                    // Try again after 1 seconds
                                    sleep(Duration::from_secs(1)).await;
                                    match deactivate_alert(database, alerts[i].get_alert_id()).await {
                                        Ok(_) => (),
                                        Err(error) => println!("Error: {:?}", error),
                                    }
                                }
                            }
                        },
                        None => (),
                    }
                }
            }
        }
        // sleep(Duration::from_secs(24*3600)).await;
        sleep(Duration::from_secs(30)).await;
    }
}