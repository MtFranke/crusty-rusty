use super::cards;
use std::io;

pub fn play() {
    let mut player: i32 = 0;
    let mut dealer = 0;

    let mut deck = cards::generate_deck();
    let card = cards::get_random_card_from_deck(&mut deck);
    let card_value = cards::get_card_value(&card);
    println!("Deler card");
    println!("{} of {}, value: {}", card.rank, card.suit, card_value);
    dealer += card_value;
    //init game with 2 random cards face up
    println!("Player cards");
    for _ in 0..2 {
        let card = cards::get_random_card_from_deck(&mut deck);
        let card_value = cards::get_card_value(&card);
        println!("{} of {}, value: {}", card.rank, card.suit, card_value);
        player += card_value;
    }

    if (player == 21) {
        println!("Player wins, blackjack!");
        return;
    }

    loop {
        println!("Hit(h) or stand(s) | current value: {player}");
        let next_card: bool = next_card();
        if next_card == false {
            break;
        }

        let card = cards::get_random_card_from_deck(&mut deck);
        let card_value = cards::get_card_value(&card);
        println!("{} of {}, value: {}", card.rank, card.suit, card_value);
        player += card_value;
        if player > 21 {
            println!("Bust! {player}");
            break;
        }
    }

    if player > 21 {
        return;
    }

    let card = cards::get_random_card_from_deck(&mut deck);
    let card_value = cards::get_card_value(&card);
    println!("Deler second card");
    println!("{} of {}, value: {}", card.rank, card.suit, card_value);
    dealer += card_value;

    if dealer < 17 {
        let card = cards::get_random_card_from_deck(&mut deck);
        let card_value = cards::get_card_value(&card);
        println!("Deler threed card");
        println!("{} of {}, value: {}", card.rank, card.suit, card_value);
        dealer += card_value;
    }

    if dealer > 21 {
        println!("Dealer busts! {dealer}!");
        println!("You won!");
        return;
    }

    if player > dealer {
        println!("You WON! against a dealer, player: {player}, dealer: {dealer}");
    } else {
        println!("You LOST! against a dealer, player: {player}, dealer: {dealer}");
    }
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
