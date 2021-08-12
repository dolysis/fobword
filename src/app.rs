use fobword_core::error::DataHandleError;
use fobword_core::converter::{Converter, Keypress, Modifier};
use fobword_core::config::{Config, Data};
use std::io::{BufRead, BufReader, Write};
use std::fs::OpenOptions;
use serde::{Serialize, Deserialize};

use super::*;

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

        let mut converter = Converter::default();
        converter.add_macro(Modifier::from(settings.macro_key[0]), settings.macro_key[2]);

        let reader: Box<dyn BufRead> = Box::new(BufReader::new(OpenOptions::new().read(true).open(&settings.input)?));
        let writer: Box<dyn Write> = Box::new(OpenOptions::new().write(true).open(&settings.output)?);
        let iohelper = IOhelper::new(reader, writer);

        Ok(App { iohelper , converter, data, settings})
    }

    pub fn main_loop(&mut self) -> Result<(), DataHandleError>
    {
        loop
        {
            self.passthrough_loop()?;

            println!("Macro teritory now haha yes");
            // Read next input so it can be checked against the command key
            
            let command = self.iohelper.read_line(&self.converter)?;
            println!("{:?}", command);
            match command.as_ref()
            {
                "" => self.iohelper.write_key(&Keypress::Macro, &self.converter)?,
                "save" => self.action_save_data()?,
                "exit" => break,
                _ => self.action_use_macro(&command)?,
            }
        }
        Ok(())
    }

    fn passthrough_loop(&mut self) -> Result<(), DataHandleError>
    {
        let mut key = self.iohelper.next_key(&self.converter)?;
        while key != Keypress::Macro
        {
            self.iohelper.write_key(&key, &self.converter)?;
            key = self.iohelper.next_key(&self.converter)?;
        }
        Ok(())
    }


    fn action_use_macro(&mut self, command: &str) -> Result<(), DataHandleError>
    {
        if let Some(blob) = self.data.decrypt_blob(&command)
        {
            let buffers= converterutilities::string_to_report_buffers(&self.converter, &String::from_utf8(blob?)?);
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
        let pass = self.iohelper.read_line(&self.converter)?;
        println!("{:?} pass", pass);
        let buffer = config.to_yaml("harry")?;

        file.write_all(buffer.as_bytes())?;
        Ok(())

    }
}
