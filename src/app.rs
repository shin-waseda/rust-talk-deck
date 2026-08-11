use core::fmt::Write;
use heapless::String;

use wio_terminal::accelerometer::Accelerometer;
use wio_terminal::prelude::*;

use crate::drivers::board::Board;
use crate::drivers::controls::{ButtonId, Direction};
use crate::ui::{talk, status, magic, item, escape};
use crate::ui::menu::{self, MENU_ITEMS};

#[derive(Clone, Copy, PartialEq)]
enum AppState {
    Menu,
    MenuConfirm,
    Talk,   
    Status, 
    Magic,  
    Item,   
    Escape, 
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
                                match cursor {
                                    0 => {
                                        talk::draw_accel_screen(&mut board.display);
                                        if let Ok(a) = board.accelerometer.accel_norm() {
                                            accel_offset = (a.x, a.y, a.z);
                                        }
                                        state = AppState::Talk;
                                    }
                                    1 => {
                                        status::draw(&mut board.display);
                                        state = AppState::Status;
                                    }
                                    2 => {
                                        magic::draw(&mut board.display);
                                        state = AppState::Magic;
                                    }
                                    3 => {
                                        item::draw(&mut board.display);
                                        state = AppState::Item;
                                    }
                                    _ => {
                                        escape::draw(&mut board.display);
                                        state = AppState::Escape;
                                    }
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

            AppState::Talk => {
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
                talk::draw_accel_value(&mut board.display, &buf);

                let button = board.top_buttons.read();
                let is_new_button = button.is_some() && last_button.is_none();
                if is_new_button && button == Some(ButtonId::A) {
                    state = AppState::Menu;
                    menu::draw_menu(&mut board.display, cursor);
                }
                last_button = button;
            }

            // --- 各画面個別の処理 ---
            AppState::Status => {
                if tick_status(board, &mut last_button) {
                    state = AppState::Menu;
                    menu::draw_menu(&mut board.display, cursor);
                }
            }

            AppState::Magic => {
                if tick_magic(board, &mut last_button) {
                    state = AppState::Menu;
                    menu::draw_menu(&mut board.display, cursor);
                }
            }

            AppState::Item => {
                if tick_item(board, &mut last_button) {
                    state = AppState::Menu;
                    menu::draw_menu(&mut board.display, cursor);
                }
            }

            AppState::Escape => {
                if tick_escape(board, &mut last_button) {
                    state = AppState::Menu;
                    menu::draw_menu(&mut board.display, cursor);
                }
            }
        }

        board.delay.delay_ms(30u16);
    }
}

// ==========================================
// 各画面ごとの個別処理 (空箱関数)
// ==========================================

fn tick_status(board: &mut Board, last_button: &mut Option<ButtonId>) -> bool {
    // ここに「じょうたい」画面独自の入力・更新処理を書いていく
    let button = board.top_buttons.read();
    let is_new_button = button.is_some() && *last_button == None;
    *last_button = button;
    is_new_button && button == Some(ButtonId::A) // true でメニューに戻る
}

fn tick_magic(board: &mut Board, last_button: &mut Option<ButtonId>) -> bool {
    // ここに「まほう」画面独自の入力・更新処理を書いていく
    let button = board.top_buttons.read();
    let is_new_button = button.is_some() && *last_button == None;
    *last_button = button;
    is_new_button && button == Some(ButtonId::A)
}

fn tick_item(board: &mut Board, last_button: &mut Option<ButtonId>) -> bool {
    // ここに「どうぐ」画面独自の入力・更新処理を書いていく
    let button = board.top_buttons.read();
    let is_new_button = button.is_some() && *last_button == None;
    *last_button = button;
    is_new_button && button == Some(ButtonId::A)
}

fn tick_escape(board: &mut Board, last_button: &mut Option<ButtonId>) -> bool {
    // ここに「にげる」画面独自の入力・更新処理を書いていく
    let button = board.top_buttons.read();
    let is_new_button = button.is_some() && *last_button == None;
    *last_button = button;
    is_new_button && button == Some(ButtonId::A)
}