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
        let mut converter = Converter::new();
        converter.add_macro(&settings.macro_key);
        let iohelper = IOhelper::new(reader, writer, Converter::new());
        Ok(App { iohelper, data, settings})
    }

    pub fn main_loop(&mut self) -> Result<(), DataHandleError>
    {
        // Load the combo you want to use
        loop
        {
            self.pass_through_loop()?;

            // Read next input so it can be checked against the command key
            let key = self.iohelper.next_key()?;

            // If the next input is the command key again, it will send the command key through to the computer
            // Else we will enter a loop where we collect the characters pressed, into the macro string, untill the command key is pressed again
            if Keypress::Macro != key
            {
                self.iohelper.write_to_file(&self.settings.macro_key)?;
            }
            else
            {
                let command = self.iohelper.read_line()?;

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

    fn pass_through_loop(&mut self) -> Result<(), DataHandleError>
    {
        let mut key = self.iohelper.next_key()?;
        while Keypress::Macro != key
        {
            self.iohelper.write_key(key)?;
            key = self.iohelper.next_key()?;
            println!("{:?}", key);
        }
        println!("exit loop");
        Ok(())
    }

    fn action_use_macro(&mut self, command: &str) -> Result<(), DataHandleError>
    {
        if let Some(blob) = self.data.decrypt_blob(&command)
        {
            let buffers= self.iohelper.converter.string_to_report_buffers(&String::from_utf8(blob?)?);
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
}
