
#[derive(Debug, PartialEq)]
/// The struct that holds a macros data
pub struct Command
{
    /// Name of the macro
    pub name: String,
    /// Password of the macro
    pub password: String,
}

impl Command
{
    pub fn new(name: String, password: String) -> Command
    {
        Command { name, password }
    }
}