use std::collections::VecDeque;
use std::fs::{OpenOptions, File};
use std::io::{Read, Write, self};
use SSD1306_Terminal::window::{Window, self};
use fobword_core::converter::{Converter, Key};
use fobword_core::error::DataHandleError;
use std::sync::mpsc::{self, RecvError, SendError, Sender, Receiver};
use libc; // 0.2.66
use nix::ioctl_read; // 0.16.1
use std::{
    mem::MaybeUninit,
    os::unix::{fs::OpenOptionsExt, io::AsRawFd},
};
use std::thread::spawn;
use fobword_core::converter::*;

use crate::converterutilities;

ioctl_read!(hid_read_sz, b'H', 0x01, libc::c_int);
ioctl_read!(hid_read_descr, b'H', 0x02, hidraw_report_descriptor);

const HID_MAX_DESCRIPTOR_SIZE: usize = 4096;

#[repr(C)]
pub struct hidraw_report_descriptor {
    size: u32,
    value: [u8; HID_MAX_DESCRIPTOR_SIZE],
}


#[derive(Debug, PartialEq)]
pub enum Events
{
    ModifierUp(u8),
    ModifierDown(u8),
    KeyUp(u8),
    KeyDown(u8),
}

/// Struct that contains helper functions to simplify reading and writing HID reports
pub struct IOhelper
{
    /// The writer to which HID reports will be written
    pub output_file: File,

    receiver: Receiver<Events>,

    modifier_state: u8,

    keys_held: Vec<u8>,

    pub converter: Converter,

    pub window: Window,
}

impl IOhelper
{
    /// Create a new helper from stuff
    pub(crate) fn new(gadget_path: &str, converter: Converter, window: Window) -> std::io::Result<IOhelper>
    {
        let receiver = IOhelper::init_hidraw_readers()?;
        let output_file = OpenOptions::new().write(true).open(gadget_path)?;
        let modifier_state = 0u8;
        let keys_held = Vec::new();
        Ok(IOhelper { output_file, receiver, modifier_state, keys_held, converter, window})
    }

    pub fn wait_for(&mut self, key: Key) -> Result<(), DataHandleError>
    {
        loop 
        { 
            if let Events::KeyDown(k) = self.process_input()?
            {
                if key == self.converter.get_key(&(Modifier::from(self.modifier_state), k))
                {
                    break;
                }
            }
            self.write_keys_to_output()?;
        }
        Ok(())
    }

    fn process_input(&mut self) -> Result<Events, std::sync::mpsc::RecvError> 
    {
        let event = self.receiver.recv()?;
        match event
        {
            Events::ModifierDown(pressed_modifier) => self.modifier_state |= pressed_modifier,
            Events::ModifierUp(released_modifier) => self.modifier_state ^= released_modifier,
            Events::KeyDown(pressed_key) => self.keys_held.push(pressed_key),
            Events::KeyUp(released_key) => self.keys_held.retain(|x| x != &released_key),
        };
        Ok(event)
    }

    pub fn next_key(&mut self) -> Result<Key, DataHandleError>
    {
        loop 
        {
            if let Events::KeyDown(k) = self.process_input()?
            {
                return Ok(self.converter.get_key(&(Modifier::from(self.modifier_state), k)))
            }
        }
    }

    pub fn screen_on(&mut self) -> std::io::Result<usize>
    {
        self.window.awaken()
    }

    pub fn screen_off(&mut self) -> std::io::Result<usize>
    {
        self.window.sleep()
    }

    fn init_hidraw_readers() -> std::io::Result<Receiver<Events>>
    {
        let (sender, receiver) = mpsc::channel();
        let files = IOhelper::viable_files()?;

        for mut file in files
        {
            let thread_sender = sender.clone();
            let mut buffer = vec![0u8;8];
            let mut old_buffer = vec![0u8;8];
            spawn(move || 
            {
                loop
                {
                    // Fix this garbage maybe?
                    if let Ok(_) = file.read(&mut buffer)
                    {
                        IOhelper::generate_events(&buffer, &old_buffer, &thread_sender);
                        std::mem::swap(&mut buffer, &mut old_buffer);
                    }
                }
            });
        }
        Ok(receiver)
    }

