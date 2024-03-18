use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub(crate) fn invoke() -> io::Result<()> {
    let file_path = "./src/one/input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut callibration_sum: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut first_digit: Option<u8> = None;
        let mut last_digit: Option<u8> = None;
        for char in line.chars() {
            if char.is_digit(10) {
                let digit = Some(char.to_digit(10).unwrap() as u8);
                if first_digit == None {
                    first_digit = digit;
                } else {
                    last_digit = digit;
                }
            }
        }

        if last_digit == None {
            last_digit = first_digit;
        }

        let first_digit = first_digit.unwrap();
        let last_digit = last_digit.unwrap();
        let callibration_value = format!("{first_digit}{last_digit}");
        let callibration_value = callibration_value.parse::<u32>().unwrap();
        callibration_sum += callibration_value;
        // println!("Line: {line} {callibration_value}");
    }

    println!("Sum: {callibration_sum}");
    Ok(())
}
