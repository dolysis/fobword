use std::sync::mpsc::RecvError;

/// Convience Error type to handle the different errors from exterior crates
#[derive(Debug)]
pub enum DataHandleError
{
    // https://doc.rust-lang.org/std/io/struct.Error.html
    IOError(std::io::Error),

    // https://docs.rs/aes-gcm/0.9.3/aes_gcm/struct.Error.html
    AesError(aes_gcm::Error),

    // https://docs.rs/argon2/0.2.2/argon2/enum.Error.html
    ArgonError(password_hash::Error),

    // https://docs.rs/base64/0.13.0/base64/enum.DecodeError.html
    B64ParseError(base64::DecodeError),

    // https://docs.serde.rs/serde_yaml/struct.Error.html
    YamlParseError(serde_yaml::Error),

    // https://doc.rust-lang.org/std/string/struct.FromUtf8Error.html
    FromUt8Error(std::string::FromUtf8Error),

    // Error for when trying to access locked data that holds message for debug
    LockedData(String),

    // Channel recvier error
    RecvError(std::sync::mpsc::RecvError)
}

impl From<std::sync::mpsc::RecvError> for DataHandleError
{
    fn from(err: std::sync::mpsc::RecvError) -> DataHandleError
    {
        DataHandleError::RecvError(err)
    }
}

impl From<std::string::FromUtf8Error> for DataHandleError
{
    fn from(err: std::string::FromUtf8Error) -> DataHandleError
    {
        DataHandleError::FromUt8Error(err)
    }
}

impl From<std::io::Error> for DataHandleError
{
    fn from(err: std::io::Error) -> DataHandleError
    {
        DataHandleError::IOError(err)
    }
}

impl From<aes_gcm::Error> for DataHandleError
{
    fn from(err: aes_gcm::Error) -> DataHandleError
    {
        DataHandleError::AesError(err)
    }
}

impl From<base64::DecodeError> for DataHandleError
{
    fn from(err: base64::DecodeError) -> DataHandleError
    {
        DataHandleError::B64ParseError(err)
    }
}

impl From<password_hash::Error> for DataHandleError
{
    fn from(err: password_hash::Error) -> DataHandleError
    {
        DataHandleError::ArgonError(err)
    }
}

impl From<serde_yaml::Error> for DataHandleError
{
    fn from(err: serde_yaml::Error) -> DataHandleError
    {
        DataHandleError::YamlParseError(err)
    }
}