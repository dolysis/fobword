use crate::config::Data;
use crate::error::DataHandleError;

use super::converter::{Converter, Keypress};
use super::config::Config;
use super::iohelper::IOhelper;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Write};
use std::fs::OpenOptions;
use serde::{Serialize, Deserialize};

pub struct App
{
    iohelper: IOhelper,
    converter: Converter,
    data: Data,
    settings: AppSettings,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings
{
    pub input: String,
    pub output: String,
    pub macro_key: Vec<u8>
}

impl App
{
    pub fn new(config: Config<AppSettings>) -> Result<App, DataHandleError>
    {
        let data = config.data.unwrap();
        let settings = config.settings.unwrap();
        let reader: Box<dyn BufRead> = Box::new(BufReader::new(OpenOptions::new().read(true).open(&settings.input)?));
        let writer: Box<dyn Write> = Box::new(OpenOptions::new().write(true).open(&settings.output)?);
        let iohelper = IOhelper::new(reader, writer, Converter::new());
        Ok(App { iohelper , converter: Converter::new(), data, settings})
    }

    pub fn main_loop(&mut self) -> Result<(), DataHandleError>
    {
        // Load the combo you want to use
        let mut buffer = vec![0u8; 8];
        loop
        {
            self.iohelper.reader.read_exact(&mut buffer)?;
            // Pass the inputs to /dev/hidg0 (to send them to the computer) while waiting for the command key
            while buffer != self.settings.macro_key
            {
                self.iohelper.write_to_file(&buffer)?;
                self.iohelper.reader.read_exact(&mut buffer)?;
            }

            // Read next input so it can be checked against the command key
            self.iohelper.read_next_character(&mut buffer)?;

            // If the next input is the command key again, it will send the command key through to the computer
            // Else we will enter a loop where we collect the characters pressed, into the macro string, untill the command key is pressed again
            if buffer == self.settings.macro_key
            {
                self.iohelper.write_to_file(&self.settings.macro_key)?;
            }
            else
            {
                let command = self.read_line(&mut buffer)?;

                println!("{:?}", command);
                match command.as_ref()
                {
                    "exit" | "exterminate" => break,
                    "save" => self.action_save_data()?,
                    _ => self.action_use_macro(&command)?,
                };
            }
        }
        Ok(())
    }

    fn action_use_macro(&mut self, command: &str) -> Result<(), DataHandleError>
    {
        if let Some(blob) = self.data.decrypt_blob(&command)
        {
            let buffers= self.converter.string_to_report_buffers(&String::from_utf8(blob?)?);
            if let Some(buffers) = buffers
            {
                self.iohelper.write_buffers_to_file(buffers)?;
            }
        }
        Ok(())
    }

    fn action_save_data(&mut self) -> Result<(), DataHandleError>
    {
        std::fs::copy("/home/pi/config.yaml", "/home/pi/back-up-config.yaml")?;
        let mut file =  OpenOptions::new().write(true).truncate(true).open("/home/pi/config.yaml")?;
        let mut config = Config::new(Some(self.settings.clone()), Some(self.data.clone()));
        let buffer = config.to_yaml("harry")?;

        file.write_all(buffer.as_bytes())?;
        Ok(())

    }

    fn read_line(&mut self, buffer: &mut Vec<u8>) -> Result<String, DataHandleError> 
    {
        let mut result_string = String::new();
        let mut character_queue = VecDeque::new();
        let mut old_buffer = vec![0u8;8];
        loop
        {
            self.converter.report_to_keypress(&mut character_queue, &buffer, &old_buffer);

            while let Some(key) = character_queue.pop_front()
            {
                println!("{:?}", key);
                match key
                {
                    Keypress::Character(character) => result_string.push(character),
                    Keypress::Enter => return Ok(result_string),
                    Keypress::Macro => return Ok(result_string),
                    Keypress::None => continue,
                }
            }

            old_buffer = buffer.clone();
            self.iohelper.reader.read_exact(buffer)?;
        }
    }
}
