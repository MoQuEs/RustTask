pub mod input {
    pub const CITY_CODE: usize = 0;
    pub const HOTEL_CODE: usize = 1;
    pub const ROOM_TYPE: usize = 2;
    pub const ROOM_CODE: usize = 3;
    pub const MEAL: usize = 4;
    pub const CHECKIN: usize = 5;
    pub const ADULTS: usize = 6;
    pub const CHILDREN: usize = 7;
    pub const PRICE: usize = 8;
    pub const SOURCE: usize = 9;
}

pub mod room_name {
    pub const HOTEL_CODE: usize = 0;
    pub const SOURCE: usize = 1;
    pub const ROOM_NAME: usize = 2;
    pub const ROOM_CODE: usize = 3;
}

pub mod hotel {
    pub const ID: &'static str = "id";
    pub const CITY_CODE: &'static str = "city_code";
    pub const NAME: &'static str = "name";
    pub const CATEGORY: &'static str = "category";
    pub const COUNTRY_CODE: &'static str = "country_code";
    pub const CITY: &'static str = "city";
}

pub mod file {
    pub const INPUT_FILE: &'static str = "input.csv";
    pub const HOTELS_FILE: &'static str = "hotels.json";
    pub const ROOM_NAMES_FILE: &'static str = "room_names.csv";
    pub const OUTPUT_FILE: &'static str = "output.csv";
}
