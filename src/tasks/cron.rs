use std::time::Duration;
use tokio::time::sleep;
use reqwest::Client;

pub async fn start_cron() {
    tokio::spawn(async move {
        let client = Client::new();
        let url = std::env::var("CRON_PING_URL")
            .unwrap_or_else(|_| "https://drafly.onrender.com/health".to_string());

        loop {
            // Wait before each ping
            sleep(Duration::from_secs(300)).await; // every 5 minutes

            match client.get(&url).send().await {
                Ok(res) => {
                    if res.status().is_success() {
                        println!("[CRON] Ping successful: {}", res.status());
                    } else {
                        eprintln!("[CRON] Ping failed with status: {}", res.status());
                    }
                }
                Err(e) => {
                    eprintln!("[CRON] Network error: {}", e);
                }
            }
        }
    });
}
