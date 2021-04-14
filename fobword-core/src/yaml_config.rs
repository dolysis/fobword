use std::path::PathBuf;


/// Struct to hold the configuration from yaml
#[derive(Debug)]
pub struct Config
{
    /// Location where to read HID reports from
    pub reader_loc: PathBuf,
    /// Location where to write HID reports too
    pub writer_loc: PathBuf,
    /// The macro key which is used to intercept the inputs and read them for the macro
    pub macro_key: Vec<u8>,
}

impl Config
{
    /// Load config settings from the specified config file location
    pub fn new() -> Config
    {
        let reader_loc = PathBuf::from("/dev/hidraw0");
        let writer_loc = PathBuf::from("/dev/hidg0");
        let macro_key = vec![1, 0, 19, 0, 0, 0, 0, 0,];
        Config { reader_loc, writer_loc, macro_key}
    }

    /// Default config settings for fobword
    pub fn default() -> Config
    {
        let reader_loc = PathBuf::from("/dev/hidraw0");
        let writer_loc = PathBuf::from("/dev/hidg0");
        let macro_key = vec![1, 0, 19, 0, 0, 0, 0, 0,];
        Config { reader_loc, writer_loc, macro_key}
    }
}