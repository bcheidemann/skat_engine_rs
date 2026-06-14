use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use itertools::Itertools;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use skat_engine::{
    bot::{Bot, suit::SuitBot},
    game::{Game, suit::SuitGame},
    state::{
        game::{GameState, PlayCardError},
        player::{PlayerId, PlayerState},
    },
    suit::Suit,
};
use skat_engine_cli::utils::{CardDisplayExt, deal::deal};
use tracing::{debug, info, instrument, trace};
use tracing_subscriber::EnvFilter;

fn main() -> io::Result<()> {
    let file_appender = tracing_appender::rolling::daily(".logs", "app.log");
    let (file_appender, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(file_appender)
        .with_ansi(false)
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    info!("starting game");

    let game = Game::Suit(SuitGame {
        hand: false,
        trump_suit: Suit::Clubs,
    });
    let (skat, hand1, hand2, hand3) = deal(&mut rand::rng());
    let players = [
        PlayerState::new(hand1.sorted(&game)),
        PlayerState::new(hand2.sorted(&game)),
        PlayerState::new(hand3.sorted(&game)),
    ];

    let state = GameState::new(game, skat, players, PlayerId::FIRST, PlayerId::FIRST);

    let mut app = App::new(
        state,
        [None, Some(Box::new(SuitBot)), Some(Box::new(SuitBot))],
    );

    ratatui::run(|terminal| app.run(terminal))
}

pub struct App {
    state: GameState,
    bots: [Option<Box<dyn Bot>>; 3],
    focused_card: Option<usize>,
    show_help: bool,
    error: Option<String>,
    exit: bool,
}

impl App {
    pub fn new(state: GameState, bots: [Option<Box<dyn Bot>>; 3]) -> Self {
        Self {
            state,
            bots,
            focused_card: None,
            show_help: false,
            error: None,
            exit: false,
        }
    }

    #[instrument(skip_all)]
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        trace!("app running");

        self.play_bot_turns();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            self.play_bot_turns();
        }

        trace!("exiting");

        Ok(())
    }

    #[instrument(skip_all)]
    fn play_bot_turns(&mut self) {
        while self.play_bot_turn() {}
    }

    #[instrument(skip_all)]
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    #[instrument(skip_all)]
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    #[instrument(skip_all)]
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.focus_prev_card(),
            KeyCode::Right => self.focus_next_card(),
            KeyCode::Up => self.focus_card(),
            KeyCode::Down => self.unfocus_card(),
            KeyCode::Char(' ') => self.play_card(),
            KeyCode::Esc => self.hide_help(),
            KeyCode::Char('?') => self.show_help(),
            _ => {}
        }
    }

    #[instrument(skip_all)]
    fn play_bot_turn(&mut self) -> bool {
        let Some(bot) = &mut self.bots[self.state.current_player_id.into_inner()] else {
            return false;
        };

        trace!(
            message = "playing bot turn",
            player_id = self.state.current_player_id.into_inner()
        );

        let card = bot.play_card(self.state.get_bot_context());

        let trick_compelte = self
            .state
            .play_card(card)
            .expect("bot should play a valid move");

        if trick_compelte {
            self.handle_maybe_game_over();
        }

        true
    }

    #[instrument(skip_all)]
    fn exit(&mut self) {
        self.exit = true;
    }

    #[instrument(skip_all)]
    fn clear_error(&mut self) {
        if self.error.is_some() {
            debug!("clearing error");
            self.error = None;
        }
    }

    #[instrument(skip_all)]
    fn set_focused_card(&mut self, card: Option<usize>) {
        if self.focused_card != card {
            debug!(
                message = "setting focused card",
                prev_focused = self.focused_card,
                new_focused = card
            );
            self.focused_card = card;
        }
    }

    #[instrument(skip_all)]
    fn focus_next_card(&mut self) {
        self.clear_error();

        if self.state.current_player().hand.cards.is_empty() {
            self.set_focused_card(None);
            return;
        }

        let next_card = if let Some(focused_card) = self.focused_card {
            (focused_card + 1) % self.state.current_player().hand.cards.len()
        } else {
            0
        };

        self.set_focused_card(Some(next_card));
    }

    #[instrument(skip_all)]
    fn focus_prev_card(&mut self) {
        self.clear_error();

        if self.state.current_player().hand.cards.is_empty() {
            self.focused_card = None;
            return;
        }

        let next_card = if let Some(focused_card) = self.focused_card {
            if focused_card > 0 {
                focused_card - 1
            } else {
                self.state
                    .current_player()
                    .hand
                    .cards
                    .len()
                    .saturating_sub(1)
            }
        } else {
            self.state
                .current_player()
                .hand
                .cards
                .len()
                .saturating_sub(1)
        };

        self.set_focused_card(Some(next_card));
    }

    #[instrument(skip_all)]
    fn focus_card(&mut self) {
        self.clear_error();

        if self.state.current_player().hand.cards.is_empty() {
            self.set_focused_card(None);
            return;
        }

        if self.focused_card.is_none() {
            self.set_focused_card(Some(0));
        }
    }

    #[instrument(skip_all)]
    fn unfocus_card(&mut self) {
        self.clear_error();
        self.set_focused_card(None);
    }

    #[instrument(skip_all)]
    fn play_card(&mut self) {
        let Some(focused_card) = self.focused_card else {
            self.error = Some("Please select a card to play.".into());
            return;
        };

        let focused_card = self.state.current_player().hand.cards[focused_card];

        debug!(
            message = "player attempting to play card",
            card = ?focused_card
        );

        match self.state.play_card(focused_card) {
            Ok(trick_complete) => {
                if trick_complete {
                    debug!("trick complete");
                    self.handle_maybe_game_over();
                }
                self.set_focused_card(None);
            }
            Err(PlayCardError::InvalidCard) => {
                debug!("player attempted to play invalid card");
                self.error = Some("Invalid card.".into());
            }
            Err(PlayCardError::CardNotOnHand) => {
                unreachable!("played card should be on the players hand");
            }
        }
    }

    #[instrument(skip_all)]
    fn handle_maybe_game_over(&mut self) {
        if self.state.is_game_over() {
            debug!("game is over");
            self.exit();
        }
    }

    #[instrument(skip_all)]
    fn show_help(&mut self) {
        if !self.show_help {
            trace!("showing help menu");
            self.show_help = true;
        }
    }

    #[instrument(skip_all)]
    fn hide_help(&mut self) {
        if self.show_help {
            trace!("hiding help menu");
            self.show_help = false;
        }
    }
}

