use itertools::Itertools;
use skat_engine::{
    card::Card,
    game::{Game, grand::GrandGame},
    utils::deal::deal,
};

pub fn main() {
    let game = Game::Grand(GrandGame {});

    let (skat, hand1, hand2, hand3) = deal(&mut rand::rng());

    println!("Skat:     {}", display_cards(skat));
    println!("Player 1: {}", display_cards(hand1.sorted(&game).cards));
    println!("Player 2: {}", display_cards(hand2.sorted(&game).cards));
    println!("Player 3: {}", display_cards(hand3.sorted(&game).cards));
}

fn display_cards(cards: impl AsRef<[Card]>) -> String {
    Itertools::intersperse(
        cards.as_ref().iter().map(Card::display_term),
        " ".to_string(),
    )
    .collect()
}
