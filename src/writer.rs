use std::fs::File;
use std::io::Write;

use crate::error::FobError;

/// Converts every character in password to the corrosponding keyboard report value and writes them to the file using buffers.
/// 
/// Characters should be `ascii` not including the `space` character for this is normally not allowed in passwords.
/// # Examples
/// 
/// ```
/// let mut file = std::fs::File::create(r"/dev/hidg0").unwrap();
/// writer::write_password("this_is_a_password_string", &mut file);
/// ```
pub fn write_password(password: &str, file: &mut File) -> Result<(), FobError>
{
    let buffers = convert_password_to_buffers(password)?;
    write_buffers_to_file(buffers, file)?;
    Ok(())
}

fn write_buffers_to_file(buffers: Vec<Vec<u8>>, file: &mut File) -> Result<(), FobError>
{
    for i in buffers
    {
        file.write(&i)?;
    }
    Ok(())
}

fn convert_password_to_buffers(password: &str) -> Result<Vec<Vec<u8>>, FobError>
{
    let mut result = Vec::new();
    let mut buffer = vec![0u8;8];

    let mut chars = password.chars();
    let (char_shift_code, char_code) = convert_char_to_report_code(chars.next().unwrap())?;        
    let mut index = 0;

    write_first_char_to_buffer(&mut index, &mut buffer, char_code, char_shift_code);
    
    for i in chars
    {
        let (char_shift_code, char_code) = convert_char_to_report_code(i)?;
        if buffer.contains(&char_code)
        {
            result.push(buffer);
            result.push(vec![0u8;8]);
            buffer = vec![0u8;8];

            // The first char decides the "shift" marker.
            write_first_char_to_buffer(&mut index, &mut buffer, char_code, char_shift_code);
            continue;
        }
        // This checks if the old buffer contains a character, if it does it needs to have a report in between where the key is not pressed
        // If the index is the same as or higher than 6, we need to send the buffer since there can be only 6 character at a time
        // And the last check is to see if the shift marker is the same, else we need to send it and make a new buffer
        else if result.last().unwrap_or(&vec![0u8;8]).contains(&char_code) || index >= 6 || buffer[0] != char_shift_code
        {
            result.push(buffer);
            buffer = vec![0u8;8];
            write_first_char_to_buffer(&mut index, &mut buffer, char_code, char_shift_code);
            continue;
        };
        write_to_buffer(&mut index, &mut buffer, char_code);
    }
    // Send the last buffer and an empty one to indicate all keys are released
    result.push(buffer);
    result.push(vec![0u8;8]);
    Ok(result)
}


/// Takes a char and returns a tuple containing shift report code and char report code as per HID usages and descriptions page
/// https://usb.org/sites/default/files/hut1_21_0.pdf#page=83&zoom=100,57,57
fn convert_char_to_report_code(c: char) -> Result<(u8, u8), FobError>
{
    match c
    {
        'a' | 'A' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x04)),
        'b' | 'B' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x05)),
        'c' | 'C' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x06)),
        'd' | 'D' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x07)),
        'e' | 'E' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x08)),
        'f' | 'F' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x09)),
        'g' | 'G' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x0a)),
        'h' | 'H' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x0b)),
        'i' | 'I' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x0c)),
        'j' | 'J' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x0d)),
        'k' | 'K' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x0e)),
        'l' | 'L' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x0f)),
        'm' | 'M' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x10)),
        'n' | 'N' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x11)),
        'o' | 'O' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x12)),
        'p' | 'P' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x13)),
        'q' | 'Q' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x14)),
        'r' | 'R' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x15)),
        's' | 'S' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x16)),
        't' | 'T' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x17)),
        'u' | 'U' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x18)),
        'v' | 'V' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x19)),
        'w' | 'W' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x1a)),
        'x' | 'X' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x1b)),
        'y' | 'Y' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x1c)),
        'z' | 'Z' => Ok(({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x1d)),
        '1' => Ok((0x00, 0x1e)),
        '2' => Ok((0x00, 0x1f)),
        '3' => Ok((0x00, 0x20)),
        '4' => Ok((0x00, 0x21)),
        '5' => Ok((0x00, 0x22)),
        '6' => Ok((0x00, 0x23)),
        '7' => Ok((0x00, 0x24)),
        '8' => Ok((0x00, 0x25)),
        '9' => Ok((0x00, 0x26)),
        '0' => Ok((0x00, 0x27)),
        '!' => Ok((0x02, 0x1e)),
        '@' => Ok((0x02, 0x1f)),
        '#' => Ok((0x02, 0x20)),
        '$' => Ok((0x02, 0x21)),
        '%' => Ok((0x02, 0x22)),
        '^' => Ok((0x02, 0x23)),
        '&' => Ok((0x02, 0x24)),
        '*' => Ok((0x02, 0x25)),
        '(' => Ok((0x02, 0x26)),
        ')' => Ok((0x02, 0x27)),
        '-' => Ok((0x00, 0x2d)),
        '_' => Ok((0x02, 0x2d)),
        '=' => Ok((0x00, 0x2e)),
        '+' => Ok((0x02, 0x2e)),
        '[' => Ok((0x00, 0x2f)),
        '{' => Ok((0x02, 0x2f)),
        ']' => Ok((0x00, 0x30)),
        '}' => Ok((0x02, 0x30)),
        '\\' => Ok((0x00, 0x31)),
        '|' => Ok((0x02, 0x31)),
        ';' => Ok((0x00, 0x33)),
        ':' => Ok((0x02, 0x33)),
        '\'' => Ok((0x00, 0x34)),
        '\"' => Ok((0x02, 0x34)),
        '`' => Ok((0x00, 0x35)),
        '~' => Ok((0x02, 0x35)),
        ',' => Ok((0x00, 0x36)),
        '<' => Ok((0x02, 0x36)),
        '.' => Ok((0x00, 0x37)),
        '>' => Ok((0x02, 0x37)),
        '/' => Ok((0x00, 0x38)),
        '?' => Ok((0x02, 0x38)),
        _ => Err(FobError::ConvertError(c.to_string()))
    }
}

