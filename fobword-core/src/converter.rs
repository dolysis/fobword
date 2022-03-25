use std::{collections::HashMap, fs::OpenOptions, io::Read};
use std::io::{Error, ErrorKind};
use serde::{ Serialize, Deserialize };
/// A struct that holds a map which can be used to convert between raw keyboard codes and Keypress enum variants.
///
/// # Example
/// ```
/// let converter = Converter::default();
///
/// let modifier_key = Modifier::NoModifier;
/// let raw_key_code_a = 0x04u8;
/// assert_eq!(Keypress::Character('a'), converter.convert_rawinput(&modifier_key, &raw_key_code_a));
///
/// let keypress = Keypress::Character('Z');
/// assert_eq!((Modifier::Shift, 0x1du8), converter.convert_keypress(&keypress));
/// ```
#[derive(Debug)]
pub struct Converter
{
    input_map: HashMap<(Modifier, u8), Key>,
    output_map: HashMap<Key, (Modifier, u8)>
}

impl Converter
{

    /// Constructs a new, empty Converter
    ///
    /// Using this without filling the map will return default values
    ///
    /// # Examples
    /// ```
    /// let mut converter = Converter::new();
    /// ```
    pub fn new() -> Converter
    {
        let input_map = HashMap::new();
        let output_map = HashMap::new();
        Converter { input_map, output_map }
    }

    pub fn from_paths(input: &str, output: &str) -> Result<Converter, Error>
    {
        let mut input_string = String::new();
        let mut input_file = OpenOptions::new().read(true).open(input)?;
        input_file.read_to_string(&mut input_string)?;
        let input_vector: Vec<(Key, (Modifier, u8))> = ron::from_str(&input_string).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        let input_map = input_vector.iter().cloned().map(|(k, c)| (c, k)).collect::<HashMap<(Modifier, u8), Key>>();

        let mut output_string = String::new();
        let mut output_file = OpenOptions::new().read(true).open(output)?;
        output_file.read_to_string(&mut output_string)?;
        let output_vector: Vec<(Key, (Modifier, u8))> = ron::from_str(&output_string).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        let output_map = output_vector.iter().cloned().collect::<HashMap<Key, (Modifier, u8)>>();
        Ok( Converter { input_map, output_map } )
    }

