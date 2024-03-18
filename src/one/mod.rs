use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use regex::Regex;

fn read_file(path: &str) -> io::Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

pub(crate) fn invoke() -> io::Result<()> {
    let file_path = "./src/one/input.txt";
    let reader = read_file(file_path)?;

    let digits_map = HashMap::<&str, u8>::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut callibration_sum: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut first_digit: Option<u8> = None;
        let mut last_digit: Option<u8> = None;

        let mut first_digit_idx: Option<usize> = None;
        let mut last_digit_idx: Option<usize> = None;
        let digit_re = Regex::new(r"(\d)").unwrap();
        // println!("\nLine: {line}");
        for capture in digit_re.captures_iter(line.as_str()) {
            let idx = capture.get(0).unwrap().start();
            let (_, [value]) = capture.extract();
            let digit = Some(value.parse::<u8>().unwrap());
            if first_digit == None {
                first_digit = digit;
                first_digit_idx = Some(idx);

                if last_digit == None {
                    last_digit = digit;
                    last_digit_idx = Some(idx);
                }
            } else {
                last_digit = digit;
                last_digit_idx = Some(idx);
            }
        }
        // println!(
        //     "{:?} {:?} : {:?} {:?}",
        //     first_digit, first_digit_idx, last_digit, last_digit_idx
        // );

        for (digit_str, digit) in digits_map.iter() {
            let re = Regex::new(format!("({})", digit_str).as_str()).unwrap();
            for m in re.find_iter(line.as_str()) {
                let match_idx = m.start();
                let digit = Some(*digit);
                match first_digit_idx {
                    None => {
                        first_digit = digit;
                        first_digit_idx = Some(match_idx);
                    }
                    Some(idx) => {
                        if match_idx < idx {
                            first_digit = digit;
                            first_digit_idx = Some(match_idx);
                        }
                    }
                }
                match last_digit_idx {
                    None => {
                        last_digit = digit;
                        last_digit_idx = Some(match_idx);
                    }
                    Some(idx) => {
                        // println!("{digit_str}: {match_idx} {idx}");
                        if match_idx >= idx {
                            last_digit = digit;
                            last_digit_idx = Some(match_idx);
                        }
                    }
                }
            }
        }

        let digits = format!("{}{}", first_digit.unwrap(), last_digit.unwrap());
        let num = digits.parse::<u32>().unwrap();
        callibration_sum += num;
        // println!(
        //     "{:?} {:?} : {:?} {:?}",
        //     first_digit, first_digit_idx, last_digit, last_digit_idx
        // );
        // println!("Digits: {digits}");
    }

    println!("Sum: {callibration_sum}");
    Ok(())
}

