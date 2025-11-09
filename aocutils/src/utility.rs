use crate::ExecutionMode;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    process::exit,
};

pub fn read_input<P>(file_name: P) -> Result<Vec<String>, String>
where
    P: AsRef<Path>,
{
    let file = File::open(&file_name);
    match file {
        Ok(file) => {
            let mut lines: Vec<String> = Vec::new();
            for line in io::BufReader::new(file).lines() {
                match line {
                    Ok(line) => lines.push(line),
                    Err(err) => {
                        return Err(format!(
                            "Unable to read line from file {:?}: {}",
                            file_name.as_ref().to_str(),
                            err
                        ))
                    }
                }
            }
            Ok(lines)
        }
        Err(err) => Err(format!(
            "Unable to open file {:?}: {}",
            file_name.as_ref().to_str(),
            err
        )),
    }
}

pub fn parse_execution_mode(mode: String) -> ExecutionMode {
    match mode.as_str() {
        "normal" => ExecutionMode::Normal,
        "test" => ExecutionMode::Test,
        _ => {
            println!("Wrong execution mode.");
            exit(-1);
        }
    }
}
