use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Context;
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Path to input file
    #[arg()]
    input: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let input_file_content = fs::read_to_string(args.input).context("Input file did not exist")?;
    let input: Vec<&str> = input_file_content.lines().collect();
    let output = process(input);
    println!("{}", output);
    Ok(())
}

/// If word matches some digit, return the digit. Otherwise none.
///
/// ```
/// assert_eq!("two", Some(2))
/// assert_eq!("eighteen", None)
/// assert_eq!("onefourfive", None)
/// ```
fn digit_from_word(word: &str) -> Option<u32> {
    match word {
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

fn process_line(input: &str) -> u32 {
    fn process_substr(input: &str, character: char, char_idx: usize) -> Option<u32> {
        let digits_as_words = vec![
            "one",
            "two",
            "three",
            "four",
            "five",
            "six",
            "seven",
            "eight",
            "nine",
        ];
        if let Some(digit) = character.to_digit(10) {
            return Some(digit);
        }
        for word in &digits_as_words {
            if input[char_idx..].starts_with(word) {
                return Some(digit_from_word(word).unwrap());
            }
        }
        None
    }

    let mut first: Option<u32> = None;
    for (idx, c) in input.char_indices() {
        first = process_substr(input, c, idx);
        if first.is_some() {
            break;
        }
    }
    let first = first.expect("should have found number in input");

    let mut last = None;
    for (idx, c) in input.char_indices().rev() {
        last = process_substr(input, c, idx);
        if last.is_some() {
            break;
        }
    }
    let last = last.expect("should have found number in input");

    let number_str = format!("{}{}", first, last);
    u32::from_str(&number_str).expect("should be valid integer")
}

fn process(input_lines: Vec<&str>) -> u32 {
    input_lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| process_line(line))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test]
    fn test_example_p1() {
        const EXAMPLE_INPUT: &str = r#"1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet"#;

        let output = process(EXAMPLE_INPUT.lines().collect());

        assert_eq!(142, output);
    }

    #[test]
    fn test_example_p2() {
        const EXAMPLE_INPUT: &str = r#"two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen"#;

            let output = process(EXAMPLE_INPUT.lines().collect());

            assert_eq!(281, output);
    }


    #[test_case(12, "1abc2")]
    #[test_case(38, "pqr3stu8vwx")]
    #[test_case(15, "a1b2c3d4e5f")]
    #[test_case(77, "treb7uchet"; "same digit twice is OK")]
    #[test_case(29, "two1nine")]
    #[test_case(83, "eightwothree")]
    #[test_case(13, "abcone2threexyz")]
    #[test_case(24, "xtwone3four")]
    #[test_case(42, "4nineeightseven2")]
    #[test_case(14, "zoneight234")]
    #[test_case(76, "7pqrstsixteen"; "single digit numbers only")]
    fn test_process_line_p2(expected: u32, line: &str) {
        let actual = process_line(line);
        assert_eq!(expected, actual, "expected {expected} but got {actual} for input {line:?}");
    }
}
