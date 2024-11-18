use chrono::NaiveDateTime;

pub fn format_date(date: NaiveDateTime) -> String {
    if date.date() == chrono::Local::now().date_naive() {
        date.format("%H:%M:%S").to_string()
    } else {
        date.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
