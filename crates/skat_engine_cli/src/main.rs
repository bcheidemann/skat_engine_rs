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
    game::{Game, grand::GrandGame},
    state::{
        game::GameState,
        player::{PlayerId, PlayerState},
    },
};
use skat_engine_cli::utils::{CardDisplayExt, deal::deal};

fn main() -> io::Result<()> {
    let game = Game::Grand(GrandGame {});
    let (skat, hand1, hand2, hand3) = deal(&mut rand::rng());
    let players = [
        PlayerState::new(hand1.sorted(&game)),
        PlayerState::new(hand2.sorted(&game)),
        PlayerState::new(hand3.sorted(&game)),
    ];

    let mut state = GameState::new(game, skat, players, PlayerId::FIRST, PlayerId::FIRST);
    state.play_card(0).unwrap();
    state.play_card(0).unwrap();
    state.play_card(0).unwrap();
    state.play_card(0).unwrap();

    let mut app = App::new(state);

    ratatui::run(|terminal| app.run(terminal))
}

#[derive(Debug)]
pub struct App {
    state: GameState,
    focused_card: Option<usize>,
    show_help: bool,
    exit: bool,
}

impl App {
    pub fn new(state: GameState) -> Self {
        Self {
            state,
            focused_card: None,
            show_help: false,
            exit: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

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

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.focus_prev_card(),
            KeyCode::Right => self.focus_next_card(),
            KeyCode::Up => self.focus_card(),
            KeyCode::Down => self.unfocus_card(),
            KeyCode::Esc => self.hide_help(),
            KeyCode::Char('?') => self.show_help(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn focus_next_card(&mut self) {
        if let Some(focused_card) = self.focused_card {
            self.focused_card =
                Some((focused_card + 1) % self.state.current_player().hand.cards.len());
        } else {
            self.focused_card = Some(0);
        }
    }

    fn focus_prev_card(&mut self) {
        if let Some(focused_card) = self.focused_card {
            if focused_card > 0 {
                self.focused_card = Some(focused_card - 1);
            } else {
                self.focused_card = Some(self.state.current_player().hand.cards.len() - 1);
            };
        } else {
            self.focused_card = Some(self.state.current_player().hand.cards.len() - 1);
        }
    }

    fn focus_card(&mut self) {
        if self.focused_card.is_none() {
            self.focused_card = Some(0);
        }
    }

    fn unfocus_card(&mut self) {
        self.focused_card = None;
    }

    fn show_help(&mut self) {
        self.show_help = true;
    }

    fn hide_help(&mut self) {
        self.show_help = false;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
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

        if let Some(card) = self.state.current_trick().top_card() {
            Text::from(card.display_tui())
                .centered()
                .render(layout[2], buf);
        }

        Text::from(format!(
            "Player #{}",
            self.state.current_player_id().into_inner() + 1
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
                    if self.focused_card.is_none() || Some(idx) == self.focused_card {
                        CardDisplayExt::display_tui(card)
                    } else {
                        card.char().gray()
                    }
                }),
            " ".into(),
        )
        .collect();

        Text::from(vec![current_player_cards])
            .centered()
            .render(layout[4], buf);
    }
}

#[derive(Debug, Default)]
struct KeybindingsMenu;

impl Widget for &KeybindingsMenu {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(" Keybindings ".bold());
        let block = Block::bordered()
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
