#![allow(dead_code)]

use std::{collections::{HashMap, VecDeque}};


/// The converter struct manages the conversion between a report code and the corrosponding ascii character or control key.
#[derive(Debug)]
pub struct Converter
{
    map: HashMap<Keypress, (Modifier, u8)>
}

impl Converter
{
    /// Create a new converter with populated look-up table, to translate between characters and
    /// HID reports.
    pub fn new() -> Converter
    {
        // save it as u16, with modifier upper byte keycode lowerbyte

        // If not in hashmap its not a character, Option<char> or enum type -char, -keycode, none?

        // hashmap to static array?

        //Hashmap keeps it on the heap(?) Since its static should probably be on the stack

        // Change the look up table fill, to a function, add qwerty, d
        let mut map = std::collections::HashMap::new();
        map.insert(Keypress::Character('a'), (Modifier::None, 0x04u8));
        map.insert(Keypress::Character('b'), (Modifier::None, 0x05u8));
        map.insert(Keypress::Character('c'), (Modifier::None, 0x06u8));
        map.insert(Keypress::Character('d'), (Modifier::None, 0x07u8));
        map.insert(Keypress::Character('e'), (Modifier::None, 0x08u8));
        map.insert(Keypress::Character('f'), (Modifier::None, 0x09u8));
        map.insert(Keypress::Character('g'), (Modifier::None, 0x0au8));
        map.insert(Keypress::Character('h'), (Modifier::None, 0x0bu8));
        map.insert(Keypress::Character('i'), (Modifier::None, 0x0cu8));
        map.insert(Keypress::Character('j'), (Modifier::None, 0x0du8));
        map.insert(Keypress::Character('k'), (Modifier::None, 0x0eu8));
        map.insert(Keypress::Character('l'), (Modifier::None, 0x0fu8));
        map.insert(Keypress::Character('m'), (Modifier::None, 0x10u8));
        map.insert(Keypress::Character('n'), (Modifier::None, 0x11u8));
        map.insert(Keypress::Character('o'), (Modifier::None, 0x12u8));
        map.insert(Keypress::Character('p'), (Modifier::None, 0x13u8));
        map.insert(Keypress::Character('q'), (Modifier::None, 0x14u8));
        map.insert(Keypress::Character('r'), (Modifier::None, 0x15u8));
        map.insert(Keypress::Character('s'), (Modifier::None, 0x16u8));
        map.insert(Keypress::Character('t'), (Modifier::None, 0x17u8));
        map.insert(Keypress::Character('u'), (Modifier::None, 0x18u8));
        map.insert(Keypress::Character('v'), (Modifier::None, 0x19u8));
        map.insert(Keypress::Character('w'), (Modifier::None, 0x1au8));
        map.insert(Keypress::Character('x'), (Modifier::None, 0x1bu8));
        map.insert(Keypress::Character('y'), (Modifier::None, 0x1cu8));
        map.insert(Keypress::Character('z'), (Modifier::None, 0x1du8));

        map.insert(Keypress::Character('A'), (Modifier::Shift, 0x04u8));
        map.insert(Keypress::Character('B'), (Modifier::Shift, 0x05u8));
        map.insert(Keypress::Character('C'), (Modifier::Shift, 0x06u8));
        map.insert(Keypress::Character('D'), (Modifier::Shift, 0x07u8));
        map.insert(Keypress::Character('E'), (Modifier::Shift, 0x08u8));
        map.insert(Keypress::Character('F'), (Modifier::Shift, 0x09u8));
        map.insert(Keypress::Character('G'), (Modifier::Shift, 0x0au8));
        map.insert(Keypress::Character('H'), (Modifier::Shift, 0x0bu8));
        map.insert(Keypress::Character('I'), (Modifier::Shift, 0x0cu8));
        map.insert(Keypress::Character('J'), (Modifier::Shift, 0x0du8));
        map.insert(Keypress::Character('K'), (Modifier::Shift, 0x0eu8));
        map.insert(Keypress::Character('L'), (Modifier::Shift, 0x0fu8));
        map.insert(Keypress::Character('M'), (Modifier::Shift, 0x10u8));
        map.insert(Keypress::Character('N'), (Modifier::Shift, 0x11u8));
        map.insert(Keypress::Character('O'), (Modifier::Shift, 0x12u8));
        map.insert(Keypress::Character('P'), (Modifier::Shift, 0x13u8));
        map.insert(Keypress::Character('Q'), (Modifier::Shift, 0x14u8));
        map.insert(Keypress::Character('R'), (Modifier::Shift, 0x15u8));
        map.insert(Keypress::Character('S'), (Modifier::Shift, 0x16u8));
        map.insert(Keypress::Character('T'), (Modifier::Shift, 0x17u8));
        map.insert(Keypress::Character('U'), (Modifier::Shift, 0x18u8));
        map.insert(Keypress::Character('V'), (Modifier::Shift, 0x19u8));
        map.insert(Keypress::Character('W'), (Modifier::Shift, 0x1au8));
        map.insert(Keypress::Character('X'), (Modifier::Shift, 0x1bu8));
        map.insert(Keypress::Character('Y'), (Modifier::Shift, 0x1cu8));
        map.insert(Keypress::Character('Z'), (Modifier::Shift, 0x1du8));

        map.insert(Keypress::Character('1'), (Modifier::None, 0x1eu8));
        map.insert(Keypress::Character('2'), (Modifier::None, 0x1fu8));
        map.insert(Keypress::Character('3'), (Modifier::None, 0x20u8));
        map.insert(Keypress::Character('4'), (Modifier::None, 0x21u8));
        map.insert(Keypress::Character('5'), (Modifier::None, 0x22u8));
        map.insert(Keypress::Character('6'), (Modifier::None, 0x23u8));
        map.insert(Keypress::Character('7'), (Modifier::None, 0x24u8));
        map.insert(Keypress::Character('8'), (Modifier::None, 0x25u8));
        map.insert(Keypress::Character('9'), (Modifier::None, 0x26u8));
        map.insert(Keypress::Character('0'), (Modifier::None, 0x27u8));

        map.insert(Keypress::Character('!'), (Modifier::Shift, 0x1eu8));
        map.insert(Keypress::Character('@'), (Modifier::Shift, 0x1fu8));
        map.insert(Keypress::Character('#'), (Modifier::Shift, 0x20u8));
        map.insert(Keypress::Character('$'), (Modifier::Shift, 0x21u8));
        map.insert(Keypress::Character('%'), (Modifier::Shift, 0x22u8));
        map.insert(Keypress::Character('^'), (Modifier::Shift, 0x23u8));
        map.insert(Keypress::Character('&'), (Modifier::Shift, 0x24u8));
        map.insert(Keypress::Character('*'), (Modifier::Shift, 0x25u8));
        map.insert(Keypress::Character('('), (Modifier::Shift, 0x26u8));
        map.insert(Keypress::Character(')'), (Modifier::Shift, 0x27u8));

        map.insert(Keypress::Character(' '), (Modifier::None, 0x2bu8));
        map.insert(Keypress::Character('\t'), (Modifier::None, 0x2cu8));
        map.insert(Keypress::Character('-'), (Modifier::None, 0x2du8));
        map.insert(Keypress::Character('_'), (Modifier::Shift, 0x2du8));
        map.insert(Keypress::Character('='), (Modifier::None, 0x2eu8));
        map.insert(Keypress::Character('+'), (Modifier::Shift, 0x2eu8));
        map.insert(Keypress::Character('['), (Modifier::None, 0x2fu8));
        map.insert(Keypress::Character('{'), (Modifier::Shift, 0x2fu8));
        map.insert(Keypress::Character(']'), (Modifier::None, 0x30u8));
        map.insert(Keypress::Character('}'), (Modifier::Shift, 0x30u8));
        map.insert(Keypress::Character('\\'), (Modifier::None, 0x31u8));
        map.insert(Keypress::Character('|'), (Modifier::Shift, 0x31u8));
        map.insert(Keypress::Character(';'), (Modifier::None, 0x33u8));
        map.insert(Keypress::Character(':'), (Modifier::Shift, 0x33u8));
        map.insert(Keypress::Character('\''), (Modifier::None, 0x34u8));
        map.insert(Keypress::Character('\"'), (Modifier::Shift, 0x34u8));
        map.insert(Keypress::Character('`'), (Modifier::None, 0x35u8));
        map.insert(Keypress::Character('~'), (Modifier::Shift, 0x35u8));
        map.insert(Keypress::Character(','), (Modifier::None, 0x36u8));
        map.insert(Keypress::Character('<'), (Modifier::Shift, 0x36u8));
        map.insert(Keypress::Character('.'), (Modifier::None, 0x37u8));
        map.insert(Keypress::Character('>'), (Modifier::Shift, 0x37u8));
        map.insert(Keypress::Character('/'), (Modifier::None, 0x38u8));
        map.insert(Keypress::Character('?'), (Modifier::Shift, 0x38u8));

        map.insert(Keypress::Macro, (Modifier::Ctrl, 0x38u8));
        map.insert(Keypress::Enter, (Modifier::None, 0x28u8));
        Converter { map }
    }

