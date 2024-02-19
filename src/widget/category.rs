use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Stylize as _},
    text::{Line, Span, Text},
    widgets::{Row, Table},
    Frame,
};

use crate::app::{App, Mode};

use super::{create_block, Widget};

pub struct CatEntry {
    name: &'static str,
    pub cfg: &'static str,
    pub id: usize,
    pub icon: CatIcon,
}

#[derive(Clone)]
pub struct CatIcon {
    pub label: &'static str,
    pub color: Color,
}

impl Default for CatIcon {
    fn default() -> Self {
        CatIcon {
            label: "???",
            color: Color::White,
        }
    }
}

impl CatEntry {
    const fn new(
        name: &'static str,
        cfg: &'static str,
        id: usize,
        label: &'static str,
        color: Color,
    ) -> Self {
        CatEntry {
            name,
            cfg,
            id,
            icon: CatIcon { label, color },
        }
    }
}

pub struct CatStruct {
    name: &'static str,
    pub entries: &'static [CatEntry],
}

impl CatStruct {
    pub fn find(&self, category: usize) -> Option<CatIcon> {
        self.entries
            .iter()
            .find(|e| e.id == category as usize)
            .and_then(|e| Some(e.icon.clone()))
    }
}

pub static ANIME: CatStruct = CatStruct {
    name: "Anime",
    entries: &[
        CatEntry::new("All Anime", "AllAnime", 10, "Ani", Color::White),
        CatEntry::new(
            "English Translated",
            "AnimeEnglishTranslated",
            12,
            "Sub",
            Color::LightMagenta,
        ),
        CatEntry::new(
            "Non-English Translated",
            "AnimeNonEnglishTranslated",
            13,
            "Sub",
            Color::LightGreen,
        ),
        CatEntry::new("Raw", "AnimeRaw", 14, "Raw", Color::Gray),
        CatEntry::new(
            "Anime Music Video",
            "AnimeMusicVideo",
            11,
            "AMV",
            Color::Magenta,
        ),
    ],
};

pub static AUDIO: CatStruct = CatStruct {
    name: "Audio",
    entries: &[
        CatEntry::new("All Audio", "AllAudio", 20, "Aud", Color::White),
        CatEntry::new("Lossless", "AudioLossless", 21, "Aud", Color::Red),
        CatEntry::new("Lossy", "AudioLossy", 22, "Aud", Color::Yellow),
    ],
};

pub static LITERATURE: CatStruct = CatStruct {
    name: "Literature",
    entries: &[
        CatEntry::new("All Literature", "AllLiterature", 30, "Lit", Color::White),
        CatEntry::new(
            "English-Translated",
            "LitEnglishTranslated",
            31,
            "Lit",
            Color::LightGreen,
        ),
        CatEntry::new(
            "Non-English Translated",
            "LitEnglishTranslated",
            32,
            "Lit",
            Color::Yellow,
        ),
        CatEntry::new("Raw", "LitRaw", 33, "Lit", Color::Green),
    ],
};

pub static LIVE_ACTION: CatStruct = CatStruct {
    name: "Live Action",
    entries: &[
        CatEntry::new("All Live Action", "AllLiveAction", 40, "Liv", Color::White),
        CatEntry::new(
            "English-Translated",
            "LiveEnglishTranslated",
            41,
            "Liv",
            Color::Yellow,
        ),
        CatEntry::new(
            "Non-English Translated",
            "LiveNonEnglishTranslated",
            43,
            "Liv",
            Color::LightCyan,
        ),
        CatEntry::new(
            "Idol/Promo Video",
            "LiveIdolPromoVideo",
            42,
            "Liv",
            Color::LightYellow,
        ),
        CatEntry::new("Raw", "LiveRaw", 44, "Liv", Color::Gray),
    ],
};

pub static PICTURES: CatStruct = CatStruct {
    name: "Pictures",
    entries: &[
        CatEntry::new("All Pictures", "AllPictures", 50, "Pic", Color::White),
        CatEntry::new("Graphics", "PicGraphics", 51, "Pic", Color::LightMagenta),
        CatEntry::new("Photos", "PicPhotos", 52, "Pic", Color::Magenta),
    ],
};

pub static SOFTWARE: CatStruct = CatStruct {
    name: "Software",
    entries: &[
        CatEntry::new("All Software", "AllSoftware", 60, "Sof", Color::White),
        CatEntry::new("Applications", "SoftApplications", 61, "Sof", Color::Blue),
        CatEntry::new("Games", "SoftGames", 62, "Sof", Color::LightBlue),
    ],
};