/// Resets the index, and set the shift byte
fn write_first_char_to_buffer(index: &mut usize, buffer: &mut Vec<u8>, char_code: u8, char_shift_code: u8)
{
    *index = 0;
    buffer[0] = char_shift_code;
    write_to_buffer(index, buffer, char_code);
}

/// Write character to buffer and increment index
fn write_to_buffer(index: &mut usize, buffer: &mut Vec<u8>, char_code: u8)
{
    buffer[*index + 2] = char_code;
    *index += 1;
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_single_letter_password()
    {
        let password = "a";
        let expected: Vec<Vec<u8>> = vec![
            vec![0x0,0x0,0x04,0x0,0x0,0x0,0x0,0x0], 
            vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]];
        assert_eq!(expected, convert_password_to_buffers(password).expect("test_single_letter"));
    }

    #[test]
    fn test_single_buffer_password()
    {
        let password = "abcdef";
        let expected = vec![
            vec![0x0,0x0,0x04,0x05,0x06,0x07,0x08,0x09], 
            vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]];
        assert_eq!(expected, convert_password_to_buffers(password).expect("test_single_buffer"));
    }

    #[test]
    fn test_two_buffer_length_password()
    {
        let password = "abcdefg";
        let expected: Vec<Vec<u8>> = vec![
            vec![0x0,0x0,0x04,0x05,0x06,0x07,0x08,0x09],
            vec![0x0,0x0,0x0a,0x0,0x0,0x0,0x0,0x0], 
            vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]];
        assert_eq!(expected, convert_password_to_buffers(password).expect("test_two_buffer_length"));
    }

    #[test]
    fn test_repeat_letter_password()
    {
        let password = "aa";
        let expected: Vec<Vec<u8>> = vec![
            vec![0x0,0x0,0x04,0x0,0x0,0x0,0x0,0x0],
            vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],
            vec![0x0,0x0,0x04,0x0,0x0,0x0,0x0,0x0],
            vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]];
        assert_eq!(expected, convert_password_to_buffers(password).expect("test_repeat_letter"));
    }

    #[test]
    fn test_shift_marker_password()
    {
        let password = "A";
        let expected: Vec<Vec<u8>> = vec![
            vec![0x02,0x0,0x04,0x0,0x0,0x0,0x0,0x0], 
            vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]];
        assert_eq!(expected, convert_password_to_buffers(password).expect("test_shift_marker"));
    }

    #[test]
    fn test_shift_differences_password()
    {
        let password = "AbcDE";
        let expected: Vec<Vec<u8>> = vec![
            vec![0x02,0x0,0x04,0x0,0x0,0x0,0x0,0x0], 
            vec![0x0,0x0,0x05,0x06,0x0,0x0,0x0,0x0],
            vec![0x02,0x0,0x07,0x08,0x0,0x0,0x0,0x0],
            vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]];
        assert_eq!(expected, convert_password_to_buffers(password).expect("test_shift_differences"));
    }

    #[test]
    fn test_normal_password()
    {
        let password = "adfa~1j;paASDA23;[];.,";
        let expected: Vec<Vec<u8>> = vec![
            vec![0x0,0x0,0x04,0x07,0x09,0x0,0x0,0x0], 
            vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],
            vec![0x0,0x0,0x04,0x0,0x0,0x0,0x0,0x0],
            vec![0x02,0x0,0x35,0x0,0x0,0x0,0x0,0x0],
            vec![0x0,0x0,0x1e,0x0d,0x33,0x13,0x04,0x0],
            vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],
            vec![0x02,0x0,0x04,0x16,0x07,0x0,0x0,0x0],
            vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],
            vec![0x2,0x0,0x04,0x0,0x0,0x0,0x0,0x0],
            vec![0x0,0x0,0x1f,0x20,0x33,0x2f,0x30,0x0],
            vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],
            vec![0x0,0x0,0x33,0x37,0x36,0x0,0x0,0x0],
            vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]];
        assert_eq!(expected, convert_password_to_buffers(password).expect("test_normal_password"));
    }

    #[test]
    #[should_panic]
    fn test_convert_error()
    {
        let password = " ";
        convert_password_to_buffers(password).unwrap();
    }
}