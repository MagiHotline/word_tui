use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame, crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect}, style::{Style, palette::tailwind},
    text::Text, widgets::{
        Block, Paragraph, StatefulWidget, Widget
    }
};
use wordle_tui::{WordleGrid, get_daily_word};

/// Struct for the main data for the App.
pub struct App {
    solution: String,
    has_won: bool,
    text_box: String,
    content: WordleGrid
}

impl Default for App {
    fn default() -> Self {
        Self {
            solution: String::new(),
            has_won: false,
            text_box: String::from("(Esc) quit"),
            content: WordleGrid::default()
        }
    }
}

/// Struct for the grid widget.
///     - cell_size: size of the single cell
///     - cols: size of the columns
///     - rows: size of the rows
pub struct Grid {
    cell_size: usize,
    cols: usize,
    rows: usize,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
                cell_size: 1,
                cols: 5,
                rows: 6
            }
        }
}

impl StatefulWidget for Grid {

    type State = WordleGrid;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State)
    {
        // Define the contraints for every cell
        let col_constraints = (0..self.cols).map(|_| Constraint::Length(self.cell_size as u16 + 4));
        let row_constraints = (0..self.rows).map(|_| Constraint::Length(self.cell_size as u16 + 2));

        let horizontal = Layout::horizontal(col_constraints).spacing(1);
        let vertical = Layout::vertical(row_constraints);

        let rows = vertical.split(area);
        for (row_index, &row_area) in rows.iter().enumerate() {
            for (col_index, &col_area) in horizontal.split(row_area).to_vec().iter().enumerate() {

                let current_cell = state.grid[row_index][col_index];

                Paragraph::new(Text::from(format!("{}", current_cell.letter
                        .unwrap_or(' ')))
                        .style(Style::new().fg(tailwind::WHITE)))
                    .block(Block::bordered())
                    .centered()
                    .style(Style::new().fg(current_cell.color.into()))
                    .render(col_area, buf);
            }
        }

    }
}

impl App {

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {

        self.solution = get_daily_word().await.expect("Couldn't get daily word");

        loop {
            terminal.draw(|frame| self.draw(frame))?;
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Enter =>
                            {
                                if WordleGrid::send_word(&mut self.content, &self.solution) && !self.has_won
                                {
                                    self.text_box = format!("You won!");
                                    self.has_won = true;
                                }
                            }
                        KeyCode::Backspace => if !self.has_won { WordleGrid::remove_char(&mut self.content)},
                        KeyCode::Esc => return Ok(()),
                        KeyCode::Char(c) => {
                            if !self.has_won {
                                if !c.is_ascii_alphabetic() { continue; }
                                self.content.append_char(c);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {

        let grid = Grid::default();

         let horizontal = &Layout::horizontal(
             [Constraint::Fill(1),
                          Constraint::Max((grid.cell_size as u16 + 5) * 5),
                          Constraint::Fill(1)]);

         let rects = horizontal.split(frame.area());

         let inner_layout = &Layout::vertical(
             [Constraint::Fill(1),
                         Constraint::Min((grid.cell_size as u16 + 2) * 6),
                         Constraint::Length(4),
                         Constraint::Fill(1)])
        .split(rects[1]);


         frame.render_stateful_widget(grid, inner_layout[1], &mut self.content);
         self.render_footer(frame, inner_layout[2]);
     }

     fn render_footer(&self, frame: &mut Frame, area: Rect) {
         let info_footer = Paragraph::new(self.text_box.to_owned())
             .wrap(ratatui::widgets::Wrap { trim: false })
             .style(
                 Style::new()
                     .fg(tailwind::WHITE)
             )
             .centered()
             .block(
                 Block::bordered()
                     .border_style(Style::new().fg(tailwind::WHITE)),
             );
         frame.render_widget(info_footer, area);
     }


}
