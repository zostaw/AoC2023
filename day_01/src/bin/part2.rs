use std::fs;

fn find_digit(line: &str, reverse: bool) -> Option<char> {
    fn find(string_to_parse: &str, reversed_digits: bool) -> Option<char> {
        let named_digits: [&str; 10];

        if reversed_digits {
            named_digits = [
                "orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
            ];
        } else {
            named_digits = [
                "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];
        }

        for (key, named_digit) in named_digits.iter().enumerate() {
            if string_to_parse.starts_with(named_digit) {
                return Some(char::from_digit(key as u32, 10).unwrap());
            }
        }
        return None;
    }

    let mut char_id = 0;
    let line_parse: String;
    if reverse == true {
        line_parse = line.chars().rev().collect::<String>();
    } else {
        line_parse = line.to_owned();
    }
    while char_id <= line_parse.len() {
        let string_to_parse = &line_parse[char_id..];
        let first_char = string_to_parse.chars().next().unwrap();
        if first_char.is_digit(10) {
            return Some(first_char);
        } else {
            match find(&string_to_parse, reverse) {
                Some(digit) => return Some(digit),
                None => char_id += 1,
            };
        }
    }
    None
}

fn parse_file(file_str: &str) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    for line in file_str.lines().into_iter() {
        let first = find_digit(&line, false).unwrap();
        let last = find_digit(&line, true).unwrap();
        let mut val = String::from(first);
        val.push(last);
        output.push(val.parse::<i32>().expect("No number found in line."));
    }
    output
}

fn main() {
    let file_path = "input";
    let file_path_example = "input.example2";
    let file_content =
        fs::read_to_string(file_path)
            .or(fs::read_to_string(file_path_example))
            .unwrap_or_else(
                |err| panic!("{err}: Expected file '{file_path}' or '{file_path_example}'")
            );
    let final_val = parse_file(&file_content)
        .into_iter()
        .rfold(0, |acc, val| acc + val);

    println!("Sum: {}", final_val)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        assert_eq!(parse_file("one\n"), vec!(11));
    }

    #[test]
    fn last() {
        assert_eq!(parse_file("lastone\n"), vec!(11));
    }

    #[test]
    fn two_last() {
        assert_eq!(parse_file("lasttwone\n"), vec!(21));
    }

    #[test]
    fn one_and_digit() {
        assert_eq!(parse_file("one2\n"), vec!(12));
        assert_eq!(parse_file("one2thee\n"), vec!(12));
    }

    #[test]
    fn overlapping_named_numbers() {
        assert_eq!(parse_file("twone2thee\n"), vec!(22));
        assert_eq!(parse_file("threeight\n"), vec!(38));
    }

    #[test]
    fn multiline() {
        assert_eq!(parse_file("onne2thee\n"), vec!(22));
    }

    #[test]
    #[should_panic]
    fn no_digits() {
        println!("{:?}", parse_file("tw\none2thee\n"));
    }

    #[test]
    #[should_panic]
    fn no_digits2() {
        println!("{:?}", parse_file("two\noethee\n12\n"));
    }
}
