use std::fs::{File, OpenOptions};
use std::io::Read;

use fobword_core::app::{App, AppSettings};
use fobword_core::config::{Config, Data};
use fobword_core::error::DataHandleError;

fn main() -> Result<(), DataHandleError>
{
    let config = match OpenOptions::new().read(true).open("/home/pi/config.yaml")
    {
        Ok(mut file) => config_exists(&mut file)?,
        Err(e) => {
            match e.kind()
            {
                std::io::ErrorKind::NotFound => default_config(),
                _ => Err(DataHandleError::IOError(e)),
            }?
        }
    };
    let mut app = App::new(config)?;
    app.main_loop()?;
    Ok(())
}

fn config_exists(file: &mut File) -> Result<Config<AppSettings>, DataHandleError>
{
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Config::from_yaml(&buffer).or_else(|_| default_config())
}

fn default_config() -> Result<Config<AppSettings>, DataHandleError>
{
    let settings = AppSettings { input: "/dev/hidraw0".to_owned(), output: "/dev/hidg0".to_owned(), macro_key: vec![0x01, 0, 0x38u8, 0, 0, 0, 0, 0,]};
    let mut data = Data::new();
    data.insert("ff14".to_owned(), "Squ3r33n1xP4ssW0rd".to_owned(), None, None)?;
    Ok(Config::new(Some(settings), Some(data)))
}