use regex::Regex;

#[derive(Debug)]
enum Cube {
    Red(i32),
    Green(i32),
    Blue(i32),
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Red(_), Self::Red(_)) => true,
            (Self::Green(_), Self::Green(_)) => true,
            (Self::Blue(_), Self::Blue(_)) => true,
            _ => false,
        }
    }
}

fn amount(cube: &Cube) -> i32 {
    match cube {
        Cube::Red(a) => *a,
        Cube::Green(a) => *a,
        Cube::Blue(a) => *a,
    }
}

fn cube_with_new_amount(cube: &Cube, amount: i32) -> Cube {
    match cube {
        Cube::Red(_) => Cube::Red(amount),
        Cube::Green(_) => Cube::Green(amount),
        Cube::Blue(_) => Cube::Blue(amount),
    }
}

fn parse_cube(color: &str, amount: i32) -> Option<Cube> {
    match color {
        "red" => Some(Cube::Red(amount)),
        "green" => Some(Cube::Green(amount)),
        "blue" => Some(Cube::Blue(amount)),
        _ => None,
    }
}

type Draw = Vec<Cube>;

fn parse_draw(input: &str) -> Option<Draw> {
    let re_draw = Regex::new(
        r"(?x)
          (?<amount>\d+)\s
          (?<type>[a-z]+)
          (;|,)?",
    )
    .ok()?;

    let mut draw = Draw::new();
    for draw_capture in re_draw.captures_iter(input) {
        let amount = draw_capture.name("amount")?.as_str().parse::<i32>().ok()?;
        let cube_type = draw_capture.name("type")?;

        draw.push(parse_cube(cube_type.as_str(), amount)?);
    }
    Some(draw)
}

#[derive(Debug)]
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

fn parse_game(line: &String) -> Option<Game> {
    let re_game = Regex::new(r"Game\s(?<id>\d+):").ok()?;

    let id = re_game
        .captures(line.as_str())?
        .name("id")?
        .as_str()
        .parse::<usize>()
        .ok()?;
    let mut draws: Vec<Draw> = Vec::new();
    for draws_slice in line.split_terminator(";") {
        draws.push(parse_draw(draws_slice)?);
    }

    if draws.is_empty() {
        return None;
    }

    Some(Game { id, draws })
}

fn is_draw_possible(maximum_draw: &Draw, draw: &Draw) -> bool {
    for cube in draw {
        let position = maximum_draw.iter().position(|c| c == cube);
        if position.is_none() {
            return false;
        }
        let maximum_amount = amount(&maximum_draw[position.unwrap()]);
        let cube_amount = amount(&cube);
        if cube_amount > maximum_amount {
            return false;
        }
    }
    true
}

fn find_minimum_configuration(game: &Game) -> Option<Draw> {
    let mut configuration = vec![Cube::Red(0), Cube::Green(0), Cube::Blue(0)];
    for draw in &game.draws {
        for cube in draw {
            let index = &configuration.iter().position(|c| c == cube)?;
            let cube_amount = amount(&cube);
            if cube_amount > amount(&configuration[index.clone()]) {
                configuration[index.clone()] =
                    cube_with_new_amount(&configuration[index.clone()], cube_amount);
            }
        }
    }
    Some(configuration)
}

pub fn day2(input: Vec<String>) {
    let games: Vec<_> = input.iter().filter_map(parse_game).collect();
    let bag_configutarion = vec![Cube::Red(12), Cube::Green(13), Cube::Blue(14)];
    let possible_games: Vec<_> = games
        .iter()
        .filter(|game| {
            game.draws
                .iter()
                .all(|draw| is_draw_possible(&bag_configutarion, draw))
        })
        .collect();

    let id_sum: usize = possible_games.iter().map(|game| game.id).sum();
    println!("The sum of possible game IDs is {id_sum}");

    let configurations: Vec<_> = games
        .iter()
        .filter_map(find_minimum_configuration)
        .collect();
    let power_sum: i32 = configurations
        .iter()
        .map(|c| c.iter().map(|cube| amount(cube)).product::<i32>())
        .sum();
    println!("The sum of powers is {power_sum}");
}
