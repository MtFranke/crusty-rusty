mod cards;
use std::io;

fn main() {
    println!("Welcome to Blackjack game!, your goal is it hit 21!");
    let mut deck = cards::generate_deck();
    //list_deck(&deck);
    let mut score: i32 = 0;

    //init game with 2 random cards face up
    for _ in 0..2 {
        let card = cards::get_random_card_from_deck(&mut deck);
        let card_value = cards::get_card_value(&card);
        println!("{} of {}, value: {}", card.rank, card.suit, card_value);
        score += card_value;
    }

    loop {
        println!("Hit(h) or stand(s) | current value: {score}");
        let next_card: bool = next_card();
        if next_card == false {
            break;
        }

        let card = cards::get_random_card_from_deck(&mut deck);
        let card_value = cards::get_card_value(&card);
        println!("{} of {}, value: {}", card.rank, card.suit, card_value);
        score += card_value;
        if score > 21 {
            println!("You lost! You reached {score}");
            break;
        }
    }

    println!("You scored {score}!");
}

fn next_card() -> bool {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    if buffer.trim() == "h" {
        return true;
    }

    false
}
