mod app;
mod converterutilities;
mod iohelper;

use app::{App, AppSettings};
use iohelper::IOhelper;

use std::env::current_dir;
use std::fs::{File, OpenOptions};
use std::io::Read;

use fobword_core::config::{Config, DataInformation, LockedData};
use fobword_core::error::DataHandleError;

fn main() -> Result<(), DataHandleError>
{
    let mut app = match OpenOptions::new().read(true).open("/home/pi/config.yaml")
    {
        Ok(mut file) => existing_config(&mut file)?,
        Err(e) => {
            match e.kind()
            {
                std::io::ErrorKind::NotFound => default_config(),
                _ => Err(DataHandleError::IOError(e)),
            }?
        }
    };
    app.main_loop()?;
    Ok(())
}

fn existing_config(file: &mut File) -> Result<App, DataHandleError>
{
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let config = Config::from_yaml(&buffer)?;
    App::new(config.settings.unwrap(), config.data.unwrap())
}

fn default_config() -> Result<App, DataHandleError>
{
    let default_settings = 
        AppSettings { 
            input: "/dev/hidraw2".to_owned(), 
            output: "/dev/hidg0".to_owned(),
            macro_key: vec![0x02, 0, 0x3au8, 0, 0, 0, 0, 0,]};
    let default_password = "password";
    let mut locked_data = LockedData::new(default_password)?;
    let mut data = locked_data.unlock(default_password)?;
    data.insert(String::from("test"), DataInformation::new(None, None, String::from("some_password")));
    locked_data.lock(default_password, data)?;
    App::new(default_settings, locked_data)
}