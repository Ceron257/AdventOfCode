use std::env;

pub mod days;
pub mod utility;

#[derive(Debug)]
pub enum ExecutionMode {
    /// Runs the task with our personalized input.
    Normal,
    /// Runs the task with the official test input.
    Test,
}

pub fn run() {
    let mut args = env::args();
    let day = args.nth(1).unwrap_or("1".to_string());
    let execution_mode = utility::parse_execution_mode(args.next().unwrap_or("test".to_string()));
    days::execute_day(day, execution_mode);
}
