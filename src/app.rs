use core::fmt::Write;
use heapless::String;

use wio_terminal::accelerometer::Accelerometer;
use wio_terminal::prelude::*;

use crate::drivers::board::Board;
use crate::drivers::controls::{ButtonId, Direction};
use crate::ui::accel;
use crate::ui::menu::{self, MENU_ITEMS};

#[derive(Clone, Copy, PartialEq)]
enum AppState {
    Menu,
    MenuConfirm, 
    Accel,
}

pub fn run(board: &mut Board) -> ! {
    menu::draw_splash(&mut board.display);
    while board.five_way.read().is_none() {
        board.delay.delay_ms(30u16);
    }
    while board.five_way.read().is_some() {
        board.delay.delay_ms(30u16);
    }

    let mut cursor: usize = 0;
    let mut confirm_cursor: usize = 0; 
    let mut state = AppState::Menu;
    let mut last_direction: Option<Direction> = None;
    let mut last_button: Option<ButtonId> = None;
    let mut buf: String<64> = String::new();
    let mut accel_offset = (0.0f32, 0.0f32, 0.0f32);

    menu::draw_menu(&mut board.display, cursor);

    loop {
        match state {
            AppState::Menu => {
                let direction = board.five_way.read();
                let is_new_press = direction.is_some() && last_direction.is_none();

                if is_new_press {
                    match direction {
                        Some(Direction::Up) => {
                            cursor = if cursor == 0 { MENU_ITEMS.len() - 1 } else { cursor - 1 };
                            menu::draw_menu(&mut board.display, cursor);
                        }
                        Some(Direction::Down) => {
                            cursor = (cursor + 1) % MENU_ITEMS.len();
                            menu::draw_menu(&mut board.display, cursor);
                        }
                        Some(Direction::Press) => {
                            state = AppState::MenuConfirm;
                            confirm_cursor = 0;
                            menu::draw_menu(&mut board.display, cursor);
                            menu::draw_confirm_dialog(&mut board.display, MENU_ITEMS[cursor], confirm_cursor);
                        }
                        _ => {}
                    }
                }
                last_direction = direction;
            }

            AppState::MenuConfirm => {
                let direction = board.five_way.read();
                let is_new_press = direction.is_some() && last_direction.is_none();

                if is_new_press {
                    match direction {
                        Some(Direction::Left) | Some(Direction::Up) => {
                            confirm_cursor = 0;
                            menu::draw_menu(&mut board.display, cursor);
                            menu::draw_confirm_dialog(&mut board.display, MENU_ITEMS[cursor], confirm_cursor);
                        }
                        Some(Direction::Right) | Some(Direction::Down) => {
                            confirm_cursor = 1;
                            menu::draw_menu(&mut board.display, cursor);
                            menu::draw_confirm_dialog(&mut board.display, MENU_ITEMS[cursor], confirm_cursor);
                        }
                        Some(Direction::Press) => {
                            if confirm_cursor == 0 {
                                if cursor == 0 {
                                    state = AppState::Accel;
                                    accel::draw_accel_screen(&mut board.display);
                                    if let Ok(a) = board.accelerometer.accel_norm() {
                                        accel_offset = (a.x, a.y, a.z);
                                    }
                                } else {
                                    state = AppState::Menu;
                                    menu::draw_menu(&mut board.display, cursor);
                                }
                            } else {
                                state = AppState::Menu;
                                menu::draw_menu(&mut board.display, cursor);
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
                        let rel_x = a.x - accel_offset.0;
                        let rel_y = a.y - accel_offset.1;
                        let rel_z = a.z - accel_offset.2;
                        let _ = write!(buf, "x:{:.2} y:{:.2} z:{:.2}", rel_x, rel_y, rel_z);
                    }
                    Err(_) => {
                        let _ = write!(buf, "read error");
                    }
                }
                accel::draw_accel_value(&mut board.display, &buf);

                let button = board.top_buttons.read();
                let is_new_button = button.is_some() && last_button.is_none();
                if is_new_button && button == Some(ButtonId::A) {
                    state = AppState::Menu;
                    menu::draw_menu(&mut board.display, cursor);
                }
                last_button = button;
            }
        }

        board.delay.delay_ms(30u16);
    }
}