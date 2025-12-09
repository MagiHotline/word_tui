use wordle_tui::{check_word, get_daily_word, Color};
use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

mod app;

use app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::default().run(terminal);
    ratatui::restore();
    result
}


fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

/*
#[tokio::main]
async fn main() {

    let word = get_daily_word().await.expect("Couldn't get daily word");

    let mut tries = 6;
    let mut input : String = String::new();

    while tries != 0
    {
        input.clear();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        println!("{}", tries);

        if input.trim().len() != 5 { continue; }
        tries -= 1;

        let colors = check_word(&input, &word);

        println!("{:?}", colors);

        if colors.iter().all(|&c| c == Color::Green) {
            println!("You won!");
            break;
        }
    }

    println!("The word was: {}", word);
}
*/
