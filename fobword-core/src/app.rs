
use crate::db::Db;

use super::converter::{Converter, Keypress};
use super::yaml_config::Config;
use super::iohelper::IOhelper;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Write, Result};
use std::fs::OpenOptions;

pub struct App
{
    iohelper: IOhelper,
    macro_key: Vec<u8>,
    converter: Converter,
    database: Db,
}

impl App
{
    pub fn new(config: Config) -> Result<App>
    {
        let reader: Box<dyn BufRead> = Box::new(BufReader::new(OpenOptions::new().read(true).open(config.reader_loc)?));
        let writer: Box<dyn Write> = Box::new(OpenOptions::new().write(true).open(config.writer_loc)?);
        let iohelper = IOhelper::new(reader, writer);
        let database = Db::new("/home/pi/macro.db3");
        Ok(App { iohelper , macro_key: config.macro_key, converter: Converter::new(), database})
    }

    pub fn main_loop(&mut self) -> Result<()>
    {
        // Load the combo you want to use
        let mut buffer = vec![0u8; 8];
        loop
        {
            self.iohelper.reader.read_exact(&mut buffer)?;
            // Pass the inputs to /dev/hidg0 (to send them to the computer) while waiting for the command key
            while buffer != self.macro_key
            {
                self.iohelper.write_to_file(&buffer)?;
                self.iohelper.reader.read_exact(&mut buffer)?;
            }

            // Read next input so it can be checked against the command key
            self.iohelper.read_next_character(&mut buffer)?;

            // If the next input is the command key again, it will send the command key through to the computer
            // Else we will enter a loop where we collect the characters pressed, into the macro string, untill the command key is pressed again
            // Then we try to query the database for the password and, if found, send it to the computer
            if buffer == self.macro_key
            {
                self.iohelper.write_to_file(&self.macro_key)?;
            }
            else
            {
                let command = self.read_line(&mut buffer)?;

                println!("{:?}", command);
                match command.as_ref()
                {
                    "make" | "create" => self.make_new_macro()?,
                    "delete" | "remove" => println!("delete"),
                    "update" => println!("update"),
                    "exit" | "exterminate" => break,
                    "harry" => {self.iohelper.write_buffers_to_file(self.converter.string_to_report_buffers("potter").unwrap())?; ()},
                    _ => println!("use the macro"),
                };
            }
        }
        Ok(())
    }

    fn make_new_macro(&mut self) -> Result<()>
    {
        let mut buffer = vec![0u8;8];
        let new_command = self.read_line(&mut buffer)?;
        let password = self.read_line(&mut buffer)?;
        let commands = self.database.load_macros().unwrap();

        

        Ok(())
    }


    fn read_line(&mut self, buffer: &mut Vec<u8>) -> Result<String> 
    {
        let mut result_string = String::new();
        let mut character_queue = VecDeque::new();
        self.converter.report_to_keypress(&mut character_queue, &buffer, &vec![0u8;8]);
        'read_character_loop: loop
        {
            match character_queue.pop_front()
            {
                //If character queue has some key
                Some(key) => match key
                {
                    Keypress::Character(character) => result_string.push(character),
                    Keypress::Enter => break 'read_character_loop,
                    Keypress::Macro => break 'read_character_loop,
                    Keypress::None => continue,
                },
                // If there is no key in the queue, read new input and append to the queue
                None => 
                {                            
                    let old_buffer = buffer.clone();
                    self.iohelper.reader.read_exact(buffer)?;
                    self.converter.report_to_keypress(&mut character_queue, &buffer, &old_buffer);
                }
            }
        }
        Ok(result_string)
    }
}
