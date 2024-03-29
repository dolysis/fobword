use fobword_core::config::{Config, Data, LockedData, SymbolLevel};
use fobword_core::converter::{Converter, Key, Modifier};
use fobword_core::error::DataHandleError;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use SSD1306_Terminal::window::Window;

use super::*;

pub struct App {
    iohelper: IOhelper,
    data: LockedData,
    settings: AppSettings,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub output: String,
    pub macro_key: Vec<u8>,
    pub output_location: String,
    pub input_location: String,
}

impl App {
    pub fn new(settings: AppSettings, data: LockedData) -> Result<App, DataHandleError> {
        let data = data;
        let settings = settings;
        let mut converter = Converter::from_paths(&settings.input_location, &settings.output_location)?;
        converter.add_macro(Modifier::from(settings.macro_key[0]), settings.macro_key[2]);
        let window = Window::new("/dev/i2c-0", 0x3c)?;
        let iohelper = IOhelper::new(&settings.output, converter, window)?;

        Ok(App {
            iohelper,
            data,
            settings,
        })
    }

    pub fn main_loop(&mut self) -> Result<(), DataHandleError> {
        'outer: loop {
            self.iohelper.screen_off()?;
            self.iohelper.wait_for(Key::Macro)?;
            self.iohelper.screen_on()?;

            self.iohelper.window
                .print_to_buffer("Please enter device password:")?;
            let mut data = match self.data.unlock(&self.iohelper.read_line()?) {
                Ok(value) => value,
                Err(DataHandleError::ArgonError(password_hash::Error::Password)) => {
                    println!("Invalid password");
                    continue;
                }
                Err(error) => return Err(error),
            };

            'inner: loop {
                let command = self.iohelper.read_line()?;
                match command.as_ref() {
                    "" => { self.iohelper.write_key(&Key::Macro)?; },
                    "lock" => break 'inner,
                    // Hint - print the hint/comment
                    "new" => self.action_create_macro(&mut data)?,
                    "save" => self.action_save_data(&mut data)?,
                    "exit" => break 'outer,
                    "change" => self.action_change_password()?,
                    "gen" | "generate" => self.action_generate_password(&mut data)?,
                    _ => self.action_use_macro(&data, &command)?,
                }

                self.iohelper.screen_off()?;
                self.iohelper.wait_for(Key::Macro)?;
                self.iohelper.screen_on();
            }
            self.iohelper.screen_off()?;
        }
        Ok(())
    }

    fn action_generate_password(&mut self, data: &mut Data) -> Result<(), DataHandleError> 
    {
        self.iohelper.println("Enter macro name:")?;
        let name = self.iohelper.read_line()?;
        data.generate(name, None, 15, SymbolLevel::Symbols)
    }

    fn action_change_password(&mut self) -> Result<(), DataHandleError> 
    {
        self.iohelper.clear_screen();
        self.iohelper.println("Type old password:")?;
        let old_password = self.iohelper.read_line()?;

        self.iohelper.println("Type new password:")?;
        let new_password = self.iohelper.read_line()?;

        self.iohelper.println("Confirm password:")?;
        if self.iohelper.read_line()? == new_password {
            return self.data.change_password(&old_password, &new_password);
        }
        self.iohelper.println("The passwords do not match")?;
        Ok(())
    }

    fn action_create_macro(&mut self, data: &mut Data) -> Result<(), DataHandleError> {
        let name = self.iohelper.read_line()?;
        let pass = self.iohelper.read_line()?;
        data.insert(name, DataInformation::new(None, None, pass));
        Ok(())
    }

    fn action_use_macro(&mut self, data: &Data, command: &str) -> Result<(), DataHandleError> {
        if let Some(information) = data.get(command) 
        {
            let buffers =
                converterutilities::string_to_report_buffers(&self.iohelper.converter, &information.blob);
            if let Some(buffers) = buffers 
            {
                self.iohelper.write_buffers_to_file(buffers)?;
            }
        } 
        else 
        {
            self.iohelper.println("No macro found with that name.")?;
        }
        Ok(())
    }

    fn action_save_data(&mut self, data: &mut Data) -> Result<(), DataHandleError> 
    {
        if std::path::Path::new("/usr/bin/config.yaml").exists()
        {
            std::fs::copy("/usr/bin/config.yaml", "/usr/bin/back-up-config.yaml")?;
        }
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("/usr/bin/config.yaml")?;
        let password = self.iohelper.read_line()?;
        self.data.lock(&password, data.clone())?;
        let config = Config::new(Some(self.settings.clone()), Some(self.data.clone()));
        let buffer = config.to_yaml()?;

        file.write_all(buffer.as_bytes())?;
        Ok(())
    }
}
