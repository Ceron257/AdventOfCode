use regex::Regex;
use std::{ops::Range, usize};

#[derive(Debug, Clone, PartialEq)]
enum Kind {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

fn parse_seeds(input: &String) -> Option<Vec<(Kind, usize)>> {
    let seeds: Vec<_> = input
        .split(|c| c == ' ')
        .skip(1)
        .map(|n| n.parse::<usize>().unwrap())
        .map(|amount| (Kind::Seed, amount))
        .collect();
    Some(seeds)
}

fn parse_seed_ranges(input: &String) -> Option<Vec<(Kind, Range<usize>)>> {
    let numbers = input.split(|c| c == ' ').skip(1).collect::<Vec<_>>();
    let chunks: Vec<_> = numbers.chunks(2).collect();
    let ranges: Vec<_> = chunks
        .iter()
        .filter_map(|c| Some((c[0].parse::<usize>().ok()?, c[1].parse::<usize>().ok()?)))
        .map(|(start, len)| (Kind::Seed, start..start + len))
        .collect();
    Some(ranges)
}

#[derive(Debug)]
struct Mapping {
    from: Vec<(Kind, Range<usize>)>,
    to: Vec<(Kind, Range<usize>)>,
}

impl Mapping {
    fn map(&self, item: (Kind, usize)) -> Option<(Kind, usize)> {
        let candidate = self
            .from
            .iter()
            .position(|(kind, range)| *kind == item.0 && range.contains(&item.1));
        if candidate.is_none() {
            return Some((self.to[0].0.clone(), item.1));
        }

        let (_, from_range) = &self.from[candidate.unwrap()];
        let (to_kind, to_range) = &self.to[candidate.unwrap()];

        Some((to_kind.clone(), item.1 + to_range.start - from_range.start))
    }
}

fn parse_kind(input: &str) -> Option<Kind> {
    match input {
        "seed" => Some(Kind::Seed),
        "soil" => Some(Kind::Soil),
        "fertilizer" => Some(Kind::Fertilizer),
        "water" => Some(Kind::Water),
        "light" => Some(Kind::Light),
        "temperature" => Some(Kind::Temperature),
        "humidity" => Some(Kind::Humidity),
        "location" => Some(Kind::Location),
        _ => None,
    }
}

fn parse_mapping_entry(input: &String) -> Option<(Range<usize>, Range<usize>)> {
    let mut parts = input.split(" ");
    let destination_start = parts.next()?.parse::<usize>().ok()?;
    let source_start = parts.next()?.parse::<usize>().ok()?;
    let length = parts.next()?.parse::<usize>().ok()?;

    Some((
        source_start..source_start + length,
        destination_start..destination_start + length,
    ))
}

fn parse_mappings(input: &[String]) -> Option<Mapping> {
    let re = Regex::new(r"(?<from>[^-]+)-to-(?<to>[^\s]+) map:").ok()?;

    let captures = re.captures(&input[0])?;
    let from = parse_kind(captures.name("from")?.as_str())?;
    let to = parse_kind(captures.name("to")?.as_str())?;

    let (from, to): (Vec<_>, Vec<_>) = input
        .iter()
        .skip(1)
        .filter_map(parse_mapping_entry)
        .map(|(l, r)| ((from.clone(), l), (to.clone(), r)))
        .unzip();

    Some(Mapping { from, to })
}

fn map_seed(mappings: &Vec<Mapping>, seed: &(Kind, usize)) -> (Kind, usize) {
    mappings.iter().fold(seed.clone(), |acc, m| {
        let result = m.map(acc).unwrap();
        result
    })
}

pub fn day5(input: Vec<String>) {
    let input_fragments: Vec<_> = input.split(|line| line.as_str() == "").collect();

    if input_fragments.len() == 0 {
        panic!("Invalid input.");
    }

    let seeds = parse_seeds(&input_fragments[0][0])
        .or_else(|| {
            panic!("Failed to parse seeds.");
        })
        .unwrap();

    let mappings: Vec<_> = input_fragments
        .iter()
        .skip(1)
        .filter_map(|fragment| parse_mappings(*fragment))
        .collect();
    let locations: Vec<_> = seeds.iter().map(|s| map_seed(&mappings, s)).collect();
    let min = locations.iter().map(|(_, l)| l).min().unwrap();
    println!("The minimum location is {min}.");

    let seed_ranges = parse_seed_ranges(&input_fragments[0][0])
        .or_else(|| {
            panic!("Failed to parse seed ranges.");
        })
        .unwrap();

    let location = seed_ranges
        .iter()
        .map(|(kind, range)| {
            range
                .clone()
                .map(|s| map_seed(&mappings, &(kind.clone(), s)))
        })
        .flatten()
        .map(|(_, l)| l)
        .min()
        .unwrap();

    println!("The minimum location for seeds parsed as ranges is {location}.");
}
