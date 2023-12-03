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
        let mut possible = true;
        let line = line
            .strip_prefix("Game ")
            .expect("Line should start with Game ");

        let (game_id, rest) = line.split_once(": ").expect("No Id");

        // I decided to parse twice according to story, but really it could be split with both
        // comma and semicolon simultaniously
        // So *reveals* contains grouped colors and *colors* split to match one color at a time
        let reveals: Vec<&str> = rest.split("; ").collect();
        for reveal in reveals {
            let colors: Vec<&str> = reveal.split(", ").collect();
            for color in colors {
                match color {
                    x if x.contains("red") => {
                        if x.split_once(" ")
                            .expect("Wrong red format.")
                            .0
                            .parse::<i32>()
                            .expect("Red should be in format '<i32> red'.")
                            > 12
                        {
                            possible = false;
                        }
                    }
                    x if x.contains("green") => {
                        if x.split_once(" ")
                            .expect("Wrong green format.")
                            .0
                            .parse::<i32>()
                            .expect("Green should be in format '<i32> green'.")
                            > 13
                        {
                            possible = false;
                        }
                    }
                    x if x.contains("blue") => {
                        if x.split_once(" ")
                            .expect("Wrong blue format.")
                            .0
                            .parse::<i32>()
                            .expect("Blue should be in format '<i32> blue'.")
                            > 14
                        {
                            possible = false;
                        }
                    }
                    _ => println!("not matched"),
                }
            }
        }

        match possible {
            true => result += game_id.parse::<i32>().unwrap(),
            false => (),
        }
    }

    println!("result: {}", result);
}
