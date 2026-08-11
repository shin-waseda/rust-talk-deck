#![no_std]
#![no_main]

mod board;
mod controls;

use panic_halt as _;
use wio_terminal as wio;

use core::fmt::Write;
use heapless::String;

use wio::accelerometer::Accelerometer;
use wio::entry;
use wio::prelude::*;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::{ascii::{FONT_9X15, FONT_10X20}, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::{Baseline, Text};

use board::Board;
use controls::{ButtonId, Direction};

const MENU_ITEMS: [&str; 5] = ["accelerometer", "stats", "magic", "items", "escape"];

#[derive(Clone, Copy, PartialEq)]
enum AppState {
    Menu,
    Accel,
}

#[entry]
fn main() -> ! {
    let mut board = Board::init();

    draw_splash(&mut board.display);
    while board.five_way.read().is_none() {
        board.delay.delay_ms(30u16);
    }
    while board.five_way.read().is_some() {
        board.delay.delay_ms(30u16);
    }

    let mut cursor: usize = 0;
    let mut state = AppState::Menu;
    let mut last_direction: Option<Direction> = None;
    let mut last_button: Option<ButtonId> = None;
    let mut buf: String<64> = String::new();

    // ゼロ点調整（キャリブレーション）用の基準オフセット (x, y, z)
    let mut accel_offset = (0.0f32, 0.0f32, 0.0f32);

    draw_menu(&mut board.display, cursor);

    loop {
        match state {
            AppState::Menu => {
                let direction = board.five_way.read();
                let is_new_press = direction.is_some() && last_direction.is_none();

                if is_new_press {
                    match direction {
                        Some(Direction::Up) => {
                            cursor = if cursor == 0 { MENU_ITEMS.len() - 1 } else { cursor - 1 };
                            draw_menu(&mut board.display, cursor);
                        }
                        Some(Direction::Down) => {
                            cursor = (cursor + 1) % MENU_ITEMS.len();
                            draw_menu(&mut board.display, cursor);
                        }
                        Some(Direction::Press) => {
                            if cursor == 0 {
                                state = AppState::Accel;
                                draw_accel_screen(&mut board.display);

                                // 画面遷移時の姿勢を基準値（0点）としてキャリブレーション
                                if let Ok(a) = board.accelerometer.accel_norm() {
                                    accel_offset = (a.x, a.y, a.z);
                                }
                            } else {
                                draw_selected(&mut board.display, cursor);
                            }
                        }
                        _ => {}
                    }
                }
                last_direction = direction;
            }

            AppState::Accel => {
                buf.clear();
                match board.accelerometer.accel_norm() {
                    Ok(a) => {
                        // 基準値からの差分を計算
                        let rel_x = a.x - accel_offset.0;
                        let rel_y = a.y - accel_offset.1;
                        let rel_z = a.z - accel_offset.2;

                        let _ = write!(buf, "x:{:.2} y:{:.2} z:{:.2}", rel_x, rel_y, rel_z);
                    }
                    Err(_) => {
                        let _ = write!(buf, "read error");
                    }
                }
                draw_accel_value(&mut board.display, &buf);

                let button = board.top_buttons.read();
                let is_new_button = button.is_some() && last_button.is_none();
                if is_new_button && button == Some(ButtonId::A) {
                    state = AppState::Menu;
                    draw_menu(&mut board.display, cursor);
                }
                last_button = button;
            }
        }

        board.delay.delay_ms(30u16);
    }
}

fn clear_screen<D: DrawTarget<Color = Rgb565>>(display: &mut D) {
    let _ = Rectangle::with_corners(Point::new(0, 0), Point::new(320, 240))
        .into_styled(PrimitiveStyleBuilder::new().fill_color(Rgb565::BLACK).build())
        .draw(display);
}

fn draw_splash<D: DrawTarget<Color = Rgb565>>(display: &mut D) {
    clear_screen(display);
    let title_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    let _ = Text::with_baseline("Hello, World!", Point::new(60, 110), title_style, Baseline::Top).draw(display);
    let hint_style = MonoTextStyle::new(&FONT_9X15, Rgb565::WHITE);
    let _ = Text::with_baseline("press any button", Point::new(55, 150), hint_style, Baseline::Top).draw(display);
}

fn draw_menu<D: DrawTarget<Color = Rgb565>>(display: &mut D, cursor: usize) {
    clear_screen(display);
    let window_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::WHITE)
        .stroke_width(2)
        .fill_color(Rgb565::new(0, 0, 10))
        .build();
    let _ = Rectangle::with_corners(Point::new(40, 30), Point::new(280, 200))
        .into_styled(window_style)
        .draw(display);

    let text_style = MonoTextStyle::new(&FONT_9X15, Rgb565::WHITE);
    for (i, item) in MENU_ITEMS.iter().enumerate() {
        let y = 50 + (i as i32) * 28;
        let cursor_mark = if i == cursor { ">" } else { " " };
        let _ = Text::with_baseline(cursor_mark, Point::new(55, y), text_style, Baseline::Top).draw(display);
        let _ = Text::with_baseline(item, Point::new(75, y), text_style, Baseline::Top).draw(display);
    }
}

fn draw_selected<D: DrawTarget<Color = Rgb565>>(display: &mut D, cursor: usize) {
    let text_style = MonoTextStyle::new(&FONT_9X15, Rgb565::YELLOW);
    let _ = Text::with_baseline("Selected:", Point::new(50, 210), text_style, Baseline::Top).draw(display);
    let _ = Text::with_baseline(MENU_ITEMS[cursor], Point::new(150, 210), text_style, Baseline::Top).draw(display);
}

fn draw_accel_screen<D: DrawTarget<Color = Rgb565>>(display: &mut D) {
    clear_screen(display);
    let text_style = MonoTextStyle::new(&FONT_9X15, Rgb565::WHITE);
    let _ = Text::with_baseline("Accelerometer (A: back)", Point::new(20, 60), text_style, Baseline::Top).draw(display);
}

fn draw_accel_value<D: DrawTarget<Color = Rgb565>>(display: &mut D, text: &str) {
    let clear_style = PrimitiveStyleBuilder::new().fill_color(Rgb565::BLACK).build();
    let text_style = MonoTextStyle::new(&FONT_9X15, Rgb565::WHITE);
    let _ = Rectangle::new(Point::new(20, 110), Size::new(280, 20))
        .into_styled(clear_style)
        .draw(display);
    let _ = Text::with_baseline(text, Point::new(20, 110), text_style, Baseline::Top).draw(display);
}