    /// Constructs a Converter with  
    /// default querty mapping for keyboard codes as per USB HID Usages and Descriptions document: https://usb.org/sites/default/files/hut1_22.pdf
    ///
    /// # Examples
    /// ```
    /// let mut converter = Converter::default();
    /// ```
    pub fn default() -> Converter
    {
        use Key::{ Char, Enter, F, Backspace };
        use Modifier::{ NoModifier, Shift };
        let pairs: Vec<(Key, (Modifier, u8))> = 
        [
            // Lowercase characters
            (Char('a'), (NoModifier, 0x04u8)),
            (Char('b'), (NoModifier, 0x05u8)),
            (Char('c'), (NoModifier, 0x06u8)),
            (Char('d'), (NoModifier, 0x07u8)),
            (Char('e'), (NoModifier, 0x08u8)),
            (Char('f'), (NoModifier, 0x09u8)),
            (Char('g'), (NoModifier, 0x0au8)),
            (Char('h'), (NoModifier, 0x0bu8)),
            (Char('i'), (NoModifier, 0x0cu8)),
            (Char('j'), (NoModifier, 0x0du8)),
            (Char('k'), (NoModifier, 0x0eu8)),
            (Char('l'), (NoModifier, 0x0fu8)),
            (Char('m'), (NoModifier, 0x10u8)),
            (Char('n'), (NoModifier, 0x11u8)),
            (Char('o'), (NoModifier, 0x12u8)),
            (Char('p'), (NoModifier, 0x13u8)),
            (Char('q'), (NoModifier, 0x14u8)),
            (Char('r'), (NoModifier, 0x15u8)),
            (Char('s'), (NoModifier, 0x16u8)),
            (Char('t'), (NoModifier, 0x17u8)),
            (Char('u'), (NoModifier, 0x18u8)),
            (Char('v'), (NoModifier, 0x19u8)),
            (Char('w'), (NoModifier, 0x1au8)),
            (Char('x'), (NoModifier, 0x1bu8)),
            (Char('y'), (NoModifier, 0x1cu8)),
            (Char('z'), (NoModifier, 0x1du8)),

            // Uppercase characters
            (Char('A'), (Shift, 0x04u8)),
            (Char('B'), (Shift, 0x05u8)),
            (Char('C'), (Shift, 0x06u8)),
            (Char('D'), (Shift, 0x07u8)),
            (Char('E'), (Shift, 0x08u8)),
            (Char('F'), (Shift, 0x09u8)),
            (Char('G'), (Shift, 0x0au8)),
            (Char('H'), (Shift, 0x0bu8)),
            (Char('I'), (Shift, 0x0cu8)),
            (Char('J'), (Shift, 0x0du8)),
            (Char('K'), (Shift, 0x0eu8)),
            (Char('L'), (Shift, 0x0fu8)),
            (Char('M'), (Shift, 0x10u8)),
            (Char('N'), (Shift, 0x11u8)),
            (Char('O'), (Shift, 0x12u8)),
            (Char('P'), (Shift, 0x13u8)),
            (Char('Q'), (Shift, 0x14u8)),
            (Char('R'), (Shift, 0x15u8)),
            (Char('S'), (Shift, 0x16u8)),
            (Char('T'), (Shift, 0x17u8)),
            (Char('U'), (Shift, 0x18u8)),
            (Char('V'), (Shift, 0x19u8)),
            (Char('W'), (Shift, 0x1au8)),
            (Char('X'), (Shift, 0x1bu8)),
            (Char('Y'), (Shift, 0x1cu8)),
            (Char('Z'), (Shift, 0x1du8)),

            // Numbers
            (Char('1'), (NoModifier, 0x1eu8)),
            (Char('2'), (NoModifier, 0x1fu8)),
            (Char('3'), (NoModifier, 0x20u8)),
            (Char('4'), (NoModifier, 0x21u8)),
            (Char('5'), (NoModifier, 0x22u8)),
            (Char('6'), (NoModifier, 0x23u8)),
            (Char('7'), (NoModifier, 0x24u8)),
            (Char('8'), (NoModifier, 0x25u8)),
            (Char('9'), (NoModifier, 0x26u8)),
            (Char('0'), (NoModifier, 0x27u8)),

            // Symbols
            (Char('!'), (Shift, 0x1eu8)),
            (Char('@'), (Shift, 0x1fu8)),
            (Char('#'), (Shift, 0x20u8)),
            (Char('$'), (Shift, 0x21u8)),
            (Char('%'), (Shift, 0x22u8)),
            (Char('^'), (Shift, 0x23u8)),
            (Char('&'), (Shift, 0x24u8)),
            (Char('*'), (Shift, 0x25u8)),
            (Char('('), (Shift, 0x26u8)),
            (Char(')'), (Shift, 0x27u8)),

            (Char('\t'), (NoModifier, 0x2bu8)),
            (Char(' '), (NoModifier, 0x2cu8)),
            (Char('-'), (NoModifier, 0x2du8)),
            (Char('_'), (Shift, 0x2du8)),
            (Char('='), (NoModifier, 0x2eu8)),
            (Char('+'), (Shift, 0x2eu8)),
            (Char('['), (NoModifier, 0x2fu8)),
            (Char('{'), (Shift, 0x2fu8)),
            (Char(']'), (NoModifier, 0x30u8)),
            (Char('}'), (Shift, 0x30u8)),
            (Char('\\'), (NoModifier, 0x31u8)),
            (Char('|'), (Shift, 0x31u8)),
            (Char(';'), (NoModifier, 0x33u8)),
            (Char(':'), (Shift, 0x33u8)),
            (Char('\''), (NoModifier, 0x34u8)),
            (Char('\"'), (Shift, 0x34u8)),
            (Char('`'), (NoModifier, 0x35u8)),
            (Char('~'), (Shift, 0x35u8)),
            (Char(','), (NoModifier, 0x36u8)),
            (Char('<'), (Shift, 0x36u8)),
            (Char('.'), (NoModifier, 0x37u8)),
            (Char('>'), (Shift, 0x37u8)),
            (Char('/'), (NoModifier, 0x38u8)),
            (Char('?'), (Shift, 0x38u8)),

            // The F keys
            (F(1), (NoModifier, 0x3au8)),
            (F(2), (NoModifier, 0x3bu8)),
            (F(3), (NoModifier, 0x3cu8)),
            (F(4), (NoModifier, 0x3du8)),
            (F(5), (NoModifier, 0x3eu8)),
            (F(6), (NoModifier, 0x3fu8)),
            (F(7), (NoModifier, 0x40u8)),
            (F(8), (NoModifier, 0x41u8)),
            (F(9), (NoModifier, 0x42u8)),
            (F(10), (NoModifier, 0x43u8)),
            (F(11), (NoModifier, 0x44u8)),
            (F(12), (NoModifier, 0x45u8)),

            (Enter, (NoModifier, 0x28u8)),
            (Backspace, (NoModifier, 0x2Au8))
        ].iter().cloned().collect();

        let input_map = pairs.iter().cloned().map(|(k, c)| (c, k)).collect();
        let output_map = pairs.iter().cloned().collect();
        Converter { input_map, output_map }
    }


