use std::cmp::min;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Margin, Rect},
    style::{Color, Style},
    widgets::{BorderType, Row, Scrollbar, ScrollbarOrientation, Table},
    Frame,
};

use crate::app::{App, Mode};

use super::{create_block, StatefulTable, Widget};

pub struct Theme {
    pub name: &'static str,
    pub bg: Color,
    pub fg: Color,
    pub border: BorderType,
    pub border_color: Color,
    pub border_focused_color: Color,
    pub hl_bg: Color,
    pub solid_bg: Color,
    pub solid_fg: Color,
    pub trusted: Color,
    pub remake: Color,
}

pub static THEMES: &[&Theme] = &[
    &Theme {
        name: "Default",
        bg: Color::Reset,
        fg: Color::White,
        border: BorderType::Plain,
        border_color: Color::White,
        border_focused_color: Color::LightCyan,
        hl_bg: Color::DarkGray,
        solid_bg: Color::White,
        solid_fg: Color::Black,
        trusted: Color::Green,
        remake: Color::Red,
    },
    &Theme {
        name: "Dracula",
        bg: Color::Rgb(40, 42, 54),
        fg: Color::Rgb(248, 248, 242),
        border: BorderType::Rounded,
        border_color: Color::Rgb(98, 114, 164),
        border_focused_color: Color::Rgb(189, 147, 249),
        hl_bg: Color::Rgb(98, 114, 164),
        solid_fg: Color::Rgb(40, 42, 54),
        solid_bg: Color::Rgb(139, 233, 253),
        trusted: Color::Rgb(80, 250, 123),
        remake: Color::Rgb(255, 85, 85),
    },
    &Theme {
        name: "Gruvbox",
        bg: Color::Rgb(40, 40, 40),
        fg: Color::Rgb(235, 219, 178),
        border: BorderType::Plain,
        border_color: Color::Rgb(102, 92, 84),
        border_focused_color: Color::Rgb(214, 93, 14),
        hl_bg: Color::Rgb(80, 73, 69),
        solid_bg: Color::Rgb(69, 133, 136),
        solid_fg: Color::Rgb(235, 219, 178),
        trusted: Color::Rgb(152, 151, 26),
        remake: Color::Rgb(204, 36, 29),
    },
    &Theme {
        name: "Catppuccin Macchiato",
        bg: Color::Rgb(24, 25, 38),
        fg: Color::Rgb(202, 211, 245),
        border: BorderType::Rounded,
        border_color: Color::Rgb(110, 115, 141),
        border_focused_color: Color::Rgb(125, 196, 228),
        hl_bg: Color::Rgb(110, 115, 141),
        solid_bg: Color::Rgb(166, 218, 149),
        solid_fg: Color::Rgb(24, 25, 38),
        trusted: Color::Rgb(166, 218, 149),
        remake: Color::Rgb(237, 135, 150),
    },
];

pub struct ThemePopup {
    pub table: StatefulTable<String>,
    pub selected: usize,
}

impl Default for ThemePopup {
    fn default() -> Self {
        ThemePopup {
            table: StatefulTable::with_items(
                THEMES.iter().map(|item| item.name.to_owned()).collect(),
            ),
            selected: 0,
        }
    }
}

impl Widget for ThemePopup {
    fn draw(&self, f: &mut Frame, app: &App, area: Rect) {
        let height = min(min(THEMES.len() as u16 + 2, 10), area.height);
        let center = super::centered_rect(30, height, area);
        let clear = super::centered_rect(center.width + 2, center.height, area);
        let items = self.table.items.iter().enumerate().map(|(i, item)| {
            Row::new(vec![
                match i == self.selected {
                    true => format!("  {}", item),
                    false => format!("   {}", item),
                },
                item.to_owned(),
            ])
        });
        let table = Table::new(items, [Constraint::Percentage(100)])
            .block(create_block(app.theme, true).title("Theme"))
            .highlight_style(Style::default().bg(app.theme.hl_bg));
        super::clear(f, clear, app.theme.bg);
        f.render_stateful_widget(table, center, &mut self.table.state.to_owned());

        // Only show scrollbar if content overflows
        if self.table.items.len() as u16 + 1 >= center.height {
            let sb = Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .track_symbol(Some("│"))
                .begin_symbol(None)
                .end_symbol(None);
            let sb_area = center.inner(&Margin {
                vertical: 1,
                horizontal: 0,
            });
            f.render_stateful_widget(sb, sb_area, &mut self.table.scrollbar_state.to_owned());
        }
    }

    fn handle_event(&mut self, app: &mut crate::app::App, e: &crossterm::event::Event) {
        if let Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) = e
        {
            match code {
                KeyCode::Esc | KeyCode::Char('t') | KeyCode::Char('q') => {
                    app.mode = Mode::Normal;
                }
                KeyCode::Char('j') => {
                    self.table.next_wrap(1);
                }
                KeyCode::Char('k') => {
                    self.table.next_wrap(-1);
                }
                KeyCode::Char('G') => {
                    self.table.select(self.table.items.len() - 1);
                }
                KeyCode::Char('g') => {
                    self.table.select(0);
                }
                KeyCode::Enter => {
                    if let Some(theme) = THEMES.get(self.table.state.selected().unwrap_or(0)) {
                        self.selected = self.table.state.selected().unwrap_or(0);
                        app.theme = theme;
                    }
                }
                _ => {}
            }
        }
    }

    fn get_help() -> Option<Vec<(&'static str, &'static str)>> {
        Some(vec![
            ("Enter", "Confirm"),
            ("Esc, t, q", "Close"),
            ("j, ↓", "Down"),
            ("k, ↑", "Up"),
            ("g", "Top"),
            ("G", "Bottom"),
        ])
    }
}