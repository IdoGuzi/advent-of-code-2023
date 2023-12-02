use std::fs;
use std::error::Error;
use std::collections::HashMap;

static NUMBERS: &[&str] = &[
    "1", "one",
    "2", "two",
    "3", "three",
    "4", "four",
    "5", "five",
    "6", "six",
    "7", "seven",
    "8", "eight",
    "9", "nine"
];

fn find_digit(text: &str, reverse: bool) -> Result<u8, Box<dyn Error>> {
    let mut indices: HashMap<u8, u8> = HashMap::new();
    for (i,v) in NUMBERS.iter().enumerate() {
        let index_option; 
        if !reverse {index_option = text.find(v);}else{index_option = text.rfind(v);}
        match index_option {
            None => continue,
            Some(index) => {
                if !indices.contains_key(&(index as u8)) {
                    indices.insert(index as u8, ((i/2) + 1) as u8);
                }
            }
        };
    }
    if indices.is_empty() {
        return Err(["no digits were found in: ", text].join("").into())
    }
    if reverse {
        return Ok(*indices.get(indices.keys().max().unwrap()).unwrap());
    }
    return Ok(*indices.get(indices.keys().min().unwrap()).unwrap());
}

fn extract_calibration_value(text: &str) -> Result<u8, Box<dyn Error>> {
    let first = find_digit(text, false)?;
    let last = find_digit(text, true)?;
    let value: u8 = first * 10 + last;
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
