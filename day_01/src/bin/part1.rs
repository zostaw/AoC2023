#![feature(ascii_char)]
use std::fs;

fn main() {
    let file_path = "input";
    let file_path_example = "input.example";
    let file_content =
        fs::read_to_string(file_path)
            .or(fs::read_to_string(file_path_example))
            .unwrap_or_else(
                |err| panic!("{err}: Expected file '{file_path}' or '{file_path_example}'")
            );

    let final_val = file_content.lines().rfold(0, |acc, line| {
        let mut nums_only = line.matches(char::is_numeric);
        let first = nums_only.next().unwrap_or_else(|| "0");
        let second = match nums_only.next_back() {
            Some(val) => val,
            _ => first,
        };
        let val = vec![first, second]
            .concat()
            .parse::<i32>()
            .expect("No number found in string.");
        acc + val
    });

    println!("Sum: {}", final_val)
}
