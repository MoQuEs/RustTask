use crate::consts::room_name;
use crate::error::AppError;
use crate::structs::RoomName;

use super::file_map_helper::FileMapHelper;

pub struct RoomNameFileMapHelper {}

impl Default for RoomNameFileMapHelper {
    fn default() -> Self {
        Self {}
    }
}

impl FileMapHelper for RoomNameFileMapHelper {
    type Return = RoomName;

    fn generate_hash(&self, text: &str) -> Result<String, AppError> {
        let columns: Vec<&str> = text.split('|').collect();

        let mut parsed = String::with_capacity(
            1 + columns[room_name::HOTEL_CODE].len()
                + columns[room_name::SOURCE].len()
                + columns[room_name::ROOM_CODE].len(),
        );

        parsed.push_str(columns[room_name::HOTEL_CODE]);
        parsed.push_str(columns[room_name::SOURCE]);
        parsed.push_str(columns[room_name::ROOM_CODE].trim());

        Ok(parsed)
    }

    fn line_to_object(&self, text: &str) -> Option<Self::Return> {
        let vec_str: Vec<&str> = text.split('|').collect();
        return Some(Self::Return {
            hotel_code: String::from(vec_str[room_name::HOTEL_CODE]),
            source: String::from(vec_str[room_name::SOURCE]),
            room_name: String::from(vec_str[room_name::ROOM_NAME]),
            room_code: String::from(vec_str[room_name::ROOM_CODE].trim()),
        });
    }
}
