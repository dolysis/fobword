use fobword_core::converter::{Converter, Keypress, Modifier};
use std::collections::VecDeque;

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
        let (char_shift_code, char_code) = conv.convert_keypress(&Keypress::Character(c));
        write_first_char_to_buffer(&mut index, &mut in_process_buffer, char_code, char_shift_code);
    }
    else
    {
        return None
    }        
    
    for c in chars
    {
        let (char_shift_code, char_code) = conv.convert_keypress(&Keypress::Character(c));
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

pub fn report_to_keypress(conv: &Converter, queue: &mut VecDeque<Keypress>, report: &[u8], old_report: &[u8])
{
    let modifier = Modifier::from(report[0]);
    let s = &old_report[2..];
    // Filter the keys if they appear twice in a report in a row
    for raw_key in report.iter().skip(2).filter(|x| !s.contains(x))
    {
        queue.push_back(conv.convert_rawinput(&modifier, &raw_key));
    }
}