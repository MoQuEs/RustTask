#[derive(Debug, Clone)]
pub struct RoomName {
    pub hotel_code: String,
    pub source: String,
    pub room_name: String,
    pub room_code: String,
}

#[derive(Debug, Clone)]
pub struct Hotel {
    pub id: String,
    pub city_code: String,
    pub name: String,
    pub category: String,
    pub country_code: String,
    pub city: String,
}
