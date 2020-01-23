use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::str::from_utf8;

use crate::error::AppError;
use crate::file_line_map::FileMapHelper;

pub struct FileLineMap<FMH: FileMapHelper> {
    handler: BufReader<File>,
    map: BTreeMap<String, u32>,
    helper: FMH,
    text_buffer: Vec<u8>,
}

impl<FMH: FileMapHelper> FileLineMap<FMH> {
    pub fn new(mut file: File, helper: FMH) -> Result<Self, AppError> {
        let mut map = BTreeMap::new();
        let mut text_buffer = Vec::with_capacity(512);
        let mut line_buffer = 0;

        file.seek(SeekFrom::Start(0))?;
        let mut handler = BufReader::new(file);
        let mut buffer_size: usize;
        loop {
            text_buffer.clear();
            buffer_size = handler.read_until(b'\n', &mut text_buffer)?;
            if buffer_size == 0 {
                break;
            };

            let parsed_id = helper.generate_hash(from_utf8(&text_buffer)?)?;
            map.insert(parsed_id, line_buffer);

            line_buffer += buffer_size as u32;
        }

        Ok(Self {
            handler,
            map,
            helper,
            text_buffer,
        })
    }

    pub fn get(&mut self, text: &str) -> Result<FMH::Return, AppError> {
        let text = self.line_from_file_by_hashmap(text)?;
        if let Some(obj) = self.helper.line_to_object(&text) {
            return Ok(obj);
        }

        AppError::new_res("HelperHashMap", "Can't get value")
    }

    fn line_from_file_by_hashmap(&mut self, text: &str) -> Result<String, AppError> {
        if let Some(buff) = self.map.get(text) {
            self.handler.seek(SeekFrom::Start(buff.clone() as u64))?;

            self.text_buffer.clear();
            self.handler.read_until(b'\n', self.text_buffer.as_mut())?;

            return Ok(from_utf8(&self.text_buffer)?.to_owned());
        }

        AppError::new_res("file_map", "Can't get line from file by hashmap")
    }
}
