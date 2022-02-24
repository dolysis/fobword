use std::collections::HashMap;
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


    /// Constructs a Converter with  
    /// default querty mapping for keyboard codes as per USB HID Usages and Descriptions document: https://usb.org/sites/default/files/hut1_22.pdf
    ///
    /// # Examples
    /// ```
    /// let mut converter = Converter::default();
    /// ```
    pub fn default() -> Converter
    {
        use Key::{ Character, Enter, F, Backspace };
        use Modifier::{ NoModifier, Shift };
        let pairs: Vec<(Key, (Modifier, u8))> = 
        [
            // Lowercase characters
            (Character('a'), (NoModifier, 0x04u8)),
            (Character('b'), (NoModifier, 0x05u8)),
            (Character('c'), (NoModifier, 0x06u8)),
            (Character('d'), (NoModifier, 0x07u8)),
            (Character('e'), (NoModifier, 0x08u8)),
            (Character('f'), (NoModifier, 0x09u8)),
            (Character('g'), (NoModifier, 0x0au8)),
            (Character('h'), (NoModifier, 0x0bu8)),
            (Character('i'), (NoModifier, 0x0cu8)),
            (Character('j'), (NoModifier, 0x0du8)),
            (Character('k'), (NoModifier, 0x0eu8)),
            (Character('l'), (NoModifier, 0x0fu8)),
            (Character('m'), (NoModifier, 0x10u8)),
            (Character('n'), (NoModifier, 0x11u8)),
            (Character('o'), (NoModifier, 0x12u8)),
            (Character('p'), (NoModifier, 0x13u8)),
            (Character('q'), (NoModifier, 0x14u8)),
            (Character('r'), (NoModifier, 0x15u8)),
            (Character('s'), (NoModifier, 0x16u8)),
            (Character('t'), (NoModifier, 0x17u8)),
            (Character('u'), (NoModifier, 0x18u8)),
            (Character('v'), (NoModifier, 0x19u8)),
            (Character('w'), (NoModifier, 0x1au8)),
            (Character('x'), (NoModifier, 0x1bu8)),
            (Character('y'), (NoModifier, 0x1cu8)),
            (Character('z'), (NoModifier, 0x1du8)),

            // Uppercase characters
            (Character('A'), (Shift, 0x04u8)),
            (Character('B'), (Shift, 0x05u8)),
            (Character('C'), (Shift, 0x06u8)),
            (Character('D'), (Shift, 0x07u8)),
            (Character('E'), (Shift, 0x08u8)),
            (Character('F'), (Shift, 0x09u8)),
            (Character('G'), (Shift, 0x0au8)),
            (Character('H'), (Shift, 0x0bu8)),
            (Character('I'), (Shift, 0x0cu8)),
            (Character('J'), (Shift, 0x0du8)),
            (Character('K'), (Shift, 0x0eu8)),
            (Character('L'), (Shift, 0x0fu8)),
            (Character('M'), (Shift, 0x10u8)),
            (Character('N'), (Shift, 0x11u8)),
            (Character('O'), (Shift, 0x12u8)),
            (Character('P'), (Shift, 0x13u8)),
            (Character('Q'), (Shift, 0x14u8)),
            (Character('R'), (Shift, 0x15u8)),
            (Character('S'), (Shift, 0x16u8)),
            (Character('T'), (Shift, 0x17u8)),
            (Character('U'), (Shift, 0x18u8)),
            (Character('V'), (Shift, 0x19u8)),
            (Character('W'), (Shift, 0x1au8)),
            (Character('X'), (Shift, 0x1bu8)),
            (Character('Y'), (Shift, 0x1cu8)),
            (Character('Z'), (Shift, 0x1du8)),

            // Numbers
            (Character('1'), (NoModifier, 0x1eu8)),
            (Character('2'), (NoModifier, 0x1fu8)),
            (Character('3'), (NoModifier, 0x20u8)),
            (Character('4'), (NoModifier, 0x21u8)),
            (Character('5'), (NoModifier, 0x22u8)),
            (Character('6'), (NoModifier, 0x23u8)),
            (Character('7'), (NoModifier, 0x24u8)),
            (Character('8'), (NoModifier, 0x25u8)),
            (Character('9'), (NoModifier, 0x26u8)),
            (Character('0'), (NoModifier, 0x27u8)),

            // Symbols
            (Character('!'), (Shift, 0x1eu8)),
            (Character('@'), (Shift, 0x1fu8)),
            (Character('#'), (Shift, 0x20u8)),
            (Character('$'), (Shift, 0x21u8)),
            (Character('%'), (Shift, 0x22u8)),
            (Character('^'), (Shift, 0x23u8)),
            (Character('&'), (Shift, 0x24u8)),
            (Character('*'), (Shift, 0x25u8)),
            (Character('('), (Shift, 0x26u8)),
            (Character(')'), (Shift, 0x27u8)),

            (Character('\t'), (NoModifier, 0x2bu8)),
            (Character(' '), (NoModifier, 0x2cu8)),
            (Character('-'), (NoModifier, 0x2du8)),
            (Character('_'), (Shift, 0x2du8)),
            (Character('='), (NoModifier, 0x2eu8)),
            (Character('+'), (Shift, 0x2eu8)),
            (Character('['), (NoModifier, 0x2fu8)),
            (Character('{'), (Shift, 0x2fu8)),
            (Character(']'), (NoModifier, 0x30u8)),
            (Character('}'), (Shift, 0x30u8)),
            (Character('\\'), (NoModifier, 0x31u8)),
            (Character('|'), (Shift, 0x31u8)),
            (Character(','), (NoModifier, 0x33u8)),
            (Character(':'), (Shift, 0x33u8)),
            (Character('\''), (NoModifier, 0x34u8)),
            (Character('\"'), (Shift, 0x34u8)),
            (Character('`'), (NoModifier, 0x35u8)),
            (Character('~'), (Shift, 0x35u8)),
            (Character(','), (NoModifier, 0x36u8)),
            (Character('<'), (Shift, 0x36u8)),
            (Character('.'), (NoModifier, 0x37u8)),
            (Character('>'), (Shift, 0x37u8)),
            (Character('/'), (NoModifier, 0x38u8)),
            (Character('?'), (Shift, 0x38u8)),

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
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
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
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Key
{
    /// Any of the number, symbols or alphabetical keys
    Character(char),
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

        let keypress = Key::Character('a');

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

        assert!(!converter.input_map.contains_key(&Key::Character('a')));

        let modifier = Modifier::NoModifier;

        let raw_key_code = 0x04u8;

        converter.add_character('a', modifier, raw_key_code);

        assert!(converter.input_map.contains_key(&Key::Character('a')));
    }

    #[test]
    fn test_convert_keypress_lowercase_character()
    {
        let converter = Converter::default();

        let keypress = Key::Character('a');

        assert_eq!((Modifier::NoModifier, 0x04u8), converter.convert_keypress(&keypress));

        // unknown characters should return 0
        let keypress = Key::Character('b');

        assert_eq!((Modifier::NoModifier, 0x05u8), converter.convert_keypress(&keypress));
    }

    #[test]
    fn test_convert_keypress_uppercase_character()
    {
        let converter = Converter::default();

        let keypress = Key::Character('A');

        assert_eq!((Modifier::Shift, 0x04u8), converter.convert_keypress(&keypress));

        // unknown characters should return 0
        let keypress = Key::Character('B');

        assert_eq!((Modifier::Shift, 0x05u8), converter.convert_keypress(&keypress));
    }

    #[test]
    fn test_convert_keypress_unknown_character()
    {
        let converter = Converter::default();

        // unknown characters should return 0
        let keypress = Key::Character('💖');

        assert_eq!((Modifier::NoModifier, 0u8), converter.convert_keypress(&keypress));
    }

    #[test]
    fn test_convert_rawinput_lowercase_character()
    {
        let converter = Converter::default();
        let modifier_key = Modifier::NoModifier;
        let raw_key_code_a = 0x04u8; // The a key on a qwerty keyboard

        assert_eq!(Key::Character('a'), converter.convert_rawinput(&modifier_key, &raw_key_code_a));

        let raw_key_code_b = 0x05u8;
        
        assert_eq!(Key::Character('b'), converter.convert_rawinput(&modifier_key, &raw_key_code_b));
    }

    #[test]
    fn test_convert_rawinput_uppercase_character()
    {
        let converter = Converter::default();
        let modifier_key = Modifier::Shift;
        let raw_key_code_a = 0x04u8; // The a key on a qwerty keyboard

        assert_eq!(Key::Character('A'), converter.convert_rawinput(&modifier_key, &raw_key_code_a));

        let raw_key_code_b = 0x05u8;
        
        assert_eq!(Key::Character('B'), converter.convert_rawinput(&modifier_key, &raw_key_code_b));
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
