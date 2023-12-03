use std::ops::Range;

#[derive(Debug)]
struct Bound2D {
    x_range: Range<usize>,
    y_range: Range<usize>,
}

fn enlarge_range(range: &Range<usize>, extent: usize) -> Range<usize> {
    range.start.saturating_sub(extent)..range.end + extent
}

impl Bound2D {
    fn contains(&self, p: (usize, usize)) -> bool {
        self.x_range.contains(&p.0) && self.y_range.contains(&p.1)
    }

    fn enlarge(&self, extent: (usize, usize)) -> Self {
        Bound2D {
            x_range: enlarge_range(&self.x_range, extent.0),
            y_range: enlarge_range(&self.y_range, extent.1),
        }
    }
}

#[derive(Debug)]
struct Number {
    bound: Bound2D,
    value: u32,
}

pub fn day3(input: Vec<String>) {
    let numbers: Vec<_> = input
        .iter()
        .enumerate()
        .map(|(y, line)| {
            let digit_indices: Vec<_> = line
                .chars()
                .into_iter()
                .enumerate()
                .filter_map(|(x, char)| match char.is_digit(10) {
                    true => Some(x),
                    false => None,
                })
                .collect();
            let ranges = digit_indices
                .iter()
                .fold(Vec::<Range<usize>>::new(), |mut acc, x| {
                    if acc.is_empty() {
                        acc.push(*x..*x + 1);
                    } else {
                        let last_element = acc.last_mut().unwrap();
                        if last_element.end == *x {
                            *last_element = last_element.start..x + 1;
                        } else {
                            acc.push(*x..*x + 1);
                        }
                    }
                    acc
                });
            ranges
                .into_iter()
                .map(move |range| Number {
                    bound: Bound2D {
                        x_range: range.clone(),
                        y_range: y..y + 1,
                    },
                    value: line.as_str()[range].parse::<u32>().unwrap(),
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let symbol_positions: Vec<_> = input
        .iter()
        .enumerate()
        .map(|(y, line)| {
            let symbol_indices: Vec<_> = line
                .chars()
                .into_iter()
                .enumerate()
                .filter_map(|(x, char)| {
                    if char != '.' && !char.is_digit(10) {
                        Some(x)
                    } else {
                        None
                    }
                })
                .collect();
            symbol_indices
                .into_iter()
                .map(move |symbol_x| (symbol_x, y))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let numbers_filtered: Vec<_> = numbers
        .iter()
        .filter_map(|number| {
            if symbol_positions
                .iter()
                .any(|position| number.bound.enlarge((1, 1)).contains(*position))
            {
                Some(number)
            } else {
                None
            }
        })
        .collect();

    let gear_positions: Vec<_> = input
        .iter()
        .enumerate()
        .map(|(y, line)| {
            let symbol_indices: Vec<_> = line
                .chars()
                .into_iter()
                .enumerate()
                .filter_map(|(x, char)| if char == '*' { Some(x) } else { None })
                .collect();
            symbol_indices
                .into_iter()
                .map(move |symbol_x| (symbol_x, y))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let gear_ratio_sum: u32 = gear_positions
        .iter()
        .filter_map(|position| {
            let neighbors: Vec<_> = numbers
                .iter()
                .filter(|number| number.bound.enlarge((1, 1)).contains(*position))
                .collect();
            if neighbors.len() == 2 {
                Some((neighbors[0], neighbors[1]))
            } else {
                None
            }
        })
        .map(|(first, second)| (*first).value * (*second).value)
        .sum();
    let sum: u32 = numbers_filtered.iter().map(|&number| (*number).value).sum();

    println!("The answer for part one is {sum}");
    println!("The answer for part two is {gear_ratio_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bound_contains() {
        let bound = Bound2D {
            x_range: 0..2,
            y_range: 0..2,
        };

        assert!(bound.contains((0, 0)));
        assert!(bound.contains((0, 1)));
        assert!(bound.contains((1, 0)));
        assert!(bound.contains((1, 1)));

        assert!(!bound.contains((0, 2)));
        assert!(!bound.contains((2, 0)));
        assert!(!bound.contains((1, 2)));
        assert!(!bound.contains((2, 2)));
    }
}
