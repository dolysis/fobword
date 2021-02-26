use std::fs::File;
use std::io::Write;

/// Converts every character in password to the corrosponding keyboard report value and writes them to the file using buffers.
/// 
/// Characters should be `ascii` not including the `space` character for this is normally not allowed in passwords.
/// # Examples
/// 
/// ```
/// let mut file = std::fs::File::create(r"/dev/hidg0").unwrap();
/// writer::write_password("this_is_a_password_string", &mut file);
/// ```
pub fn write_password(password: &str, file: &mut File)
{
    let buffers = convert_password_to_buffers(password);
    write_buffers_to_file(buffers, file)
}

fn write_buffers_to_file(buffers: Vec<Vec<u8>>, file: &mut File)
{
    for i in buffers
    {
        file.write(&i);
    }
}

fn convert_password_to_buffers(password: &str) -> Vec<Vec<u8>>
{
    let mut result = Vec::new();
    let mut buffer = vec![0u8;8];
    let mut old_buffer = &vec![0u8;8];

    let mut chars = password.chars();
    let (char_shift_code, char_code) = convert_char_to_report_code(chars.next().unwrap());        
    let mut index = 0;

    write_first_char_to_buffer(&mut index, &mut buffer, char_code, char_shift_code);
    
    for i in chars
    {
        let (char_shift_code, char_code) = convert_char_to_report_code(i);
        if buffer.contains(&char_code)
        {
            result.push(buffer);
            result.push(vec![0u8;8]);
            buffer = vec![0u8;8];
            old_buffer = result.last().expect("bla");

            // The first char decides the "shift" marker.
            write_first_char_to_buffer(&mut index, &mut buffer, char_code, char_shift_code);
            continue;
        }
        // This checks if the old buffer contains a character, if it does it needs to have a report in between where the key is not pressed
        // If the index is the same as or higher than 6, we need to send the buffer since there can be only 6 character at a time
        // And the last check is to see if the shift marker is the same, else we need to send it and make a new buffer
        else if old_buffer.contains(&char_code) || index >= 6 || buffer[0] != char_shift_code
        {
            result.push(buffer);
            old_buffer = result.last().expect("bla");
            buffer = vec![0u8;8];
            write_first_char_to_buffer(&mut index, &mut buffer, char_code, char_shift_code);
            continue;
        };
        write_to_buffer(&mut index, &mut buffer, char_code);
    }
    // Send the last buffer and an empty one to indicate all keys are released
    result.push(buffer);
    result.push(vec![0u8;8]);
    result
}


/// Takes a char and returns a tuple containing shift report code and char report code as per HID usages and descriptions page
/// https://usb.org/sites/default/files/hut1_21_0.pdf#page=83&zoom=100,57,57
fn convert_char_to_report_code(c: char) -> (u8, u8)
{
    match c
    {
        'a' | 'A' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x04),
        'b' | 'B' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x05),
        'c' | 'C' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x06),
        'd' | 'D' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x07),
        'e' | 'E' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x08),
        'f' | 'F' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x09),
        'g' | 'G' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x0a),
        'h' | 'H' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x0b),
        'i' | 'I' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x0c),
        'j' | 'J' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x0d),
        'k' | 'K' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x0e),
        'l' | 'L' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x0f),
        'm' | 'M' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x10),
        'n' | 'N' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x11),
        'o' | 'O' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x12),
        'p' | 'P' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x13),
        'q' | 'Q' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x14),
        'r' | 'R' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x15),
        's' | 'S' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x16),
        't' | 'T' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x17),
        'u' | 'U' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x18),
        'v' | 'V' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x19),
        'w' | 'W' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x1a),
        'x' | 'X' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x1b),
        'y' | 'Y' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x1c),
        'z' | 'Z' => ({ if c.is_lowercase() { 0x00} else { 0x02 } }, 0x1d),
        '1' => (0x00, 0x1e),
        '2' => (0x00, 0x1f),
        '3' => (0x00, 0x20),
        '4' => (0x00, 0x21),
        '5' => (0x00, 0x22),
        '6' => (0x00, 0x23),
        '7' => (0x00, 0x24),
        '8' => (0x00, 0x25),
        '9' => (0x00, 0x26),
        '0' => (0x00, 0x27),
        '!' => (0x02, 0x1e),
        '@' => (0x02, 0x1f),
        '#' => (0x02, 0x20),
        '$' => (0x02, 0x21),
        '%' => (0x02, 0x22),
        '^' => (0x02, 0x23),
        '&' => (0x02, 0x24),
        '*' => (0x02, 0x25),
        '(' => (0x02, 0x26),
        ')' => (0x02, 0x27),
        '-' => (0x00, 0x2d),
        '_' => (0x02, 0x2d),
        '=' => (0x00, 0x2e),
        '+' => (0x02, 0x2e),
        '[' => (0x00, 0x2f),
        '{' => (0x02, 0x2f),
        ']' => (0x00, 0x30),
        '}' => (0x02, 0x30),
        '\\' => (0x00, 0x31),
        '|' => (0x02, 0x31),
        ';' => (0x00, 0x33),
        ':' => (0x02, 0x33),
        '\'' => (0x00, 0x34),
        '\"' => (0x02, 0x34),
        '`' => (0x00, 0x35),
        '~' => (0x02, 0x35),
        ',' => (0x00, 0x36),
        '<' => (0x02, 0x36),
        '.' => (0x00, 0x37),
        '>' => (0x02, 0x37),
        '/' => (0x00, 0x38),
        '?' => (0x02, 0x38),
        
        

        _ => (0x00, 0x00) 
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
        assert_eq!(expected, convert_password_to_buffers(password));
    }

    #[test]
    fn test_single_buffer_password()
    {
        let password = "abcdef";
        let expected = vec![
            vec![0x0,0x0,0x04,0x05,0x06,0x07,0x08,0x09], 
            vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]];
        assert_eq!(expected, convert_password_to_buffers(password));
    }

    #[test]
    fn test_two_buffer_length_password()
    {
        let password = "abcdefg";
        let expected: Vec<Vec<u8>> = vec![
            vec![0x0,0x0,0x04,0x05,0x06,0x07,0x08,0x09],
            vec![0x0,0x0,0x0a,0x0,0x0,0x0,0x0,0x0], 
            vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]];
        assert_eq!(expected, convert_password_to_buffers(password));
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
        assert_eq!(expected, convert_password_to_buffers(password));
    }

    #[test]
    fn test_shift_marker_password()
    {
        let password = "A";
        let expected: Vec<Vec<u8>> = vec![
            vec![0x02,0x0,0x04,0x0,0x0,0x0,0x0,0x0], 
            vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]];
        assert_eq!(expected, convert_password_to_buffers(password));
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
        assert_eq!(expected, convert_password_to_buffers(password));
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
        assert_eq!(expected, convert_password_to_buffers(password));
    }
}