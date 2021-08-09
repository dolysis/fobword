use std::collections::VecDeque;
use std::io::{BufRead, Write, BufReader};
use std::fs::OpenOptions;
use std::path::Path;

use crate::converter::{Converter, Keypress};
use crate::error::DataHandleError;

#[allow(missing_debug_implementations)]
/// Struct that contains helper functions to simplify reading and writing HID reports
pub struct IOhelper
{
    /// The reader from which HID reports will be read
    pub reader: Box<dyn BufRead>,
    /// The writer to which HID reports will be written
    pub writer: Box<dyn Write>,
    
    pub buffer: [u8; 8],

    pub converter: Converter,

    pub character_queue: VecDeque<Keypress>
}

impl IOhelper
{
    /// Create a new helper from stuff
    pub(crate) fn new(reader: Box<dyn BufRead>, writer: Box<dyn Write>, converter: Converter) -> IOhelper
    {
        let buffer = [0u8;8];
        let character_queue = VecDeque::new();
        IOhelper { reader, writer, buffer, converter, character_queue}
    }

    pub fn write_key(&mut self, key: Keypress) -> std::io::Result<()>
    {
        if let Some((modifier, code)) = self.converter.keypress_to_report_code(key)
        {
            let buffer = [modifier as u8, 0, code, 0 ,0 ,0 ,0 ,0u8];
            self.write_to_file(&buffer)?;
        }
        Ok(())
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

    /// Read exact while waiting for a 'character' to be pressed
    /// 
    /// The buffer: [1,0,0,0,0,0,0,0], would be that only the 'control key' is pressed and that can't be used to get a password
    /// and thus won't be returned by this function.
    /// 
    /// The buffer: [0,0,4,0,0,0,0,0], would be that only the 'a' is pressed and that can be used so will exit the internal loop
    pub fn read_next_character(&mut self) -> std::io::Result<Keypress>
    {
        while let key = self.next_key()?
        {
            if let Keypress::Character(c) = key
            {
                return Ok(Keypress::Character(c))
            }
        }
        Ok(Keypress::None)
    }

    pub fn next_key(&mut self) -> std::io::Result<Keypress>
    {
        loop
        {
            if let Some(key) = self.character_queue.pop_front()
            {
                return Ok(key)
            }
            self.read_exact()?;
        }
    }

    fn read_exact(&mut self) -> std::io::Result<()>
    {
        let old_buffer = self.buffer.clone();
        self.reader.read_exact(&mut self.buffer)?;
        self.converter.report_to_keypress(&mut self.character_queue, &self.buffer, &old_buffer);
        Ok(())
    }

    pub fn read_line(&mut self) -> Result<String, DataHandleError> 
    {
        let mut result_string = String::new();
        loop
        {
            let key = self.next_key()?;
            println!("{:?}", key);
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