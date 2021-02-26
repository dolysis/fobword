use std::io;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum FobError
{
    ConvertError(String),
    IoError(io::Error),
}

impl fmt::Display for FobError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            FobError::ConvertError(message) => write!(f, "Error converting character: {}, into hid report code", message),
            FobError::IoError(err) => write!(f, "{}", err),
        }
    }
}

impl Error for FobError
{
    fn source(&self) -> Option<&(dyn Error + 'static)>
    {
        match self
        {
            FobError::ConvertError(_) => None,
            FobError::IoError(cause) => Some(cause)
        }
    }
}

impl From<io::Error> for FobError
{
    fn from(error: io::Error) -> Self
    {
        FobError::IoError(error)
    }
}