use chrono::NaiveDateTime;
use tower_sessions::Session;

use crate::middleware::FROM_PROTECTED_KEY;

pub fn format_date(date: NaiveDateTime) -> String {
    if date.date() == chrono::Local::now().date_naive() {
        date.format("%H:%M:%S").to_string()
    } else {
        date.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

pub async fn get_protected(session: Session) -> bool {
    let from_protected: bool = session
        .get(FROM_PROTECTED_KEY)
        .await
        .unwrap()
        .unwrap_or_default();
    from_protected
}
