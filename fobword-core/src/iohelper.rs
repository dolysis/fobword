use std::io::{BufRead, Write, BufReader};
use std::fs::OpenOptions;
use std::path::Path;


#[allow(missing_debug_implementations)]
/// Struct that contains helper functions to simplify reading and writing HID reports
pub struct IOhelper
{
    /// The reader from which HID reports will be read
    pub reader: Box<dyn BufRead>,
    /// The writer to which HID reports will be written
    pub writer: Box<dyn Write>,
}

impl IOhelper
{
    /// Create a new helper from stuff
    pub(crate) fn new(reader: Box<dyn BufRead>, writer: Box<dyn Write>) -> IOhelper
    {
        IOhelper { reader, writer}
    }
    
    /// Create a new helper from with the location to read from, and to write buffers to.
    pub fn from_paths(reader_loc: &Path, writer_loc: &Path) -> std::io::Result<IOhelper>
    {
        let reader: Box<dyn BufRead> = Box::new(BufReader::new(OpenOptions::new().read(true).open(reader_loc)?));
        let writer: Box<dyn Write> = Box::new(OpenOptions::new().write(true).open(writer_loc)?);

        Ok(IOhelper { reader, writer})
    }

    /// Read exact while waiting for a 'character' to be pressed
    /// 
    /// The buffer: [1,0,0,0,0,0,0,0], would be that only the 'control key' is pressed and that can't be used to get a password
    /// and thus won't be returned by this function.
    /// 
    /// The buffer: [0,0,4,0,0,0,0,0], would be that only the 'a' is pressed and that can be used so will exit the internal loop
    pub fn read_next_character(&mut self, buffer: &mut [u8]) -> std::io::Result<()>
    {
        self.reader.read_exact(buffer)?;
        // If the third index of buffer is 0, it means no character key has been pressed
        while buffer[2] == 0
        {
            self.reader.read_exact(buffer)?;
        }
        Ok(())
    }

    /// Write a single buffer to the file, flushing it to ensure the data will be pushed to the file
    pub fn write_to_file(&mut self, buffer: &[u8]) -> std::io::Result<()>
    {
        //self.writer.write(buffer)?;
        //self.writer.flush()?;
        println!("{:?}", buffer);
        Ok(())
    }

    /// Write multiple buffers to the file, flushing between every buffer, as to simulate a single HID report
    pub fn write_buffers_to_file(&mut self, buffers: Vec<Vec<u8>>) -> std::io::Result<()>
    {
        for i in buffers
        {
            self.writer.write(&i)?;
            self.writer.flush()?;
        }
        Ok(())
    }
}

