use fobword_core::converter::{Converter, Modifier, Keypress};
use std::collections::VecDeque;

#[test]
fn number_to_flag()
{
    // First 4 bits or combination
    assert_eq!(Modifier::None, Modifier::from(0));
    assert_eq!(Modifier::Ctrl, Modifier::from(1));
    assert_eq!(Modifier::CtrlAltGui, Modifier::from(13));

    // Last 4 bits or combination
    // 16 = 0001_0000
    assert_eq!(Modifier::Ctrl, Modifier::from(16));
    // 34 = 0010_0010
    assert_eq!(Modifier::Shift, Modifier::from(34));
    // 240 = 1111_0000
    assert_eq!(Modifier::CtrlShiftAltGui, Modifier::from(240));
}   

/// Test to verify if the conversion from Modifier enum to u8 works as intended
#[test]
fn flag_to_number()
{
    assert_eq!(0, Modifier::None as u8);
    assert_eq!(1, Modifier::Ctrl as u8);
    assert_eq!(15, Modifier::CtrlShiftAltGui as u8);
}

/// Test to verify if the conversion from an lowercase 'character' to a 'report code' works as intended
#[test]
fn one_lowercase_character_to_report_code()
{
    let converter = Converter::new();
    assert_eq!(converter.character_to_report_code('a'), Some((Modifier::None, 0x04)));
}

/// Test to verify if the conversion from an uppercase 'character' to a 'report code' works as intended
#[test]
fn one_uppercase_character_to_report_code()
{
    let converter = Converter::new();
    assert_eq!(converter.character_to_report_code('A'), Some((Modifier::Shift, 0x04)));
}

/// Test to verify if the conversion from an uppercase 'character' to a 'report code' works as intended
#[test]
fn non_alphabet_character_to_report_code()
{
    let converter = Converter::new();
    assert_eq!(converter.character_to_report_code('!'), Some((Modifier::Shift, 0x1e)));
}

/// Test to verify if the report code with a single lowercase character gets converted to a ascii character
#[test]
fn report_code_to_character()
{
    let converter = Converter::new();
    assert_eq!(converter.report_code_to_keypress((Modifier::None, 0x04)), Keypress::Character('a'))
}

#[test]
fn full_report_buffer_to_string()
{
    let converter = Converter::new();
    let report_buffer = vec![0, 0, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
    let mut queue = VecDeque::new();
    converter.report_to_keypress(&mut queue, &report_buffer, &vec![0u8;8]);
    assert_eq!(queue.pop_front(), Some(Keypress::Character('a')));
    assert_eq!(queue.pop_front(), Some(Keypress::Character('b')));
    assert_eq!(queue.pop_front(), Some(Keypress::Character('c')));
    assert_eq!(queue.pop_front(), Some(Keypress::Character('d')));
    assert_eq!(queue.pop_front(), Some(Keypress::Character('e')));
    assert_eq!(queue.pop_front(), Some(Keypress::Character('f')));
    assert_eq!(queue.pop_front(), None);
}

#[test]
fn shift_report_buffer_to_keypresses()
{
    let converter = Converter::new();
    let report_buffer = vec![0x02, 0, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
    let mut queue = VecDeque::new();
    converter.report_to_keypress(&mut queue, &report_buffer, &vec![0u8;8]);
    assert_eq!(queue.pop_front(), Some(Keypress::Character('A')));
    assert_eq!(queue.pop_front(), Some(Keypress::Character('B')));
    assert_eq!(queue.pop_front(), Some(Keypress::Character('C')));
    assert_eq!(queue.pop_front(), Some(Keypress::Character('D')));
    assert_eq!(queue.pop_front(), Some(Keypress::Character('E')));
    assert_eq!(queue.pop_front(), Some(Keypress::Character('F')));
    assert_eq!(queue.pop_front(), None);
}

#[test]
fn string_to_report_buffers()
{
    let converter = Converter::new();
    let result = converter.string_to_report_buffers("abcdef");
    let expected = vec![
        vec![0, 0, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09], 
        vec![0, 0, 0, 0, 0, 0, 0, 0,]];
    assert_eq!(result, Some(expected));
}

#[test]
fn repeat_character_string_to_report_buffers()
{
    let converter = Converter::new();
    let result = converter.string_to_report_buffers("aA");
    let expected = vec![
        vec![0, 0, 0x04, 0, 0, 0, 0, 0], 
        vec![0, 0, 0, 0, 0, 0, 0, 0,],
        vec![0x02, 0, 0x04, 0, 0, 0, 0, 0,],
        vec![0, 0, 0, 0, 0, 0, 0, 0,]];
    assert_eq!(result, Some(expected));
}