    /// Add a new Macro keypress to the converter with the given raw inputs.
    pub fn add_macro(&mut self, modifier: Modifier, raw_key: u8)
    {
        self.add_type(Key::Macro, modifier, raw_key);
    }

    /// Add a keypress-raw input (Modifier key, u8 keycode) pair into the Converter 
    fn add_type(&mut self, keypress: Key, modifier: Modifier, raw_key: u8)
    {
        self.input_map.insert((modifier, raw_key), keypress);
        self.output_map.insert(keypress, (modifier, raw_key));
    }

    pub fn get_key(&self, k: &(Modifier, u8)) -> Key
    {
        // Needs to return an owned value for undefined keys
        self.input_map.get(k).map_or_else(|| Key::Undefined(k.0, k.1), |v| v.to_owned())
    }

    pub fn get_raw(&self, k: &Key) -> (Modifier, u8)
    {
        match k
        {
            // If they key didn't exist in the input map, check if it exist without a modifier attached
            &Key::Undefined(m, c) =>
            {
                // Try to find the key as if it had no modifier
                let no_mod_key = self.input_map.get(&(Modifier::NoModifier, c));
                match no_mod_key
                {
                    // If they key without modifier exists, get the converted code, and add the original modifier
                    Some(k) => self.output_map.get(k).map_or_else(|| (Modifier::NoModifier, 0), |v| (m, v.1).to_owned()),
                    // If it still can't be found return the raw codes and it will default to your computers mapping
                    None => (m, c),
                }
            },
            _ => self.output_map.get(k).map_or_else(|| (Modifier::NoModifier, 0), |v| v.to_owned())
        }
    }
}

/// Keyboard modifier input values
///
/// Type Modifier represents the pressed combinations of the modifier keys on the keyboard (ctrl, shift, alt, gui or none).
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, Deserialize, Serialize)]
pub enum Modifier
{
    /// No modifier key is pressed
    NoModifier = 0,
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
    /// Will convert a u8 to a Modifier using a byte operation to check which bits are set
    fn from(num: u8) -> Modifier
    {
        // Left modifier keys use the lower half of the u8
        // Right modifier keys use the upper half of the u8
        // Here we combine them into one Modifier.
        match (num >> 4) | (num & 15)
        {
            0 => Modifier::NoModifier,
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
            _ => Modifier::NoModifier,
        }
    }
}

