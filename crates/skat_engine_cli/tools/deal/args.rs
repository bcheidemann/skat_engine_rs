use clap::{Parser, ValueEnum};

/// Deal each player a hand.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Output format for the dealt hands.
    #[arg(short, long, value_enum, default_value_t = Output::Pretty)]
    pub output: Output,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Output {
    /// Outputs the dealt hands in a visually appealing manner.
    Pretty,
    /// Outputs the dealt hands as Rust code.
    Rust,
}
