use crate::consts::hotel;
use crate::error::AppError;
use crate::structs::Hotel;

use super::file_map_helper::FileMapHelper;

pub struct HotelFileMapHelper {}

impl Default for HotelFileMapHelper {
    fn default() -> Self {
        Self {}
    }
}

impl FileMapHelper for HotelFileMapHelper {
    type Return = Hotel;

    fn generate_hash(&self, text: &str) -> Result<String, AppError> {
        let hotel = json::parse(text)?;
        Ok(hotel[hotel::ID].to_string())
    }

    fn line_to_object(&self, text: &str) -> Option<Self::Return> {
        if let Ok(hotel) = json::parse(&text) {
            return Some(Self::Return {
                id: hotel[hotel::ID].to_string(),
                city_code: hotel[hotel::CITY_CODE].to_string(),
                name: hotel[hotel::NAME].to_string(),
                category: hotel[hotel::CATEGORY].to_string(),
                country_code: hotel[hotel::COUNTRY_CODE].to_string(),
                city: hotel[hotel::CITY].to_string(),
            });
        }
        None
    }
}
