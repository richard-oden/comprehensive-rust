#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let chars: Vec<char> = cc_number.chars().filter(|c| !c.is_whitespace()).collect();

    if chars.len() < 2 || chars.iter().any(|c| !c.is_numeric()) {
        return false;
    }

    let mut digits: Vec<u32> = chars.iter().map(|c| c.to_digit(10).unwrap()).collect();
    let max_index = digits.len() - 1;

    for i in (0..max_index).rev().step_by(2) {
        let new_value = (digits[i] * 2).to_string().chars().map(|c| c.to_digit(10).unwrap()).sum();
        digits[i] = new_value;
    }

    let sum = digits.iter().sum::<u32>();

    sum.to_string().ends_with('0')
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}