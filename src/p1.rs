use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "./input/p1";

pub fn p1() {
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
            panic!("Failed to find calibration value(s) in input")
        }
        let calibration_value = calibration_value.unwrap();
        calibration_sum += calibration_value;
        text.clear();
    }
    println!("Day 1 Part 1 answer: {calibration_sum}");
}

fn get_calibration_value(text: &str) -> Option<u32> {
    let mut left_cal_digit: Option<u32> = None;
    for ch in text.chars() {
        if let Some(digit) = ch.to_digit(10) {
            left_cal_digit = Some(digit);
            break;
        }
    }

    if left_cal_digit == None {
        return None;
    }
    let left_cal_digit = left_cal_digit.unwrap();

    let mut right_cal_digit = 10;
    for ch in text.chars().rev() {
        if let Some(digit) = ch.to_digit(10) {
            right_cal_digit = digit;
            break;
        }
    }
    let right_cal_digit = right_cal_digit;

    Some(left_cal_digit * 10 + right_cal_digit)
}
