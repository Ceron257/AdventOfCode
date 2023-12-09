use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Joker,
    Queen,
    King,
    Ace,
}

fn strength(card: &Card) -> usize {
    match card {
        Card::Two => 1,
        Card::Three => 2,
        Card::Four => 3,
        Card::Five => 4,
        Card::Six => 5,
        Card::Seven => 6,
        Card::Eight => 7,
        Card::Nine => 8,
        Card::Ten => 9,
        Card::Joker => 10,
        Card::Queen => 11,
        Card::King => 12,
        Card::Ace => 13,
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

fn parse_card(input: char) -> Option<Card> {
    match input {
        '2' => Some(Card::Two),
        '3' => Some(Card::Three),
        '4' => Some(Card::Four),
        '5' => Some(Card::Five),
        '6' => Some(Card::Six),
        '7' => Some(Card::Seven),
        '8' => Some(Card::Eight),
        '9' => Some(Card::Nine),
        'T' => Some(Card::Ten),
        'J' => Some(Card::Joker),
        'Q' => Some(Card::Queen),
        'K' => Some(Card::King),
        'A' => Some(Card::Ace),
        _ => None,
    }
}

fn count_amounts(cards: Vec<Card>) -> Vec<(Card, usize)> {
    let mut amounts = HashMap::new();
    for card in cards {
        *amounts.entry(card).or_insert(0 as usize) += 1;
    }

    let mut result = amounts
        .keys()
        .zip(amounts.values())
        .map(|(c, a)| (*c, *a))
        .collect::<Vec<_>>();
    result.sort_by(|(_, l), (_, r)| l.cmp(r).reverse());

    result
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Hand {
    cards: Vec<Card>,
    cards_sorted: Vec<Card>,
    amounts: Vec<(Card, usize)>,
    bid: usize,
    line: String,
}

impl Hand {
    fn hand_type(&self) -> Option<HandType> {
        let mut amount_it = self.amounts.iter();

        match amount_it.next()? {
            (_, 5) => Some(HandType::Five),
            (_, 4) => Some(HandType::Four),
            (_, 3) => match amount_it.next()? {
                (_, 2) => Some(HandType::FullHouse),
                (_, 1) => Some(HandType::Three),
                _ => None,
            },
            (_, 2) => match amount_it.next()? {
                (_, 2) => Some(HandType::TwoPair),
                (_, 1) => Some(HandType::OnePair),
                _ => None,
            },
            _ => Some(HandType::HighCard),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let (Some(l), Some(r)) = (self.hand_type(), other.hand_type()) {
            match l.cmp(&r) {
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => (),
                Ordering::Greater => return Ordering::Greater,
            };
        }

        let mut my_card = self.cards.iter();
        let mut other_card = other.cards.iter();

        loop {
            let (left, right) = (my_card.next(), other_card.next());
            match (left, right) {
                (None, _) => {
                    return Ordering::Less;
                }
                (_, None) => {
                    return Ordering::Greater;
                }
                (Some(l), Some(r)) => match l.cmp(&r) {
                    Ordering::Less => {
                        return Ordering::Less;
                    }
                    Ordering::Equal => (),
                    Ordering::Greater => {
                        return Ordering::Greater;
                    }
                },
            }
        }
    }
}

fn parse_hand(input: &String) -> Option<Hand> {
    let mut s = input.split(" ");
    let cards: Vec<_> = s
        .next()?
        .chars()
        .into_iter()
        .filter_map(parse_card)
        .collect();
    let mut cards_sorted = cards.clone();
    cards_sorted.sort_by(|l, r| strength(l).cmp(&strength(r)).reverse());
    let bid = s.next()?.parse::<usize>().ok()?;

    let amounts = count_amounts(cards.clone());

    Some(Hand {
        cards,
        cards_sorted,
        amounts,
        bid,
        line: input.clone(),
    })
}

pub fn day7(input: Vec<String>) {
    let mut hands: Vec<_> = input.iter().filter_map(parse_hand).collect();
    hands.sort_by(|l, r| l.cmp(&r));

    let total_winnings: usize = hands.iter().enumerate().map(|(r, c)| (r + 1) * c.bid).sum();
    println!("The total winnings are {total_winnings}.");
}