pub static ALL_CATEGORIES: &'static [&CatStruct] = &[
    &ANIME,
    &AUDIO,
    &LITERATURE,
    &LIVE_ACTION,
    &PICTURES,
    &SOFTWARE,
];

pub struct CategoryPopup {
    pub category: usize,
    pub major: usize,
    pub minor: usize,
}

impl Default for CategoryPopup {
    fn default() -> Self {
        return CategoryPopup {
            category: 10,
            major: 0,
            minor: 0,
        };
    }
}

impl Widget for CategoryPopup {
    fn draw(&self, f: &mut Frame, app: &App, area: Rect) {
        if let Some(cat) = ALL_CATEGORIES.get(self.major) {
            let mut tbl: Vec<Row> = ALL_CATEGORIES
                .iter()
                .enumerate()
                .map(|(i, e)| match i == self.major {
                    false => Row::new(Text::raw(format!(" ▶ {}", e.name))),
                    true => Row::new(Text::raw(format!(" ▼ {}", e.name)))
                        .bg(app.theme.solid_bg)
                        .fg(app.theme.solid_fg),
                })
                .collect();
            let all = Row::new(vec![Line::from(vec![
                Span::raw(match self.category == 0 {
                    true => "  ",
                    false => "   ",
                }),
                Span::raw(" All Categories"),
            ])]);
            tbl.insert(
                0,
                match self.category == 0 {
                    true => all.bg(app.theme.hl_bg),
                    false => all,
                },
            );

            let cat_rows = cat.entries.iter().enumerate().map(|(i, e)| {
                let row = Row::new(vec![Line::from(vec![
                    Span::raw(match e.id == self.category {
                        true => "  ",
                        false => "   ",
                    }),
                    e.icon.label.fg(e.icon.color),
                    Span::raw(" "),
                    Span::raw(e.name),
                ])]);
                match i == self.minor {
                    true => row.bg(app.theme.hl_bg),
                    false => row,
                }
            });

            tbl.splice(self.major + 2..self.major + 2, cat_rows);

            let center = super::centered_rect(33, 13, area);
            let clear = super::centered_rect(center.width + 2, center.height, area);
            super::clear(f, clear, app.theme.bg);
            f.render_widget(
                Table::new(tbl, &[Constraint::Percentage(100)])
                    .block(create_block(app.theme, true).title("Category")),
                center,
            );
        }
    }

    fn handle_event(&mut self, app: &mut App, e: &Event) {
        if let Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) = e
        {
            match code {
                KeyCode::Enter => {
                    if let Some(cat) = ALL_CATEGORIES.get(self.major) {
                        if let Some(item) = cat.entries.get(self.minor) {
                            self.category = item.id;
                        }
                    }
                    app.mode = Mode::Loading;
                }
                KeyCode::Esc | KeyCode::Char('c') | KeyCode::Char('q') => {
                    app.mode = Mode::Normal;
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    if let Some(cat) = ALL_CATEGORIES.get(self.major) {
                        self.minor = match self.minor + 1 >= cat.entries.len() {
                            true => 0,
                            false => self.minor + 1,
                        };
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    if let Some(cat) = ALL_CATEGORIES.get(self.major) {
                        self.minor = match self.minor < 1 {
                            true => cat.entries.len() - 1,
                            false => self.minor - 1,
                        };
                    }
                }
                KeyCode::Char('G') => {
                    if let Some(cat) = ALL_CATEGORIES.get(self.major) {
                        self.minor = cat.entries.len() - 1;
                    }
                }
                KeyCode::Char('g') => {
                    self.minor = 0;
                }
                KeyCode::Tab | KeyCode::Char('J') => {
                    self.major = match self.major + 1 >= ALL_CATEGORIES.len() {
                        true => 0,
                        false => self.major + 1,
                    };
                    self.minor = 0;
                }
                KeyCode::BackTab | KeyCode::Char('K') => {
                    self.major = match self.major < 1 {
                        true => ALL_CATEGORIES.len() - 1,
                        false => self.major - 1,
                    };
                    self.minor = 0;
                }
                _ => {}
            }
        }
    }

    fn get_help() -> Option<Vec<(&'static str, &'static str)>> {
        Some(vec![
            ("Enter", "Confirm"),
            ("Esc, c, q", "Close"),
            ("j, ↓", "Down"),
            ("k, ↑", "Up"),
            ("g", "Top"),
            ("G", "Bottom"),
            ("Tab, J", "Next Tab"),
            ("S-Tab, K", "Prev Tab"),
        ])
    }
}
