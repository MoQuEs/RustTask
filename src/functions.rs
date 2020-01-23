use chrono::{Date, Duration, TimeZone, Utc};

use crate::error::AppError;

pub fn room_type_with_meal(room_type: &str, meal: &str) -> String {
    let mut str = String::with_capacity(2 + room_type.len() + meal.len());

    str.push_str(room_type);
    str.push(' ');
    str.push_str(meal);

    str
}

pub fn pax(adults: &str, children: &str) -> Result<u32, AppError> {
    Ok(adults.parse::<u32>()? + children.parse::<u32>()?)
}

pub fn price(pax: u32, price: &str) -> Result<String, AppError> {
    Ok(format!("{:.2}", price.parse::<f64>()? / pax as f64))
}

pub fn checkin(checkin: &str) -> Result<(String, Date<Utc>), AppError> {
    let mut str = String::with_capacity(3 + checkin.len());

    str.push_str(&checkin[0..4]);
    str.push('-');
    str.push_str(&checkin[4..6]);
    str.push('-');
    str.push_str(&checkin[6..8]);

    Ok((
        str,
        Utc.ymd(
            checkin[0..4].parse::<i32>()?,
            checkin[4..6].parse::<u32>()?,
            checkin[6..8].parse::<u32>()?,
        ),
    ))
}

pub fn checkout(checkin: Date<Utc>, days: i64) -> String {
    (checkin + Duration::days(days))
        .format("%Y-%m-%d")
        .to_string()
}
