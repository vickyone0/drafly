use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailRow {
    pub id: i64,
    pub gmail_id: String,
    pub thread_id: Option<String>,
    pub user_email: Option<String>,
    pub sender: Option<String>,
    pub to_recipients: Option<String>,
    pub subject: Option<String>,
    pub snippet: Option<String>,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub labels: Option<Vec<String>>,
    pub fetched_at: NaiveDateTime,
}
