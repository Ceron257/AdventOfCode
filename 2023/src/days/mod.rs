use aocutils::{load_input, ExecutionMode};

use day1::day1;
use day2::day2;
use day3::day3;
use day4::day4;
use day5::day5;
use day6::day6;
use day7::day7;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub fn execute_day(day: String, mode: ExecutionMode) {
    println!("Running {day} with {mode:?}");
    let input = load_input(&day, &mode);

    match day.as_str() {
        "1" => day1(input),
        "2" => day2(input),
        "3" => day3(input),
        "4" => day4(input),
        "5" => day5(input),
        "6" => day6(input),
        "7" => day7(input),
        _ => {
            panic!("Day {day} does not (yet) exist.")
        }
    }
}
