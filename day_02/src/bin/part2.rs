use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./input");
    let example_path = Path::new("./input.example");

    let input = fs::read_to_string(path)
        .or(fs::read_to_string(example_path))
        .expect("Expected 'input' or 'input.example' file.");

    let mut result = 0;
    for line in input.lines() {
        let line = line
            .strip_prefix("Game ")
            .expect("Line should start with Game ");

        let (_game_id, rest) = line.split_once(": ").expect("No Id");

        let reveals: Vec<&str> = rest.split("; ").collect();
        let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);
        for reveal in reveals {
            let colors: Vec<&str> = reveal.split(", ").collect();
            for color in colors {
                match color {
                    x if x.contains("red") => {
                        let val = x
                            .split_once(" ")
                            .expect("Wrong red format.")
                            .0
                            .parse::<i32>()
                            .expect("red should be in format '<i32> red'.");
                        if val > min_red {
                            min_red = val;
                        }
                    }
                    x if x.contains("green") => {
                        let val = x
                            .split_once(" ")
                            .expect("Wrong green format.")
                            .0
                            .parse::<i32>()
                            .expect("green should be in format '<i32> green'.");
                        if val > min_green {
                            min_green = val;
                        }
                    }
                    x if x.contains("blue") => {
                        let val = x
                            .split_once(" ")
                            .expect("Wrong blue format.")
                            .0
                            .parse::<i32>()
                            .expect("blue should be in format '<i32> blue'.");
                        if val > min_blue {
                            min_blue = val;
                        }
                    }
                    _ => println!("not matched"),
                }
            }
        }

        result += min_red * min_green * min_blue;
    }

    println!("result: {}", result);
}
