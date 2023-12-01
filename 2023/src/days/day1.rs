use std::cmp::Ordering;

fn to_digit(s: &str) -> Option<u32> {
    match s {
        "one" => Some(1),
        "1" => Some(1),
        "two" => Some(2),
        "2" => Some(2),
        "three" => Some(3),
        "3" => Some(3),
        "four" => Some(4),
        "4" => Some(4),
        "five" => Some(5),
        "5" => Some(5),
        "six" => Some(6),
        "6" => Some(6),
        "seven" => Some(7),
        "7" => Some(7),
        "eight" => Some(8),
        "8" => Some(8),
        "nine" => Some(9),
        "9" => Some(9),
        _ => None,
    }
}

fn compare(first: &&(usize, &str), second: &&(usize, &str)) -> Ordering {
    first.0.cmp(&second.0)
}

fn extract_number(input: &String) -> Option<u32> {
    let digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    let matches: Vec<_> = digits
        .iter()
        .map(|digit| input.match_indices(digit))
        .flatten()
        .collect();

    let first_digit = to_digit(matches.iter().min_by(compare)?.1)?;
    let second_digit = to_digit(matches.iter().max_by(compare)?.1)?;

    Some(first_digit * 10 + second_digit)
}

pub fn day1(input: Vec<String>) {
    println!("Welcome to day 1.");
    let digits: Vec<_> = (&input)
        .iter()
        .map(|x| x.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .collect();
    let sum: u32 = digits
        .iter()
        .filter_map(|number| Some(number.iter().nth(0)? * 10 + number.iter().nth_back(0)?))
        .sum();
    println!("The answer for part one is {sum}");

    let sum: u32 = input.iter().filter_map(extract_number).sum();
    println!("The answer for part two is {sum:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_number_test() {
        assert_eq!(extract_number(&"two1nine".to_string()), Some(29));
        assert_eq!(extract_number(&"eightwothree".to_string()), Some(83));
        assert_eq!(extract_number(&"abcone2threexyz".to_string()), Some(13));
        assert_eq!(extract_number(&"xtwone3four".to_string()), Some(24));
        assert_eq!(extract_number(&"4nineeightseven2".to_string()), Some(42));
        assert_eq!(extract_number(&"zoneight234".to_string()), Some(14));
        assert_eq!(extract_number(&"7pqrstsixteen".to_string()), Some(76));
    }
}
