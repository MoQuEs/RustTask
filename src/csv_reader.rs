use std::io::{Read, Seek};

use csv::{Position, Reader, ReaderBuilder, StringRecordsIter};

use crate::error::AppError;

#[derive(Debug)]
pub struct CsvReader<T: Read + Seek> {
    reader: Reader<T>,
    header: bool,
}

impl<T: Read + Seek> CsvReader<T> {
    pub fn new(handler: T, delimiter: u8, header: bool) -> Self {
        let reader = ReaderBuilder::new()
            .delimiter(delimiter)
            .has_headers(header)
            .from_reader(handler);

        Self { reader, header }
    }

    pub fn records(&mut self) -> Result<StringRecordsIter<T>, AppError> {
        let mut position = Position::new();
        position.set_byte(if self.header {
            self.reader.byte_headers()?.len() as u64
        } else {
            0
        });
        position.set_line(if self.header { 2 } else { 1 });
        position.set_record(if self.header { 1 } else { 0 });

        self.reader.seek(position)?;
        Ok(self.reader.records())
    }
}
