use fobword_core::converter::{Converter, Key, Modifier};


/// Convert a string to raw input buffers.
///
/// This is a convience function to convert a String into the least ammount of raw input reports.
///
/// # Example
/// ```
/// let conv = Converter::default();
/// let word = "test";
/// 
/// let buffers = converterutilities::string_to_report_buffers(&conv, &word).unwrap();
///
/// let expected = vec![
///     vec![0, 0, 0x17, 0x08, 0x16, 0, 0, 0], 
///     vec![0, 0, 0, 0, 0, 0, 0, 0,],
///     vec![0, 0, 0x17, 0, 0, 0, 0, 0],
///     vec![0, 0, 0, 0, 0, 0, 0, 0,]];
/// 
/// assert_eq!(expected, buffers);
/// ```
pub fn string_to_report_buffers(conv: &Converter, word: &str) -> Option<Vec<Vec<u8>>>
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
        let (char_shift_code, char_code) = conv.get_raw(&Key::Char(c));
        write_first_char_to_buffer(&mut index, &mut in_process_buffer, char_code, char_shift_code);
    }
    else
    {
        return None
    }        
    
    for c in chars
    {
        let (char_shift_code, char_code) = conv.get_raw(&Key::Char(c));
        if in_process_buffer.contains(&char_code)
        {
            completed_report_buffers.push(in_process_buffer);
            completed_report_buffers.push(vec![0u8;8]);
            in_process_buffer = vec![0u8;8];

            // The first char decides the "Shift" marker.
            write_first_char_to_buffer(&mut index, &mut in_process_buffer, char_code, char_shift_code);
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
            write_first_char_to_buffer(&mut index, &mut in_process_buffer, char_code, char_shift_code);
            continue;
        };
        write_to_buffer(&mut index, &mut in_process_buffer, char_code);
    }
    // Send the last buffer and an empty one to indicate all keys are released
    completed_report_buffers.push(in_process_buffer);
    completed_report_buffers.push(vec![0u8;8]);
    Some(completed_report_buffers)
}

/// Resets the index, and set the Modifier bit
fn write_first_char_to_buffer<'a>(index: &mut usize, buffer: &mut Vec<u8>, char_code: u8, modifier: Modifier)
{
    *index = 0;
    buffer[0] = modifier as u8;
    write_to_buffer(index, buffer, char_code);
}

/// Write character to buffer and increment index
fn write_to_buffer<'a>(index: &mut usize, buffer: &mut Vec<u8>, char_code: u8)
{
    buffer[*index + 2] = char_code;
    *index += 1;
}