impl Widget for &App {
    #[instrument(skip_all)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        trace!("rendering app");

        if self.show_help {
            KeybindingsMenu.render(area, buf);
            return;
        }

        let title = Line::from(" Ben’s Skat Engine ".bold());
        let instructions = Line::from(vec![
            " Help ".into(),
            "<?>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .on_black()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let inner_area = block.inner(area);
        block.render(area, buf);

        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .flex(Flex::Center)
        .split(inner_area);

        Text::from("Last Trick").centered().render(layout[0], buf);

        if let Some(trick) = self.state.last_won_trick() {
            let last_trick: Line = Itertools::intersperse(
                trick.cards.iter().map(CardDisplayExt::display_tui),
                " ".into(),
            )
            .collect();

            Text::from(last_trick).centered().render(layout[1], buf);
        } else {
            Text::from("-".gray()).centered().render(layout[1], buf);
        }

        let current_trick = self.state.current_trick();
        if current_trick.cards().is_empty() {
            Text::from(Stylize::gray(Stylize::dim("🂠")))
                .centered()
                .render(layout[2], buf);
        } else {
            let current_trick: Line = Itertools::intersperse(
                current_trick
                    .cards()
                    .iter()
                    .map(CardDisplayExt::display_tui),
                " ".into(),
            )
            .collect();

            Text::from(current_trick).centered().render(layout[2], buf);
        }

        Text::from(format!(
            "Player #{}",
            self.state.current_player_id.into_inner() + 1
        ))
        .centered()
        .render(layout[3], buf);

        let current_player_cards: Line = Itertools::intersperse(
            self.state
                .current_player()
                .hand
                .cards
                .iter()
                .enumerate()
                .map(|(idx, card)| {
                    if Some(idx) == self.focused_card {
                        CardDisplayExt::display_tui(card)
                    } else {
                        Stylize::dim(CardDisplayExt::display_tui(card))
                    }
                }),
            " ".into(),
        )
        .collect();

        Text::from(vec![current_player_cards])
            .centered()
            .render(layout[4], buf);

        if let Some(error) = &self.error {
            Text::from(Stylize::on_light_red(format!(" Error: {error} ")))
                .centered()
                .render(layout[5], buf);
        }
    }
}

#[derive(Debug, Default)]
struct KeybindingsMenu;

impl Widget for &KeybindingsMenu {
    #[instrument(skip_all)]
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        trace!("rendering keybindings menu");

        let title = Line::from(" Keybindings ".bold());
        let block = Block::bordered()
            .on_black()
            .title(title.centered())
            .border_set(border::THICK);
        let inner_area = block.inner(area);
        block.render(area, buf);

        let keys = Text::from(vec![
            Line::from("<Up>".blue().bold()),
            Line::from("<Down>".blue().bold()),
            Line::from("<Left>".blue().bold()),
            Line::from("<Right>".blue().bold()),
            Line::from("<Space>".blue().bold()),
            Line::from(""),
            Line::from("<?>".green().bold()),
            Line::from("<Q>".red().bold()),
            Line::from(""),
            Line::from("<Escape>".gray().bold()),
        ]);

        let descriptions = Text::from(vec![
            Line::from("Focus cards"),
            Line::from("Unfocus cards"),
            Line::from("Focus previous card"),
            Line::from("Focus next card"),
            Line::from("Play card"),
            Line::from(""),
            Line::from("Show this menu"),
            Line::from("Quit"),
            Line::from(""),
            Line::from("Back"),
        ]);

        let content_size = usize::max(keys.height(), descriptions.height()) as u16;

        if inner_area.height < content_size {
            Text::from("Increase terminal size".italic())
                .centered()
                .render(inner_area, buf);

            return;
        }

        let center_area = inner_area.centered_vertically(Constraint::Length(content_size));

        let layout = Layout::horizontal([
            Constraint::Length(keys.width() as u16),
            Constraint::Length(descriptions.width() as u16),
        ])
        .spacing(2)
        .flex(Flex::Center)
        .split(center_area);

        Paragraph::new(keys).right_aligned().render(layout[0], buf);
        Paragraph::new(descriptions)
            .left_aligned()
            .render(layout[1], buf);
    }
}
