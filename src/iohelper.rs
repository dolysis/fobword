use std::collections::VecDeque;
use std::io::{BufRead, Write};
use fobword_core::converter::{Converter, Keypress};

use crate::converterutilities;


#[allow(missing_debug_implementations)]
/// Struct that contains helper functions to simplify reading and writing HID reports
pub struct IOhelper
{
    /// The reader from which HID reports will be read
    pub reader: Box<dyn BufRead>,
    /// The writer to which HID reports will be written
    pub writer: Box<dyn Write>,
    
    pub buffer: [u8; 8],

    pub queue: VecDeque<Keypress>,
}

impl IOhelper
{
    /// Create a new helper from stuff
    pub(crate) fn new(reader: Box<dyn BufRead>, writer: Box<dyn Write>) -> IOhelper
    {
        let buffer = [0u8;8];
        let queue = VecDeque::new();
        IOhelper { reader, writer, buffer , queue}
    }

    pub fn write_key(&mut self, key: &Keypress, conv: &Converter) -> std::io::Result<()>
    {
        let (modifier, keycode) = conv.convert_keypress(&key);
        let buffer = [modifier as u8, 0, keycode, 0 ,0 ,0 ,0 ,0];
        self.write_to_file(&buffer)?;
        // Send an empty buffer to indicate key release
        let empty = [0,0,0,0,0,0,0,0];
        self.write_to_file(&empty)
    }

    /// Write a single buffer to the file, flushing it to ensure the data will be pushed to the file
    pub fn write_to_file(&mut self, buffer: &[u8]) -> std::io::Result<()>
    {
        self.writer.write(buffer)?;
        self.writer.flush()
    }

    /// Write multiple buffers to the file, flushing between every buffer, as to simulate a single HID report
    pub fn write_buffers_to_file(&mut self, buffers: Vec<Vec<u8>>) -> std::io::Result<()>
    {
        for i in buffers
        {
            self.write_to_file(&i)?;
        }
        Ok(())
    }


    pub fn next_key(&mut self, conv: &Converter) -> std::io::Result<Keypress>
    {
        let mut key = self.queue.pop_front();
        while let None = key
        {
            self.read_and_convert(conv)?;
            key = self.queue.pop_front();
        }
        Ok(key.unwrap())
    }

    fn read_and_convert(&mut self, conv: &Converter) -> std::io::Result<()>
    {
        let old_report = self.buffer.clone();
        self.reader.read_exact(&mut self.buffer)?;
        converterutilities::report_to_keypress(conv, &mut self.queue, &self.buffer, &old_report);
        Ok(())
    }

    pub fn read_line(&mut self, conv: &Converter) -> std::io::Result<String>
    {
        let mut result_string = String::new();

        loop 
        {
            let key = self.next_key(&conv)?;
            match key
            {
                Keypress::Character(character) => result_string.push(character),
                Keypress::Enter => return Ok(result_string),
                Keypress::Macro => return Ok(result_string),
                _ => continue,
            }
        }
    }
}