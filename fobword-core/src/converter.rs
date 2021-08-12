use std::collections::HashMap;
/// A struct that holds a map which can be used to convert between raw keyboard codes and Keypress enum variants.
///
/// # Example
/// ```
/// let converter = Converter::default();
///
/// let modifier_key = Modifier::None;
/// let raw_key_code_a = 0x04u8;
/// assert_eq!(Keypress::Character('a'), converter.convert_rawinput(&modifier_key, &raw_key_code_a));
///
/// let keypress = Keypress::Character('Z');
/// assert_eq!((Modifier::Shift, 0x1du8), converter.convert_keypress(&keypress));
/// ```
#[derive(Debug)]
pub struct Converter
{
    map: HashMap<Keypress, (Modifier, u8)>
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
        let map = HashMap::new();

        Converter { map }
    }


    /// Constructs a Converter with  
    /// default mapping for keyboard codes as per USB HID Usages and Descriptions document: https://usb.org/sites/default/files/hut1_22.pdf
    ///
    /// # Examples
    /// ```
    /// let mut converter = Converter::default();
    /// ```
    pub fn default() -> Converter
    {
        let mut map = HashMap::new();

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

        map.insert(Keypress::Character('\t'), (Modifier::None, 0x2bu8));
        map.insert(Keypress::Character(' '), (Modifier::None, 0x2cu8));
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

        map.insert(Keypress::F(1), (Modifier::None, 0x3au8));
        map.insert(Keypress::F(2), (Modifier::None, 0x3bu8));
        map.insert(Keypress::F(3), (Modifier::None, 0x3cu8));
        map.insert(Keypress::F(4), (Modifier::None, 0x3du8));
        map.insert(Keypress::F(5), (Modifier::None, 0x3eu8));
        map.insert(Keypress::F(6), (Modifier::None, 0x3fu8));
        map.insert(Keypress::F(7), (Modifier::None, 0x40u8));
        map.insert(Keypress::F(8), (Modifier::None, 0x41u8));
        map.insert(Keypress::F(9), (Modifier::None, 0x42u8));
        map.insert(Keypress::F(10), (Modifier::None, 0x43u8));
        map.insert(Keypress::F(11), (Modifier::None, 0x44u8));
        map.insert(Keypress::F(12), (Modifier::None, 0x45u8));

        map.insert(Keypress::Enter, (Modifier::None, 0x28u8));

        Converter { map }
    }

    /// Add a new Macro keypress to the converter with the given raw inputs.
    pub fn add_macro(&mut self, modifier: Modifier, raw_key: u8)
    {
        self.add_type(Keypress::Macro, modifier, raw_key);
    }

    /// Add a new Character keypress to the converter with the given raw inputs.
    pub fn add_character(&mut self, ch: char, modifier: Modifier, raw_key: u8)
    {
        self.add_type(Keypress::Character(ch), modifier, raw_key);
    }

    /// Add a new Function keypress to the converter with the given raw inputs.
    pub fn add_f_key(&mut self, f_number: u8, modifier: Modifier, raw_key: u8)
    {
        self.add_type(Keypress::F(f_number), modifier, raw_key);
    }


    /// Add a keypress-raw input (Modifier key, u8 keycode) pair into the Converter 
    fn add_type(&mut self, keypress: Keypress, modifier: Modifier, raw_key: u8)
    {
        self.map.insert(keypress, (modifier, raw_key));
    }

    /// Remove a value from the Converter, after which the raw inputs will return default value.
    pub fn remove(&mut self, keypress: &Keypress)
    {
        self.map.remove(keypress);
    }


    /// Remove a value from the Converter by using the value of the key-value pair.
    pub fn remove_by_value(&mut self, modifier: &Modifier, raw_key: &u8)
    {   
        if let Some(pair) = self.map.iter().find(|(_key, raw)| raw.0 == *modifier && raw.1 == *raw_key)
        {
            let keypress = pair.0.clone();
            self.remove(&keypress)
        }
    }

    /// Convert a keypress into the corrosponding modifier key, u8 key code combination.
    ///
    /// # Examples
    /// ```
    /// let converter = Converter::Default(); 
    ///
    /// let keypress = Keypress::Character('Z');
    /// assert_eq!((Modifier::Shift, 0x1du8), converter.convert_keypress(&keypress));
    ///    
    /// let keypress = Keypress::Character('💖');
    /// assert_eq!((Modifier::None, 0x0u8), converter.convert_keypress(&keypress));
    /// ```
    pub fn convert_keypress(&self, keypress: &Keypress) -> (Modifier, u8)
    {
        if let Keypress::Undefined(modifier, code) = keypress
        {
            return (*modifier, *code)
        }
        match self.map.get(keypress)
        {
            Some(v) => v.clone(),
            None => (Modifier::None, 0),
        }
    }


    /// Convert the raw input into their corresponding keypress.
    ///
    /// A raw input will comprise a pressed modifier key and a u8 key code of the keys pressed on the keyboard.
    /// The modifier key is necessary because the keycode 0x04 maps to both 'a' and 'A' depending if the shift (modifier key) is pressed.
    ///
    /// On an unknown raw input, this function will return a Keypress::Undefined(Modifier_key, Keycode) that holds the given inputs.
    ///
    /// # Example
    /// ```
    /// let converter = Converter::default();
    /// let modifier_key = Modifier::None;
    /// let raw_key_code_a = 0x04u8;
    /// assert_eq!(Keypress::Character('a'), converter.convert_rawinput(&modifier_key, &raw_key_code_a));
    ///
    /// let modifier_key = Modifier::Shift;
    /// assert_eq!(Keypress::Character('A'), converter.convert_rawinput(&modifier_key, &raw_key_code_a));
    ///
    /// let modifier_key = Modifier::None;
    /// let raw_key_code_a = 0x01u8;
    /// assert_eq!(Keypress::Undefined(Modifier::None, 1), converter.convert_rawinput(&modifier_key, &raw_key_code_a));
    /// ```
    pub fn convert_rawinput(&self, modifier: &Modifier, raw_key: &u8) -> Keypress
    {
        if let Some(element) = self.map.iter().find(|(_key, raw)| raw.0 == *modifier && raw.1 == *raw_key)
        {
            return *element.0 // 0 is the Keypress part of the element key-value pair
        }
        Keypress::Undefined(*modifier, *raw_key)
    }
}

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
    /// Will convert a u8 to a Modifier using a byte operation to check which bits are set
    fn from(num: u8) -> Modifier
    {
        // (num >> 4) bitshift the u8 to the right by 4 bits making it a u4
        // (num & 15) will return the first 4 bits that are set by num
        // the | will return all of the bits set by (num >> 4) and (num & 15)
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


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Keypress
{
    Character(char),
    Undefined(Modifier, u8),
    Enter,
    Macro,
    F(u8),
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_new_converter()
    {
        let converter = Converter::new();

        assert!(converter.map.is_empty());

        let keypress = Keypress::Character('a');

        assert_eq!((Modifier::None, 0x0u8), converter.convert_keypress(&keypress));

        let modifier = Modifier::Ctrl;

        let raw_key_code = 0x04u8; // The a key as defined by the HID USB usages and descriptions

        assert_eq!(Keypress::Undefined(Modifier::Ctrl, 0x04u8), converter.convert_rawinput(&modifier, &raw_key_code));
    }
    
    #[test]
    fn test_converter_add_macro()
    {
        let mut converter = Converter::new();

        assert!(!converter.map.contains_key(&Keypress::Macro));

        let modifier = Modifier::Ctrl;

        let raw_key_code = 0x04u8;

        converter.add_macro(modifier, raw_key_code);

        assert!(converter.map.contains_key(&Keypress::Macro));
    }

    #[test]
    fn test_converter_add_character()
    {
        let mut converter = Converter::new();

        assert!(!converter.map.contains_key(&Keypress::Character('a')));

        let modifier = Modifier::None;

        let raw_key_code = 0x04u8;

        converter.add_character('a', modifier, raw_key_code);

        assert!(converter.map.contains_key(&Keypress::Character('a')));
    }

    #[test]
    fn test_convert_keypress_lowercase_character()
    {
        let converter = Converter::default();

        let keypress = Keypress::Character('a');

        assert_eq!((Modifier::None, 0x04u8), converter.convert_keypress(&keypress));

        // unknown characters should return 0
        let keypress = Keypress::Character('b');

        assert_eq!((Modifier::None, 0x05u8), converter.convert_keypress(&keypress));
    }

    #[test]
    fn test_convert_keypress_uppercase_character()
    {
        let converter = Converter::default();

        let keypress = Keypress::Character('A');

        assert_eq!((Modifier::Shift, 0x04u8), converter.convert_keypress(&keypress));

        // unknown characters should return 0
        let keypress = Keypress::Character('B');

        assert_eq!((Modifier::Shift, 0x05u8), converter.convert_keypress(&keypress));
    }

    #[test]
    fn test_convert_keypress_unknown_character()
    {
        let converter = Converter::default();

        // unknown characters should return 0
        let keypress = Keypress::Character('💖');

        assert_eq!((Modifier::None, 0u8), converter.convert_keypress(&keypress));
    }

    #[test]
    fn test_convert_rawinput_lowercase_character()
    {
        let converter = Converter::default();
        let modifier_key = Modifier::None;
        let raw_key_code_a = 0x04u8; // The a key on a qwerty keyboard

        assert_eq!(Keypress::Character('a'), converter.convert_rawinput(&modifier_key, &raw_key_code_a));

        let raw_key_code_b = 0x05u8;
        
        assert_eq!(Keypress::Character('b'), converter.convert_rawinput(&modifier_key, &raw_key_code_b));
    }

    #[test]
    fn test_convert_rawinput_uppercase_character()
    {
        let converter = Converter::default();
        let modifier_key = Modifier::Shift;
        let raw_key_code_a = 0x04u8; // The a key on a qwerty keyboard

        assert_eq!(Keypress::Character('A'), converter.convert_rawinput(&modifier_key, &raw_key_code_a));

        let raw_key_code_b = 0x05u8;
        
        assert_eq!(Keypress::Character('B'), converter.convert_rawinput(&modifier_key, &raw_key_code_b));
    }

    #[test]
    fn test_convert_rawinput_unknown_character()
    {
        let converter = Converter::default();
        let modifier_key = Modifier::None;
        let raw_key_code = 0x01u8;

        assert_eq!(Keypress::Undefined(Modifier::None, 0x01u8), converter.convert_rawinput(&modifier_key, &raw_key_code));

        let modifier_key = Modifier::Ctrl;
        let raw_key_code = 0;

        assert_eq!(Keypress::Undefined(Modifier::Ctrl, 0), converter.convert_rawinput(&modifier_key, &raw_key_code));
    }
}
