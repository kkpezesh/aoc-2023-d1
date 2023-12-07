use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "./input/p2";

const TEXT_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn text_digits_to_u32(text_digit: &str) -> Option<u32> {
    match text_digit {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

pub fn p2() {
    let input_file = File::open(INPUT_PATH);
    if input_file.is_err() {
        panic!("Failed to open file {INPUT_PATH}")
    }
    let input_file = input_file.unwrap();

    let mut reader = BufReader::new(input_file);
    let mut text = String::new();
    let mut calibration_sum = 0;
    while reader.read_line(&mut text).unwrap_or(0) > 0 {
        let calibration_value = get_calibration_value(&text);
        if calibration_value.is_none() {
            panic!("Failed to find calibration values in input")
        }
        let calibration_value = calibration_value.unwrap();
        calibration_sum += calibration_value;
        text.clear();
    }
    println!("Day 1 Part 2 answer: {calibration_sum}");
}

fn get_calibration_value(text: &str) -> Option<u32> {
    let mut left_cal_digit: Option<u32> = None;
    for (index, ch) in text.char_indices() {
        if let Some(digit) = ch.to_digit(10) {
            left_cal_digit = Some(digit);
            break;
        } else if let Some(digit) = text_to_digit(index, text) {
            left_cal_digit = Some(digit);
            break;
        }
    }

    if left_cal_digit == None {
        return None;
    }
    let left_cal_digit = left_cal_digit.unwrap();

    let mut right_cal_digit = 10;
    for (index, ch) in text.char_indices().rev() {
        if let Some(digit) = ch.to_digit(10) {
            right_cal_digit = digit;
            break;
        } else if let Some(digit) = text_to_digit(index, text) {
            right_cal_digit = digit;
            break;
        }
    }
    let right_cal_digit = right_cal_digit;

    Some(left_cal_digit * 10 + right_cal_digit)
}

fn text_to_digit(index: usize, text: &str) -> Option<u32> {
    for text_digit in TEXT_DIGITS {
        let end_index = text.len().min(index + text_digit.len());
        if &text[index..end_index] == text_digit {
            return text_digits_to_u32(text_digit);
        }
    }
    None
}
