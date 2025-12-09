/* Library for the Wordle App */
use ratatui::style::palette::tailwind;
use reqwest::*;
use chrono::{Datelike};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Color {
    Green,
    Yellow,
    Gray,
    #[default] Blank
}

impl Into<ratatui::style::Color> for Color {
    fn into(self) -> ratatui::style::Color {
        match self {
            Color::Blank => tailwind::NEUTRAL.c700,
            Color::Green => tailwind::GREEN.c600,
            Color::Gray =>  tailwind::GRAY.c500,
            Color::Yellow => tailwind::YELLOW.c400
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct WordleBox {
    pub letter: Option<char>,
    pub color: Color
}

impl WordleBox {

    pub fn new(c: Option<char>, color: Color) -> WordleBox {
        WordleBox { letter: c, color }
    }

    pub fn to_string(wboxes : [WordleBox; 5]) -> String {

        let mut str = String::new();
        for w in wboxes {
            str.push(w.letter.expect("String empty found"));
        }

        str
    }
}


pub struct WordleGrid {
    pub grid: [[WordleBox; 5]; 6],
    first_free: (usize, usize)
}

impl Default for WordleGrid {
    fn default() -> WordleGrid {
        WordleGrid { grid: [[WordleBox::new(None, Color::Blank); 5]; 6], first_free: (0,0) }
    }
}

impl WordleGrid {

    pub fn append_char(&mut self, c : char) {
        if self.first_free.1 < 5 {
            self.grid[self.first_free.0][self.first_free.1] = WordleBox::new(Some(c), Color::Blank);
            self.first_free.1 += 1;
        }
    }

    pub fn remove_char(&mut self) {
        self.grid[self.first_free.0][self.first_free.1] = WordleBox::new(None, Color::Blank);
        self.first_free.1 = self.first_free.1.saturating_sub(1);
    }

    pub fn send_word(&mut self, solution : &str) {
        if self.first_free.0 < 6 {
            self.first_free.0 += 1;
        }

        let input = WordleBox::to_string(self.grid[self.first_free.0]);

        if check_word(&input, solution).iter().all(|&c| c.color == Color::Green) {
            println!("You won!");
        }

    }
}

pub async fn get_daily_word() -> std::result::Result<String, reqwest::Error> {

    let today_date = chrono::Utc::now();

    let fetch_word = get(format!("https://www.nytimes.com/svc/wordle/v2/{:04}-{:02}-{:02}.json",
            today_date.year(),
            today_date.month(),
            today_date.day())
        )
        .await?
        .text()
        .await?;


    // It's better to remove the expects and use if lets because
    // we don't want our function to crash and just return an error
    // if it happens, and then handle it in the main program
    let word = fetch_word.split(",")
        .find(|x| x.contains("solution"))
        .expect("Couldn't find solution")
        .split_once(":")
        .expect("Split failed")
        .1
        [1..6]
        .to_owned();

    Ok(word)
}


pub fn check_word(input : &str, word : &str) -> Vec<WordleBox>
{
    input.trim().chars().zip(word.chars())
        .map(|(i, w)| {
            if i == w {
                return WordleBox::new(Some(i), Color::Green)
            } else if word.contains(i) {
                return WordleBox::new(Some(i), Color::Yellow)
            } else {
                return WordleBox::new(Some(i), Color::Gray)
            }
        })
        .collect()
}
