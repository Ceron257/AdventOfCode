use regex::Regex;

#[derive(Debug, Clone)]
struct Card {
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>,
    amount: usize,
}

impl Card {
    fn winning_numbers(&self) -> i32 {
        self.numbers
            .iter()
            .filter_map(|n| {
                self.winning_numbers
                    .iter()
                    .any(|winner| winner == n)
                    .then(|| n)
            })
            .count() as i32
    }

    fn score(&self) -> i32 {
        let winning_count = self.winning_numbers();

        if winning_count == 0 {
            return 0;
        }

        (2 as i32).pow((winning_count as u32).saturating_sub(1))
    }
}

fn parse_numbers(input: &str) -> Option<Vec<i32>> {
    Some(
        input
            .split_terminator(" ")
            .filter_map(|n| n.parse::<i32>().ok())
            .collect(),
    )
}

fn parse_card(input: &String) -> Option<Card> {
    let re = Regex::new(r"Card\s+(?<id>\d+):").ok()?;
    let id_match = re.captures(input)?.name("id")?;

    let numbers_input = input[id_match.end() + 1..].to_string();
    let mut number_lists = numbers_input.split_terminator("|");

    let winning_numbers = parse_numbers(number_lists.next()?)?;
    let numbers = parse_numbers(number_lists.next()?)?;

    Some(Card {
        winning_numbers,
        numbers,
        amount: 1,
    })
}

pub fn day4(input: Vec<String>) {
    let cards: Vec<_> = input.iter().filter_map(parse_card).collect();
    let score: i32 = cards.iter().map(Card::score).sum();
    println!("The score of all cards sums up to {score}");

    let mut final_cards = cards.clone();

    cards
        .iter()
        .enumerate()
        .map(|(index, card)| {
            let winning_numbers = card.winning_numbers() as usize;
            let num_copies = *(&final_cards[index].amount);
            final_cards
                .iter_mut()
                .skip(index + 1)
                .take(winning_numbers)
                .map(|winner| winner.amount += num_copies)
                .for_each(|_| ());
        })
        .for_each(|_| ());

    let total_cards_num: usize = final_cards.iter().map(|c| c.amount).sum();
    println!("At the end we have {total_cards_num} cards.");
}
