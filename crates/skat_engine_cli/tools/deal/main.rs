use clap::Parser as _;
use itertools::Itertools;
use skat_engine::{
    card::Card,
    game::{Game, grand::GrandGame},
};
use skat_engine_cli::utils::{
    CardDisplayExt,
    deal::{DealtHands, deal},
};

use crate::args::Args;

mod args;

pub fn main() {
    let args = Args::parse();

    let game = Game::Grand(GrandGame { hand: false });
    let dealt_hands = deal(&mut rand::rng());

    match args.output {
        args::Output::Pretty => display_dealt_hands_pretty(game, dealt_hands),
        args::Output::Rust => display_dealt_hands_rust(game, dealt_hands),
    }
}

fn display_dealt_hands_pretty(game: Game, dealt_hands: DealtHands) {
    fn display_cards(cards: impl AsRef<[Card]>) -> String {
        Itertools::intersperse(
            cards.as_ref().iter().map(CardDisplayExt::display_term),
            " ".to_string(),
        )
        .collect()
    }

    let (skat, hand1, hand2, hand3) = dealt_hands;

    println!("Skat:     {}", display_cards(skat));
    println!("Player 1: {}", display_cards(hand1.sorted(&game).cards));
    println!("Player 2: {}", display_cards(hand2.sorted(&game).cards));
    println!("Player 3: {}", display_cards(hand3.sorted(&game).cards));
}

fn display_dealt_hands_rust(game: Game, dealt_hands: DealtHands) {
    fn display_cards(cards: impl AsRef<[Card]>, new_lines: bool) -> String {
        Itertools::intersperse(
            cards.as_ref().iter().map(CardDisplayExt::display_rust),
            if new_lines {
                ",\n    ".to_string()
            } else {
                ", ".to_string()
            },
        )
        .collect()
    }

    let (skat, hand1, hand2, hand3) = dealt_hands;

    println!("let skat = [{}];", display_cards(skat, false));
    println!(
        "let hand1 = [\n    {},\n];",
        display_cards(hand1.sorted(&game).cards, true)
    );
    println!(
        "let hand2 = [\n    {},\n];",
        display_cards(hand2.sorted(&game).cards, true)
    );
    println!(
        "let hand3 = [\n    {},\n];",
        display_cards(hand3.sorted(&game).cards, true)
    );
}
