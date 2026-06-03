use owo_colors::OwoColorize;
use ratatui::{style::Stylize, text::Span};
use skat_engine::{card::Card, suit::Suit};

pub mod deal;

pub trait CardDisplayExt {
    fn display_term(&self) -> String;
    fn display_rust(&self) -> String;
    fn display_tui(&self) -> Span<'_>;
}

impl CardDisplayExt for Card {
    fn display_term(&self) -> String {
        let c = self.char();
        match self.suit {
            Suit::Diamonds | Suit::Hearts => c
                .if_supports_color(owo_colors::Stream::Stdout, |text| OwoColorize::red(text))
                .to_string(),
            Suit::Spades | Suit::Clubs => c
                .if_supports_color(owo_colors::Stream::Stdout, |text| OwoColorize::black(text))
                .to_string(),
        }
    }

    fn display_rust(&self) -> String {
        format!("Card!({} of {})", self.rank.name(), self.suit.name())
    }

    fn display_tui(&self) -> Span<'_> {
        let c = self.char();
        match self.suit {
            Suit::Diamonds | Suit::Hearts => Stylize::red(c),
            Suit::Spades | Suit::Clubs => Stylize::black(c),
        }
    }
}
