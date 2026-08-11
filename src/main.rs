#![no_std]
#![no_main]

mod app;
mod drivers;
mod model;
mod ui;

use panic_halt as _;
use wio_terminal::entry;
use drivers::board::Board;

#[entry]
fn main() -> ! {
    let mut board = Board::init();
    app::run(&mut board)
}