    fn generate_events(report: &[u8], old_report: &[u8], sender: &Sender<Events>) -> Result<(), SendError<Events>>
    {
        IOhelper::modifier_events(report[0], old_report[0], &sender)?;
        IOhelper::key_events(report, old_report, &sender)
    }

    fn modifier_events(new: u8, old: u8, sender: &Sender<Events>) -> Result<(), SendError<Events>>
    {
        use std::cmp::Ordering::{Equal, Less, Greater};
        match new.cmp(&old)
        {
            Equal => Ok(()),
            Less => sender.send(Events::ModifierUp(old - new)),
            Greater => sender.send(Events::ModifierDown(new - old)),
        }
    }

    fn key_events(report: &[u8], old_report: &[u8], sender: &Sender<Events>) -> Result<(), SendError<Events>>
    {
        for i in report[2..].iter().filter(|x| !old_report.contains(x))
        {
            sender.send(Events::KeyDown(*i))?;
        }
        for i in old_report[2..].iter().filter(|x| !report.contains(x))
        {
            sender.send(Events::KeyUp(*i))?;
        }
        Ok(())
    }


    fn viable_files()-> std::io::Result<Vec<File>>
    {
        let mut viable_files = Vec::new();
        let files = std::fs::read_dir("/dev/")?;
        for f in files
        {
            let f = f?.path();
            if f.to_str().expect("what").contains("hidraw")
            {
                let file = OpenOptions::new()
                .read(true)
                .write(true)
                .custom_flags(libc::O_NONBLOCK)
                .open(f)?;
                let fd = file.as_raw_fd();
        
                let mut size = 0;
                unsafe { hid_read_sz(fd, &mut size)?; }
                
                let mut desc_raw = hidraw_report_descriptor { size: size as u32, value: [0u8; HID_MAX_DESCRIPTOR_SIZE] };
                unsafe { hid_read_descr(fd, &mut desc_raw)?; }
                let data = &desc_raw.value[..desc_raw.size as usize];

                // Is keyboard?
                if data[3] == 6
                {
                    viable_files.push(file);
                }
                
            }
        }

        Ok(viable_files)
    }

    /// Write multiple buffers to the file, flushing between every buffer, as to simulate a single HID report
    pub fn write_buffers_to_file(&mut self, buffers: Vec<Vec<u8>>) -> std::io::Result<()>
    {
        for i in buffers
        {
            self.output_file.write(&i)?;
        }
        Ok(())
    }

    pub fn write_key(&mut self, key: &Key) -> std::io::Result<usize>
    {
        let (modi, key_code) = self.converter.get_raw(key);
        let output = vec![modi as u8, 0, key_code, 0, 0 ,0 ,0 ,0];
        self.output_file.write(&output)
    }

    pub fn write_keys_to_output(&mut self) -> std::io::Result<usize>
    {
        let mut output = vec![self.modifier_state, 0, 0, 0, 0, 0, 0, 0];
        for (index, key_code) in IOhelper::last_x_or_all(&self.keys_held, 6).enumerate()
        {
            let key = self.converter.get_key(&(Modifier::from(self.modifier_state), *key_code));
            let (modifier, key) = self.converter.get_raw(&key);
            output[0]  = modifier as u8;
            output[2 + index] = key
            
        }
        self.output_file.write(&output)
    }
    
    fn last_x_or_all<T>(collection: &[T], size: usize) -> std::slice::Iter<T>
    {
        if collection.len() > size
        {
            return collection[collection.len() - size..].iter()
        }
        collection.iter()
    }

    pub fn read_line(&mut self) -> Result<String, DataHandleError>
    {
        let mut result = String::new();
        loop 
        {
            match self.next_key()?
            {
                Key::Enter | Key::Macro => { self.window.print_write_buffer()?; return Ok(result) },
                Key::Char(c) => { self.window.add_char(c)?; result.push(c) },
                Key::Backspace => { self.window.remove_char(); result.pop(); },
                _ => continue,
            }
        }
    }
}