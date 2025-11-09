use crate::utility::read_input;

pub mod utility;

pub fn load_input(day: &String, mode: &ExecutionMode) -> Vec<String> {
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

#[derive(Debug)]
pub enum ExecutionMode {
    /// Runs the task with our personalized input.
    Normal,
    /// Runs the task with the official test input.
    Test,
}