/// Keyboard input values.
///
/// Type Keypress represents the different type of inputs from the keyboard
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Serialize)]
pub enum Key
{
    /// Any of the number, symbols or alphabetical keys
    Char(char),
    // The enter key
    Enter,
    // The F keys
    F(u8),       
    // A combination of a modifier key and regular input defined by user
    Macro, 
    // The backspace key
    Backspace,
    // A key or combination of keys that do not fall in the other Keypress values
    Undefined(Modifier, u8),
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_new_converter()
    {
        let converter = Converter::new();

        assert!(converter.input_map.is_empty());

        let keypress = Key::Char('a');

        assert_eq!((Modifier::NoModifier, 0x0u8), converter.convert_keypress(&keypress));

        let modifier = Modifier::Ctrl;

        let raw_key_code = 0x04u8; // The a key as defined by the HID USB usages and descriptions

        assert_eq!(Key::Undefined(Modifier::Ctrl, 0x04u8), converter.convert_rawinput(&modifier, &raw_key_code));
    }
    
    #[test]
    fn test_converter_add_macro()
    {
        let mut converter = Converter::new();

        assert!(!converter.input_map.contains_key(&Key::Macro));

        let modifier = Modifier::Ctrl;

        let raw_key_code = 0x04u8;

        converter.add_macro(modifier, raw_key_code);

        assert!(converter.input_map.contains_key(&Key::Macro));
    }

    #[test]
    fn test_converter_add_character()
    {
        let mut converter = Converter::new();

        assert!(!converter.input_map.contains_key(&Key::Char('a')));

        let modifier = Modifier::NoModifier;

        let raw_key_code = 0x04u8;

        converter.add_character('a', modifier, raw_key_code);

        assert!(converter.input_map.contains_key(&Key::Char('a')));
    }

    #[test]
    fn test_convert_keypress_lowercase_character()
    {
        let converter = Converter::default();

        let keypress = Key::Char('a');

        assert_eq!((Modifier::NoModifier, 0x04u8), converter.convert_keypress(&keypress));

        // unknown characters should return 0
        let keypress = Key::Char('b');

        assert_eq!((Modifier::NoModifier, 0x05u8), converter.convert_keypress(&keypress));
    }

    #[test]
    fn test_convert_keypress_uppercase_character()
    {
        let converter = Converter::default();

        let keypress = Key::Char('A');

        assert_eq!((Modifier::Shift, 0x04u8), converter.convert_keypress(&keypress));

        // unknown characters should return 0
        let keypress = Key::Char('B');

        assert_eq!((Modifier::Shift, 0x05u8), converter.convert_keypress(&keypress));
    }

    #[test]
    fn test_convert_keypress_unknown_character()
    {
        let converter = Converter::default();

        // unknown characters should return 0
        let keypress = Key::Char('💖');

        assert_eq!((Modifier::NoModifier, 0u8), converter.convert_keypress(&keypress));
    }

    #[test]
    fn test_convert_rawinput_lowercase_character()
    {
        let converter = Converter::default();
        let modifier_key = Modifier::NoModifier;
        let raw_key_code_a = 0x04u8; // The a key on a qwerty keyboard

        assert_eq!(Key::Char('a'), converter.convert_rawinput(&modifier_key, &raw_key_code_a));

        let raw_key_code_b = 0x05u8;
        
        assert_eq!(Key::Char('b'), converter.convert_rawinput(&modifier_key, &raw_key_code_b));
    }

    #[test]
    fn test_convert_rawinput_uppercase_character()
    {
        let converter = Converter::default();
        let modifier_key = Modifier::Shift;
        let raw_key_code_a = 0x04u8; // The a key on a qwerty keyboard

        assert_eq!(Key::Char('A'), converter.convert_rawinput(&modifier_key, &raw_key_code_a));

        let raw_key_code_b = 0x05u8;
        
        assert_eq!(Key::Char('B'), converter.convert_rawinput(&modifier_key, &raw_key_code_b));
    }

    #[test]
    fn test_convert_rawinput_unknown_character()
    {
        let converter = Converter::default();
        let modifier_key = Modifier::NoModifier;
        let raw_key_code = 0x01u8;

        assert_eq!(Key::Undefined(Modifier::NoModifier, 0x01u8), converter.convert_rawinput(&modifier_key, &raw_key_code));

        let modifier_key = Modifier::Ctrl;
        let raw_key_code = 0;

        assert_eq!(Key::Undefined(Modifier::Ctrl, 0), converter.convert_rawinput(&modifier_key, &raw_key_code));
    }
}
