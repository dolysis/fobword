use super::converter::{Converter, Keypress};
use super::yaml_config::Config;
use super::iohelper::IOhelper;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Write};
use std::fs::OpenOptions;

pub struct App
{
    iohelper: IOhelper,
    macro_key: Vec<u8>,
}

impl App
{
    pub fn new(config: Config) -> std::io::Result<App>
    {
        let reader: Box<dyn BufRead> = Box::new(BufReader::new(OpenOptions::new().read(true).open(config.reader_loc)?));
        let writer: Box<dyn Write> = Box::new(std::io::stdout());
        let iohelper = IOhelper::new(reader, writer);
        Ok(App { iohelper , macro_key: config.macro_key})
    }

    pub fn main_loop(&mut self) -> std::io::Result<()>
    {
        let converter = Converter::new();
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
                let mut command = String::new();
                let mut queue = VecDeque::new();

                loop
                {
                    // If the there is nothing left in the queue and we haven't left the loop we wait for the next keypress
                    match queue.pop_front()
                    {
                        Some(v) => match v
                        {
                            Keypress::Character(c) => command.push(c),
                            Keypress::Enter => println!("{}", command),
                            Keypress::Macro => break,
                            Keypress::None => continue,
                        },
                        None => 
                        {                            
                            let old_buffer = buffer.clone();
                            self.iohelper.read_next_character(&mut buffer)?;
                            converter.report_to_keypress(&mut queue, &buffer, &old_buffer);
                        }
                    }
                }

                println!("{:?}", command);
                match command.as_ref()
                {
                    "make" | "create" => println!("make"),
                    "delete" | "remove" => println!("delete"),
                    "update" => println!("update"),
                    "exit" | "exterminate" => break,
                    _ => println!("use the macro"),
                };
            }
        }
        Ok(())
    }
}

// read_word() -> String?

// Report can contain up to 6 characters
// queue? 
// Queue<Enum>>
// Enum -> Char, control, none
// while Enum::Control(buffer) != queue.pop()?
// 