    /// This function will convert a given `&str` to the least amount report buffers required.
    /// 
    /// # Examples
    /// ```
    /// use fobword_core::converter::{Converter, Modifier};
    ///
    /// let converter = Converter::new();
    /// let result = converter.string_to_report_buffers("abcdef");
    /// let expected = vec![
    ///     vec![0, 0, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09], 
    ///     vec![0, 0, 0, 0, 0, 0, 0, 0,]];
    /// assert_eq!(result, Some(expected));
    /// 
    /// let result = converter.string_to_report_buffers("aA");
    /// let expected = vec![
    ///     vec![0, 0, 0x04, 0, 0, 0, 0, 0], 
    ///     vec![0, 0, 0, 0, 0, 0, 0, 0,],
    ///     vec![0x02, 0, 0x04, 0, 0, 0, 0, 0,],
    ///     vec![0, 0, 0, 0, 0, 0, 0, 0,]];
    /// assert_eq!(result, Some(expected));
    /// ```
    pub fn string_to_report_buffers(&self, word: &str) -> Option<Vec<Vec<u8>>>
    {
        // vector of buffers that are to be send
        let mut completed_report_buffers = Vec::new();
        // current buffer we are writing to
        let mut in_process_buffer = vec![0u8;8];
        // index of the current buffer we are writing too
        let mut index = 0;

        let mut chars = word.chars();
        if let Some(c) = chars.next()
        {   
            if let Some((char_shift_code, char_code)) = self.keypress_to_report_code(Keypress::Character(c))
            {
                self.write_first_char_to_buffer(&mut index, &mut in_process_buffer, char_code, char_shift_code);
            }
        }
        else
        {
            return None
        }        
        
        for i in chars
        {
            if let Some((char_shift_code, char_code)) = self.keypress_to_report_code(Keypress::Character(i))
            {
                if in_process_buffer.contains(&char_code)
                {
                    completed_report_buffers.push(in_process_buffer);
                    completed_report_buffers.push(vec![0u8;8]);
                    in_process_buffer = vec![0u8;8];
    
                    // The first char decides the "Shift" marker.
                    self.write_first_char_to_buffer(&mut index, &mut in_process_buffer, char_code, char_shift_code);
                    continue;
                }
                // This checks if the old buffer contains a character, if it does it needs to have a report in between where the key is not pressed
                // If the index is the same as or higher than 6, we need to send the buffer since there can be only 6 character at a time
                // And the last check is to see if the Shift marker is the same, else we need to send it and make a new buffer
                else if completed_report_buffers.last().unwrap_or(&vec![0u8;8]).contains(&char_code) || 
                        index >= 6 || 
                        &in_process_buffer[0] != &(char_shift_code as u8)
                {
                    completed_report_buffers.push(in_process_buffer);
                    in_process_buffer = vec![0u8;8];
                    self.write_first_char_to_buffer(&mut index, &mut in_process_buffer, char_code, char_shift_code);
                    continue;
                };
                self.write_to_buffer(&mut index, &mut in_process_buffer, char_code);
            }
        }
        // Send the last buffer and an empty one to indicate all keys are released
        completed_report_buffers.push(in_process_buffer);
        completed_report_buffers.push(vec![0u8;8]);
        Some(completed_report_buffers)
    }

