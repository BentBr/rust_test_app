use chrono::NaiveDateTime;

pub fn format_datetime(dt: Option<NaiveDateTime>) -> Option<String> {
    dt.map(|datetime| datetime.to_string())
}
