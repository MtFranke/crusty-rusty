use std::fmt;
const ALL_SUITS: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

fn main() {
    println!("Welcome to Blackjack game!");
    let deck = generate_deck();
    list_deck(deck);
}

fn generate_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::with_capacity(52);
    for suit_index in 0..4 {
        let suit = ALL_SUITS[suit_index];
        for x in 2..15 {
            let mut rank: String = String::from(x.to_string());

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

fn list_deck(deck: Vec<Card>) {
    for index in 0..52 {
        let card = &deck[index];
        println!("rank: {}, suit: {}", card.rank, card.suit);
    }
}

struct Card {
    rank: String,
    suit: Suit,
}

#[derive(Copy, Clone, Debug)]
enum Suit {
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
