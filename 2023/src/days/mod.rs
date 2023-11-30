use crate::utility::read_input;
use crate::ExecutionMode;

use day1::day1;

pub mod day1;

fn load_input(day: &String, mode: &ExecutionMode) -> Vec<String> {
    let suffix = match mode {
        ExecutionMode::Normal => "",
        ExecutionMode::Test => "-test",
    };

    let input_path = format!("inputs/day-{day}{suffix}.txt");
    let input = read_input(input_path);

    if input.is_err() {
        let error = input.err().unwrap();
        panic!("Failed to read input: {error}");
    }

    input.unwrap()
}

pub fn execute_day(day: String, mode: ExecutionMode) {
    println!("Running {day} with {mode:?}");
    let input = load_input(&day, &mode);

    match day.as_str() {
        "1" => day1(input),
        _ => {
            panic!("Day {day} does not (yet) exist.")
        }
    }
}
