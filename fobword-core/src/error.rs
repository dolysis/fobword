#[derive(Debug)]
pub enum DataHandleError
{
    IOError(std::io::Error),
    AesError(aes_gcm::Error),
    ArgonError(password_hash::Error),
    B64ParseError(base64::DecodeError),
    YamlParseError(serde_yaml::Error),
    LockedData(String),
    FromUt8Error(std::string::FromUtf8Error)
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