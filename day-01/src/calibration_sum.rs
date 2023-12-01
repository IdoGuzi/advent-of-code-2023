use std::fs;
use std::error::Error;

fn find_digit(text: &mut Vec<char>, reverse: bool) -> Result<usize, Box<dyn Error>> {
    if reverse {
        text.reverse();
    }
    for i in 0..text.len() {
        match text[i] {
            '0' | '1' |'2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                if !reverse {
                    return Ok(i);
                }
                return Ok(text.len()-1 - i);
            },
            _ => continue,
        };
    }
    let original_string: String = text.iter().collect();
    let err = ["digit not found in ", &original_string].join("");
    return Err(err.into());
}

fn extract_calibration_value(text: &str) -> Result<u8, Box<dyn Error>> {
    let vec: Vec<char> = text.chars().collect();
    let first = find_digit(&mut vec.clone(), false)?;
    let last = find_digit(&mut vec.clone(), true)?;
    let value: u8 = (vec[first] as u8 - 48) * 10 + (vec[last] as u8 - 48);
    return Ok(value);

}

pub fn read_file(inputs_path: String) -> Result<u64, Box<dyn Error>> {
    let inputs = fs::read_to_string(inputs_path)?;
    let lines = inputs.lines();
    let mut sum: u64 = 0;
    for line in lines {
        sum += extract_calibration_value(line)? as u64;
    }
    return Ok(sum);
}
