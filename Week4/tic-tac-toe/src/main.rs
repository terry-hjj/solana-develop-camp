mod app_runner;
mod io;
mod board_printer;
mod board;
mod game;
mod game_types;
mod marker;
mod players;
mod computer;
mod human;
mod board_formatter;
mod lines;

fn main() {
    app_runner::start();
}