    /// Attempts to retrieve `the HID report equivalent u8 and modifier key` from a lookup table with `character` as key.
    /// 
    /// If found returns an owned tuple of type `Some(<Modifier, u8>)`.
    /// 
    /// Will return `None`, if it cannot be found in the lookup table.
    /// 
    /// This will only work with ASCII characters.
    /// 
    /// # Examples
    /// ```
    /// use fobword_core::converter::{Converter, Modifier};
    ///
    /// let converter = Converter::new();
    /// let result = converter.character_to_report_code(&'a');
    /// assert_eq!(result, Some((Modifier::None, 0x04)));
    /// 
    /// let result = converter.character_to_report_code(&'æ');
    /// assert_eq!(result, None);
    /// ```
    pub fn keypress_to_report_code(&self, keypress: Keypress) -> Option<(Modifier, u8)>
    {
        if let Keypress::Control(val) = keypress
        {
            return Some(val)
        }
        match self.map.get(&keypress)
        {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }


    /// Attempts to retrieve `character` from a lookup table with `the HID report equivalent u8 and modifier key` as key.
    /// 
    /// If found returns an owned type `Some(char)`.
    /// 
    /// Will return `None`, if it cannot be found in the lookup table.
    /// 
    /// This will only work with ASCII characters.
    /// 
    /// # Examples
    /// ```
    /// use fobword_core::converter::{Converter, Modifier};
    ///
    /// let converter = Converter::new();
    /// let result = converter.report_code_to_character((Modifier::Shift, 0x04));
    /// assert_eq!(result, Some('A'));
    /// 
    /// let result = converter.report_code_to_character((Modifier::None, 0xFF));
    /// assert_eq!(result, None);
    /// ```
    pub fn report_code_to_keypress(&self, report_code: (Modifier, u8)) -> Keypress
    {
        match self.map.iter().find_map(|(key, val)| 
            if val.0 == report_code.0 && val.1 == report_code.1
            { 
                Some(key) 
            }
            else
            {
                None
            })
        {
            Some(value) => value.clone(),
            None => Keypress::Control(report_code),
        }
    }

    /// Since there can be up to 6 characters in a single HID report, this will look at each spot and attempt to retrieve a character 
    /// and combine them into Some(String).
    /// 
    /// # Example
    /// ```
    /// use fobword_core::converter::{Converter, Modifier};
    ///
    /// let converter = Converter::new();
    /// let report_buffer = vec![0, 0, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
    /// let result = converter.report_to_string(&report_buffer);
    /// assert_eq!(result, Some(String::from("abcdef")));
    /// ```
    pub fn report_to_keypress(&self, queue: &mut VecDeque<Keypress>, report: &[u8], old_report: &[u8])
    {
        let control_code = Modifier::from(report[0]);
        let s = &old_report[2..];
        for i in report.iter().skip(2).filter(|x| x != &&0 && !s.contains(x))
        {
            queue.push_back(self.report_code_to_keypress((control_code, *i)))
        }
    }
    
    /// Resets the index, and set the Modifier bit
    fn write_first_char_to_buffer<'a>(&self, index: &mut usize, buffer: &mut Vec<u8>, char_code: u8, modifier: Modifier)
    {
        *index = 0;
        buffer[0] = modifier as u8;
        self.write_to_buffer(index, buffer, char_code);
    }

    /// Write character to buffer and increment index
    fn write_to_buffer<'a>(&self, index: &mut usize, buffer: &mut Vec<u8>, char_code: u8)
    {
        buffer[*index + 2] = char_code;
        *index += 1;
    }

    fn add_type(&mut self, keypress: Keypress, tuple: (Modifier, u8))
    {
        self.map.insert(keypress, tuple);
    }

    pub fn add_macro(&mut self, report: &[u8])
    {
        let control_code = Modifier::from(report[0]);
        let key = report[1];
        self.add_type(Keypress::Macro, (control_code, key));
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
#[non_exhaustive]
/// Enum to indicate what key has been pressed
pub enum Keypress
{
    /// The key can be translated to an ascii character
    Character(char),
    //
    Control((Modifier, u8)),
    /// The enter key
    Enter,
    /// The macro combination (ctrl, shift, P)
    Macro,
    /// Not a usefull keypress (home key)
    None,
}

/// Enum to hold all combinations of modifier keys that can be used in this crate
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Modifier
{
    /// No modifier key is pressed
    None = 0,
    /// Control modifier key is pressed
    Ctrl = 1,
    /// Shift modifier key is pressed
    Shift = 2,
    /// Control and shift modifier keys are pressed
    CtrlShift = 3,
    /// Alt modifier key is pressed
    Alt = 4,
    /// Control and alt modifier keys are pressed
    CtrlAlt = 5,
    /// Shift and alt modifier keys are pressed
    ShiftAlt = 6,
    /// Control, shift and alt modifier keys are pressed
    CtrlShiftAlt = 7,
    /// GUI modifier key is pressed
    Gui = 8,
    /// Control and Gui modifier keys are pressed
    CtrlGui = 9,
    /// Shift and Gui modifier keys are pressed
    ShiftGui = 10,
    /// Control, Shift and Gui modifier keys are pressed
    CtrlShiftGui = 11,
    /// Alt and Gui modifier keys are pressed
    AltGui = 12,
    /// Control, alt and gui modifier keys are pressed
    CtrlAltGui = 13,
    /// Shift, alt and gui modifier keys are pressed
    ShiftAltGui = 14,
    /// Control, shift, alt and gui modifier keys are pressed
    CtrlShiftAltGui = 15,
}

impl From<u8> for Modifier
{
    /// Will convert a u8 to a Modifier using byte operation to check which bits are set
    fn from(num: u8) -> Modifier
    {
        match (num >> 4) | (num & 15)
        {
            0 => Modifier::None,
            1 => Modifier::Ctrl,
            2 => Modifier::Shift,
            3 => Modifier::CtrlShift,
            4 => Modifier::Alt,
            5 => Modifier::CtrlAlt,
            6 => Modifier::ShiftAlt,
            7 => Modifier::CtrlShiftAlt,
            8 => Modifier::Gui,
            9 => Modifier::CtrlGui,
            10 => Modifier::ShiftGui,
            11 => Modifier::CtrlShiftGui,
            12 => Modifier::AltGui,
            13 => Modifier::CtrlAltGui,
            14 => Modifier::ShiftAltGui,
            15 => Modifier::CtrlShiftAltGui,
            _ => Modifier::None,
        }
    }
}