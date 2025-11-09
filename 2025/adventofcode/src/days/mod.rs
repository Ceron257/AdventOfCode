use aocutils::ExecutionMode;
use aocutils::load_input;

use day1::day1;

pub mod day1;

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
