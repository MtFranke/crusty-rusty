use rand::Rng;
use std::fmt;

const ALL_SUITS: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

pub struct Card {
    pub rank: String,
    pub suit: Suit,
}

#[derive(Copy, Clone, Debug)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suit_str = match self {
            Suit::Clubs => "Clubs",
            Suit::Diamonds => "Diamonds",
            Suit::Hearts => "Hearts",
            Suit::Spades => "Spades",
        };
        write!(f, "{}", suit_str)
    }
}

pub fn generate_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::with_capacity(52);
    for suit_index in 0..4 {
        let suit = ALL_SUITS[suit_index];
        for x in 2..15 {
            let rank;

            match x {
                11 => rank = "Jack".to_string(),
                12 => rank = "Queen".to_string(),
                13 => rank = "King".to_string(),
                14 => rank = "Ace".to_string(),
                _ => rank = x.to_string(),
            }
            deck.push(Card { rank, suit });
        }
    }
    deck
}

fn list_deck(deck: &Vec<Card>) {
    for index in 0..52 {
        let card = &deck[index];
        println!("rank: {}, suit: {}", card.rank, card.suit);
    }
}

pub fn get_card_value(card: &Card) -> i32 {
    match card.rank.as_str() {
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "10" => 10,
        "Jack" => 10,
        "Queen" => 10,
        "King" => 10,
        "Ace" => 11, // Typically, an Ace is worth 11, but this can be adjusted during gameplay
        _ => -1,     // Default case, should not happen if inputs are correct
    }
}

pub fn get_random_card_from_deck(deck: &Vec<Card>) -> &Card {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..52);
    let card: &Card = &deck[random_number];
    card
}
