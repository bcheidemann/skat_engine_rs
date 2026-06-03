use owo_colors::OwoColorize as _;
use skat_engine::{card::Card, suit::Suit};

pub mod deal;

pub trait CardDisplayExt {
    fn display_term(&self) -> String;
}

impl CardDisplayExt for Card {
    fn display_term(&self) -> String {
        let c = self.char();
        match self.suit {
            Suit::Diamonds | Suit::Hearts => c
                .if_supports_color(owo_colors::Stream::Stdout, |text| text.red())
                .to_string(),
            Suit::Spades | Suit::Clubs => c
                .if_supports_color(owo_colors::Stream::Stdout, |text| text.black())
                .to_string(),
        }
    }
}
