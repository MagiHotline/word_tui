use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame, crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect}, style::{Color, Style, Stylize, palette::tailwind},
    text::Text, widgets::{
        Block, BorderType, Cell, Paragraph, Row, Table, TableState,
    }
};
use unicode_width::UnicodeWidthChar;
use wordle_tui::WordleBox;

const INFO_TEXT: &str = "(Esc) quit | (←) move left | (→) move right";

const ITEM_HEIGHT: usize = 6;

struct TableColors {
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            // Let's insert the REAL colors we want to implement
            // One for BLANK, for GREEN, for YELLOW and GRAY of the wordle_tui Color enumerator
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}


pub struct App {
    state: TableState,
    cell_size: usize,
    input: [[WordleBox; 5]; 6]
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: TableState::default().with_selected(0),
            input: [[WordleBox::new('a', wordle_tui::Color::Blank); 5]; 6],
            cell_size: 5,
        }
    }
}

impl App {

    pub fn next_column(&mut self) {
        self.state.select_next_column();
    }

    pub fn previous_column(&mut self) {
        self.state.select_previous_column();
    }


    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('l') | KeyCode::Right => self.next_column(),
                        KeyCode::Char('h') | KeyCode::Left => self.previous_column(),
                        _ => {}
                    }
                }
            }
        }
    }


    fn draw(&mut self, frame: &mut Frame) {
         let horizontal = &Layout::horizontal([Constraint::Percentage(33),
                                                                   Constraint::Percentage(33),
                                                                   Constraint::Percentage(33)])
                                         ;
         let rects = horizontal.split(frame.area());

         let inner_layout = &Layout::vertical([Constraint::Percentage(20),
                                                                    Constraint::Percentage(60),
                                                                    Constraint::Length(3),
                                                                    Constraint::Percentage(20)])
        .split(rects[1]);

         self.render_table(frame, inner_layout[1]);
         self.render_footer(frame, inner_layout[2]);
     }


     fn render_table(&mut self, frame: &mut Frame, area: Rect) {

        let rows = self.input.map(|wordlebox|  {
                wordlebox.map(|w| {
                    return Cell::from(Text::from(format!("{}", w.letter)))
                        .style(Style::new().bg(w.color.into()).fg(tailwind::WHITE));
                }).into_iter().collect::<Row>().height(area.height / 6)
        });

        let t = Table::new(
            rows,
            [
                Constraint::Length(area.width / 5); 5
            ]
        )
        .bg(tailwind::BLACK);

        frame.render_stateful_widget(t, area, &mut self.state);
     }

     fn render_footer(&self, frame: &mut Frame, area: Rect) {
         let info_footer = Paragraph::new(Text::from(INFO_TEXT))
             .style(
                 Style::new()
                     .fg(tailwind::WHITE)
                     .bg(tailwind::BLACK),
             )
             .centered()
             .block(
                 Block::bordered()
                     .border_type(BorderType::Double)
                     .border_style(Style::new().fg(tailwind::WHITE)),
             );
         frame.render_widget(info_footer, area);
     }


}
