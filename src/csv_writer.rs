use std::io::Write;

use crate::error::AppError;
use csv::{Writer, WriterBuilder};

#[derive(Debug)]
pub struct CsvWriter<F: Write> {
    writer: Writer<F>,
    delimiter: char,
}

impl<F: Write> CsvWriter<F> {
    pub fn new(handler: F, delimiter: char) -> Result<Self, AppError> {
        Ok(Self {
            writer: WriterBuilder::new()
                .buffer_capacity(20 * 1024)
                .from_writer(handler),
            delimiter,
        })
    }

    pub fn write(&mut self, to_write: &[&str]) -> Result<(), AppError> {
        for text in to_write {
            self.writer.write_field(text)?;
        }
        self.writer.write_record(None::<&[u8]>)?;

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), AppError> {
        Ok(self.writer.flush()?)
    }
}
