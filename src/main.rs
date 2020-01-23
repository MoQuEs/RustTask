use std::fs::OpenOptions;

use crate::consts::{file, input};
use crate::csv_reader::CsvReader;
use crate::csv_writer::CsvWriter;
use crate::error::AppError;
use crate::file_line_map::{FileLineMap, HotelFileMapHelper, RoomNameFileMapHelper};
use crate::functions::{checkin, checkout, pax, price, room_type_with_meal};

mod consts;
mod csv_reader;
mod csv_writer;
mod error;
mod file_line_map;
mod functions;
mod structs;

fn main() -> Result<(), AppError> {
    let input_handler = OpenOptions::new().read(true).open(file::INPUT_FILE)?;

    let hotels_handler = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file::HOTELS_FILE)?;

    let room_names_handler = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file::ROOM_NAMES_FILE)?;

    let output_handler = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file::OUTPUT_FILE)?;

    let mut output = CsvWriter::new(output_handler, ';')?;
    let mut input_reader = CsvReader::new(input_handler, b'|', true);
    let mut hotel_map = FileLineMap::new(hotels_handler, HotelFileMapHelper::default())?;
    let mut room_name_map = FileLineMap::new(room_names_handler, RoomNameFileMapHelper::default())?;

    let mut hotel_hash = String::with_capacity(50);
    let mut room_names_hash = String::with_capacity(50);

    output.write(&[
        "room_type meal",
        "room_code",
        "source",
        "hotel_name",
        "city_name",
        "city_code",
        "hotel_category",
        "pax",
        "adults",
        "children",
        "room_name",
        "checkin",
        "checkout",
        "price",
    ])?;

    for input_res_line in input_reader.records()? {
        if let Ok(input_line) = input_res_line {
            let pax = pax(&input_line[input::ADULTS], &input_line[input::CHILDREN])?;
            let (checkin_date, checkin_utc) = checkin(&input_line[input::CHECKIN])?;

            hotel_hash.clear();
            hotel_hash.push_str(&input_line[input::HOTEL_CODE]);
            let hotel = hotel_map.get(&hotel_hash)?;

            room_names_hash.clear();
            room_names_hash.push_str(&input_line[input::HOTEL_CODE]);
            room_names_hash.push_str(&input_line[input::SOURCE]);
            room_names_hash.push_str(&input_line[input::ROOM_CODE]);
            let room_name = room_name_map.get(&room_names_hash)?;

            output.write(&[
                &room_type_with_meal(&input_line[input::ROOM_TYPE], &input_line[input::MEAL]),
                &input_line[input::ROOM_CODE],
                &input_line[input::SOURCE],
                &hotel.name,
                &hotel.city,
                &input_line[input::CITY_CODE],
                &hotel.category,
                &pax.to_string(),
                &input_line[input::ADULTS],
                &input_line[input::CHILDREN],
                &room_name.room_name,
                &checkin_date,
                &checkout(checkin_utc, 1),
                &price(pax, &input_line[input::PRICE])?,
            ])?;
        }
    }

    output.flush()?;

    Ok(())
}
