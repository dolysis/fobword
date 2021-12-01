use fobword_core::error::DataHandleError;
use fobword_core::converter::{Converter, Keypress, Modifier};
use fobword_core::config::{Config, Data, LockedData};
use std::io::{BufRead, BufReader, Write};
use std::fs::OpenOptions;
use serde::{Serialize, Deserialize};

use super::*;

pub struct App
{
    iohelper: IOhelper,
    converter: Converter,
    data: LockedData,
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
    pub fn new(settings: AppSettings, data: LockedData) -> Result<App, DataHandleError>
    {
        let data = data;
        let settings = settings;

        let mut converter = Converter::default();
        converter.add_macro(Modifier::from(settings.macro_key[0]), settings.macro_key[2]);

        let reader: Box<dyn BufRead> = Box::new(BufReader::new(OpenOptions::new().read(true).open(&settings.input)?));
        let writer: Box<dyn Write> = Box::new(OpenOptions::new().write(true).open(&settings.output)?);
        let iohelper = IOhelper::new(reader, writer);

        Ok(App { iohelper , converter, data, settings})
    }


    pub fn main_loop(&mut self) -> Result<(), DataHandleError>
    {
        'outer: loop 
        {      
            self.passthrough_loop()?;

            println!("Please enter password to unlock");
            let mut data = match self.data.unlock(&self.iohelper.read_line(&self.converter)?)
            {
                Ok(value) => value,
                Err(DataHandleError::ArgonError(password_hash::Error::Password)) => { println!("Invalid password"); continue;},
                Err(error) => return Err(error),
            };

            'inner: loop
            {
                let command = self.iohelper.read_line(&self.converter)?;
                println!("{}", command);
                match command.as_ref()
                {
                    "" => self.iohelper.write_key(&Keypress::Macro, &self.converter)?,
                    "lock" => break 'inner,
                    "new" => self.action_create_macro(&mut data)?,
                    "save" => self.action_save_data(&mut data)?,
                    "exit" => break 'outer,
                    "change" => self.action_change_password()?,
                    "macro" => self.action_change_macro()?,
                    _ => self.action_use_macro(&data, &command)?,
                }

                self.passthrough_loop()?;
            }
        }
        Ok(())
    }

    fn passthrough_loop(&mut self) -> Result<(), DataHandleError>
    {
        let raw_macro = self.converter.convert_keypress(&Keypress::Macro);
        let macro_buffer = [raw_macro.0 as u8, 0 , raw_macro.1, 0 ,0 ,0,0,0];
        let mut buffer = [0u8;8];
        while macro_buffer != buffer
        {
            self.iohelper.write_to_file(&buffer)?;
            self.iohelper.reader.read_exact(&mut buffer)?;
        }
        Ok(())
    }

    fn action_change_macro(&mut self) -> Result<(), DataHandleError>
    {
        let key = self.iohelper.next_key(&self.converter)?;
        let raw_key = self.converter.convert_keypress(&key);
        self.converter.add_macro(raw_key.0, raw_key.1);
        self.settings.macro_key = vec![raw_key.0 as u8, 0, raw_key.1, 0, 0, 0, 0 ,0];
        Ok(())
    }

    fn action_change_password(&mut self) -> Result<(), DataHandleError>
    {
        println!("Type old password");
        let old_password = self.iohelper.read_line(&self.converter)?;

        println!("Type new password");
        let new_password = self.iohelper.read_line(&self.converter)?;
        
        println!("Type new password to confirm");
        if self.iohelper.read_line(&self.converter)? == new_password
        {
            return self.data.change_password(&old_password, &new_password)
        }
        println!("Passwords do not match");
        Ok(())
    }

    fn action_create_macro(&mut self, data: &mut Data) -> Result<(), DataHandleError>
    {
        let name = self.iohelper.read_line(&self.converter)?;
        let pass = self.iohelper.read_line(&self.converter)?;
        data.insert(name, DataInformation::new(None, None, pass));
        Ok(())
    }

    fn action_use_macro(&mut self, data: &Data, command: &str) -> Result<(), DataHandleError>
    {
        if let Some(information) = data.get(command)
        {
            let buffers= converterutilities::string_to_report_buffers(&self.converter, &information.blob);
            if let Some(buffers) = buffers
            {
                self.iohelper.write_buffers_to_file(buffers)?;
            }
        }
        else 
        {
            println!("No data found.")    
        }
        Ok(())
    }

    fn action_save_data(&mut self, data: &mut Data) -> Result<(), DataHandleError>
    {        
        let mut file =  OpenOptions::new().write(true).create(true).truncate(true).open("/home/pi/config.yaml")?;
        std::fs::copy("/home/pi/config.yaml", "/home/pi/back-up-config.yaml")?;
        let password = self.iohelper.read_line(&self.converter)?;
        self.data.lock(&password, data.clone())?;
        let config = Config::new(Some(self.settings.clone()), Some(self.data.clone()));
        let buffer = config.to_yaml()?;

        file.write_all(buffer.as_bytes())?;
        Ok(())

    }
}
