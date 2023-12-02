use std::fs::read_to_string;

// Day 1 calibration calculation

pub fn calculate(filename: &str) -> u32 {
    let mut total: u32 = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let row_val: u32 = get_row_value(line);
        println!("line '{:?}' value '{:?}'", line, row_val);
        total = total + row_val;
    }
    println!("Total {}", total);
    return total;
}

fn get_digit(line: &str, pos: usize) -> Option<u32> {
    let ch = line.chars().nth(pos).unwrap().to_digit(10);
    if ch.is_some() {
        return ch;
    }
    let remains = &line[pos..];
    return if remains.starts_with("zero") {
        Some(0)
    } else if remains.starts_with("one") {
        Some(1)
    } else if remains.starts_with("two") {
        Some(2)
    } else if remains.starts_with("three") {
        Some(3)
    } else if remains.starts_with("four") {
        Some(4)
    } else if remains.starts_with("five") {
        Some(5)
    } else if remains.starts_with("six") {
        Some(6)
    } else if remains.starts_with("seven") {
        Some(7)
    } else if remains.starts_with("eight") {
        Some(8)
    } else if remains.starts_with("nine") {
        Some(9)
    } else {
        None
    };
}

fn get_row_value(line: &str) -> u32 {
    let mut first = Option::None;
    let mut last = Option::None;

    for i in 0..line.len() {
        let digit = get_digit(line, i);
        if digit.is_some() {
            last = digit;
            if (first.is_none()) {
                first = last;
            }
        }
    }
    println!("first '{}', last '{}'", first.unwrap(), last.unwrap());
    return first.unwrap() * 10u32 + last.unwrap();
}
