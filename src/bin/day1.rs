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

fn process_line(input: &str) -> u32 {
    let digits: Vec<char> = input.chars().filter(|c| c.is_numeric()).collect();
    let number_str = format!("{}{}", digits[0], digits[digits.len() - 1]);
    u64::from_str(&number_str).expect("should be valid integer")
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
    fn test_example() {
        const EXAMPLE_INPUT: &str = r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"#;

        let output = process(EXAMPLE_INPUT.lines().collect());

        assert_eq!(142, output);
    }

    #[test_case(12, "1abc2")]
    #[test_case(38, "pqr3stu8vwx")]
    #[test_case(15, "a1b2c3d4e5f")]
    #[test_case(77, "treb7uchet"; "same digit twice is OK")]
    fn test_process_line(expected: u32, line: &str) {
        assert_eq!(expected, process_line(line));
    }
}
