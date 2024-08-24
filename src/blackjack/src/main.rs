mod cards;
use std::io;

fn main() {
    println!("Welcome to Blackjack game!");
    let deck = cards::generate_deck();
    //list_deck(&deck);
    println!("Your goal is it hit 21!");

    let mut score: i32 = 0;
    loop {
        let card = cards::get_random_card_from_deck(&deck);
        let card_value = cards::get_card_value(card);
        println!("{} of {}, value: {}", card.rank, card.suit, card_value);
        score += card_value;
        if score > 21 {
            println!("You lost! You reached {score}");
            break;
        }

        println!("Hit(h) or stand(s) | current value: {score}");
        let next_card: bool = next_card();
        println!("{}", next_card);
        if next_card == false {
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
        println!("returning true");
        return true;
    }

    false
}
