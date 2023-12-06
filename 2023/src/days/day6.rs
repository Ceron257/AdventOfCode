fn parse_numbers(input: &String) -> Vec<usize> {
    input
        .split(" ")
        .filter(|e| *e != "")
        .skip(1)
        .filter_map(|e| e.parse::<usize>().ok())
        .collect()
}

fn parse_number(input: &String) -> Option<usize> {
    input
        .split(" ")
        .filter(|e| *e != "")
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .ok()
}
fn compute_number_of_wins(time: usize, distance: usize) -> usize {
    /*
     * We want to intersect `time * (time - t)` with `distance`.
     * The maximum will be reached at `time / 2.` and the intersection
     * points have the same distance to the maximum for symmetry reasons.
     * 
     * So we can look for the first intersection point and then calculate
     * calculate the number of ways to win from that.
     */
    let maximum_time = time as f64 / 2.;
    let left_winning_time = (1..time)
        .filter_map(|t| {
            if t * (time - t) > distance {
                Some(t)
            } else {
                None
            }
        })
        .take(1)
        .next()
        .unwrap();
    ((maximum_time - left_winning_time as f64) * 2. + 1.) as usize
}

pub fn day6(input: Vec<String>) {
    let times = parse_numbers(&input[0]);
    let distances = parse_numbers(&input[1]);

    let result: usize = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| compute_number_of_wins(*t, *d))
        .product();

    println!("The answer for part one is {result}.");

    let time = parse_number(&input[0])
        .or_else(|| panic!("Could not parse time."))
        .unwrap();
    let distance = parse_number(&input[1])
        .or_else(|| panic!("Could not parse distance."))
        .unwrap();

    let number_of_wins = compute_number_of_wins(time, distance);
    println!("There are {number_of_wins} ways to win the long race.");
}
