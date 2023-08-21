extern crate termion;
use marker;
use marker::Marker;
use std::io::{self, BufRead};

pub const TITLE: &str = "Tic Tac Toe";
pub const SELECT_A_SPACE: &str = ", select a space";
pub const WINNER: &str = " wins the game!";


pub fn display(output: &str) {
    println!("{}", output);
}

pub fn select_number() -> i32 {
    let input = get_input();
    match input.trim().parse::<i32>() {
        Ok(n) => n,
        Err(_e) => select_number(),
    }
}


fn get_input() -> String {
    let stdio = io::stdin();
    let input = stdio.lock();
    process_input(input)
}

fn process_input<R>(mut reader: R) -> String
where
    R: BufRead,
{
    let mut input = String::new();
    reader.read_line(&mut input).expect("Unable to read");
    input
}

pub fn select_space(player: &Marker) -> String {
    let mut select: String = marker::inspect(player);
    select += SELECT_A_SPACE;
    select
}

pub fn alert_winner(player: &Marker) -> String {
    let mut winner: String = marker::inspect(player);
    winner += WINNER;
    winner
}

pub fn clear_screen() {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
}
