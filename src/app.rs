use fobword_core::config::{Config, Data, LockedData};
use fobword_core::converter::{Converter, Key, Modifier};
use fobword_core::error::DataHandleError;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use SSD1306_Terminal::window::Window;

use super::*;

pub struct App {
    iohelper: IOhelper,
    converter: Converter,
    data: LockedData,
    settings: AppSettings,
    window: Window,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub input: String,
    pub output: String,
    pub macro_key: Vec<u8>,
}

impl App {
    pub fn new(settings: AppSettings, data: LockedData) -> Result<App, DataHandleError> {
        let data = data;
        let settings = settings;
        let mut converter = Converter::default();
        converter.add_macro(Modifier::from(settings.macro_key[0]), settings.macro_key[2]);

        let reader: Box<dyn BufRead> = Box::new(BufReader::new(
            OpenOptions::new().read(true).open(&settings.input)?,
        ));
        let writer: Box<dyn Write> =
            Box::new(OpenOptions::new().write(true).open(&settings.output)?);
        let iohelper = IOhelper::new(reader, writer);

        let window = Window::new("/dev/i2c-0", 0x3c)?;

        Ok(App {
            iohelper,
            converter,
            data,
            settings,
            window,
        })
    }

    pub fn main_loop(&mut self) -> Result<(), DataHandleError> {
        'outer: loop {
            self.passthrough_loop()?;

            self.window
                .print_to_buffer("Please enter device password:")?;
            let mut data = match self.data.unlock(&self.iohelper.read_line(&self.converter, &mut self.window)?) {
                Ok(value) => value,
                Err(DataHandleError::ArgonError(password_hash::Error::Password)) => {
                    println!("Invalid password");
                    continue;
                }
                Err(error) => return Err(error),
            };

            'inner: loop {
                let command = self.iohelper.read_line(&self.converter, &mut self.window)?;
                match command.as_ref() {
                    "" => self.iohelper.write_key(&Key::Macro, &self.converter)?,
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

    fn passthrough_loop(&mut self) -> Result<(), DataHandleError> {
        self.window.sleep()?;
        let raw_macro = self.converter.get_raw(&Key::Macro);
        let macro_buffer = [raw_macro.0 as u8, 0, raw_macro.1, 0, 0, 0, 0, 0];
        let mut buffer = [0u8; 8];
        while macro_buffer != buffer {
            self.iohelper.write_to_file(&buffer)?;
            self.iohelper.reader.read_exact(&mut buffer)?;
        }
        self.window.awaken()?;
        Ok(())
    }

    fn action_change_macro(&mut self) -> Result<(), DataHandleError> {
        let key = self.iohelper.next_key(&self.converter)?;
        let raw_key = self.converter.get_raw(&key);
        self.converter.add_macro(raw_key.0, raw_key.1);
        self.settings.macro_key = vec![raw_key.0 as u8, 0, raw_key.1, 0, 0, 0, 0, 0];
        Ok(())
    }

    fn action_change_password(&mut self) -> Result<(), DataHandleError> {
        println!("Type old password");
        let old_password = self.iohelper.read_line(&self.converter, &mut self.window)?;

        println!("Type new password");
        let new_password = self.iohelper.read_line(&self.converter, &mut self.window)?;

        println!("Type new password to confirm");
        if self.iohelper.read_line(&self.converter, &mut self.window)? == new_password {
            return self.data.change_password(&old_password, &new_password);
        }
        println!("Passwords do not match");
        Ok(())
    }

    fn action_create_macro(&mut self, data: &mut Data) -> Result<(), DataHandleError> {
        let name = self.iohelper.read_line(&self.converter, &mut self.window)?;
        let pass = self.iohelper.read_line(&self.converter, &mut self.window)?;
        data.insert(name, DataInformation::new(None, None, pass));
        Ok(())
    }

    fn action_use_macro(&mut self, data: &Data, command: &str) -> Result<(), DataHandleError> {
        if let Some(information) = data.get(command) {
            let buffers =
                converterutilities::string_to_report_buffers(&self.converter, &information.blob);
            if let Some(buffers) = buffers {
                self.iohelper.write_buffers_to_file(buffers)?;
            }
        } else {
            println!("No data found.")
        }
        Ok(())
    }

    fn action_save_data(&mut self, data: &mut Data) -> Result<(), DataHandleError> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("/home/pi/config.yaml")?;
        std::fs::copy("/home/pi/config.yaml", "/home/pi/back-up-config.yaml")?;
        let password = self.iohelper.read_line(&self.converter, &mut self.window)?;
        self.data.lock(&password, data.clone())?;
        let config = Config::new(Some(self.settings.clone()), Some(self.data.clone()));
        let buffer = config.to_yaml()?;

        file.write_all(buffer.as_bytes())?;
        Ok(())
    }
}
