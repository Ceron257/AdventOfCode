use std::env;
use aocutils::utility::parse_execution_mode;
pub mod days;

pub fn run() {
    let mut args = env::args();
    let day = args.nth(1).unwrap_or("1".to_string());
    let execution_mode = parse_execution_mode(args.next().unwrap_or("test".to_string()));
    days::execute_day(day, execution_mode);
}
