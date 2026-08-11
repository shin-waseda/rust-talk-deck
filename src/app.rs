use core::fmt::Write;
use heapless::String;

use wio_terminal::accelerometer::Accelerometer;
use wio_terminal::prelude::*;

use crate::drivers::board::Board;
use crate::drivers::controls::{ButtonId, Direction};
use crate::ui::menu::{self, MENU_ITEMS};
use crate::ui::{escape, item, magic, status, talk};

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

#[derive(Clone, Copy, PartialEq)]
enum TalkState {
    WazaMenu,
    Miru,
    TonaeruResult,
    TonaeruConfirm,
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
    let mut talk_state = TalkState::WazaMenu;
    let mut waza_cursor: usize = 0;
    let mut result_confirm_cursor: usize = 0;
    let mut tonaeru_result = (0i32, 0i32, 0i32);

    menu::draw_menu(&mut board.display, cursor);

    loop {
        match state {
            AppState::Menu => {
                if let Some(direction) = board.five_way.is_pressed(&mut last_direction) {
                    match direction {
                        Direction::Up => {
                            cursor = if cursor == 0 { MENU_ITEMS.len() - 1 } else { cursor - 1 };
                            menu::draw_menu(&mut board.display, cursor);
                        }
                        Direction::Down => {
                            cursor = (cursor + 1) % MENU_ITEMS.len();
                            menu::draw_menu(&mut board.display, cursor);
                        }
                        Direction::Press => {
                            state = AppState::MenuConfirm;
                            confirm_cursor = 0;
                            menu::draw_menu(&mut board.display, cursor);
                            menu::draw_confirm_dialog(&mut board.display, MENU_ITEMS[cursor], confirm_cursor);
                        }
                        _ => {}
                    }
                }
                if let Some(button) = board.top_buttons.is_pressed(&mut last_button) {
                    if button == ButtonId::B {
                        cursor = (cursor + 1) % MENU_ITEMS.len();
                        menu::draw_menu(&mut board.display, cursor);
                    } else if button == ButtonId::A {
                        state = AppState::MenuConfirm;
                        confirm_cursor = 0;
                        menu::draw_menu(&mut board.display, cursor);
                        menu::draw_confirm_dialog(&mut board.display, MENU_ITEMS[cursor], confirm_cursor);
                    }
                }
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
                                        if let Ok(a) = board.accelerometer.accel_norm() {
                                            accel_offset = (a.x, a.y, a.z);
                                        }
                                        waza_cursor = 0;
                                        talk_state = TalkState::WazaMenu;
                                        // わざ画面全体を描画
                                        talk::draw_talk_screen(&mut board.display, waza_cursor, None, None);
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
                                        board.delay.delay_ms(2000u16);
                                        cortex_m::peripheral::SCB::sys_reset();
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

            // --- 各画面個別の処理 ---
            AppState::Talk => match talk_state {
                TalkState::WazaMenu => {
                    if board.top_buttons.is_pressed(&mut last_button) == Some(ButtonId::C) {
                        state = AppState::Menu;
                        menu::draw_menu(&mut board.display, cursor);
                    }

                    let direction = board.five_way.read();
                    let is_new_press = direction.is_some() && last_direction.is_none();
                    if is_new_press {
                        match direction {
                            Some(Direction::Left) | Some(Direction::Up) => {
                                waza_cursor = if waza_cursor == 0 { 3 } else { waza_cursor - 1 };
                                talk::draw_talk_screen(&mut board.display, waza_cursor, None, None);
                            }
                            Some(Direction::Right) | Some(Direction::Down) => {
                                waza_cursor = (waza_cursor + 1) % 4;
                                talk::draw_talk_screen(&mut board.display, waza_cursor, None, None);
                            }
                            Some(Direction::Press) => match waza_cursor {
                                0 => {
                                    // ぜろのはどう
                                    if let Ok(a) = board.accelerometer.accel_norm() {
                                        accel_offset = (a.x, a.y, a.z);
                                    }
                                    talk::draw_talk_screen(
                                        &mut board.display,
                                        waza_cursor,
                                        Some("きじゅんち こうしん！"),
                                        None,
                                    );
                                    board.delay.delay_ms(800u16);
                                    talk::draw_talk_screen(&mut board.display, waza_cursor, None, None);
                                }
                                1 => {
                                    // となえる
                                    talk::draw_measuring(&mut board.display);
                                    tonaeru_result = talk::sample_tonaeru(board, accel_offset);
                                    talk::draw_tonaeru_result(
                                        &mut board.display,
                                        tonaeru_result.0,
                                        tonaeru_result.1,
                                        tonaeru_result.2,
                                    );
                                    result_confirm_cursor = 0;
                                    talk::draw_result_confirm(&mut board.display, result_confirm_cursor);
                                    talk_state = TalkState::TonaeruConfirm;
                                }
                                2 => {
                                    // みる
                                    talk::draw_talk_screen(&mut board.display, waza_cursor, None, Some(2));
                                    talk_state = TalkState::Miru;
                                }
                                _ => {
                                    // にげる(わざメニューから)
                                    escape::draw(&mut board.display);
                                    board.delay.delay_ms(1500u16);
                                    state = AppState::Menu;
                                    menu::draw_menu(&mut board.display, cursor);
                                }
                            },
                            _ => {}
                        }
                    }
                    last_direction = direction;
                }

                TalkState::Miru => {
                    talk::update_accel_screen(board, accel_offset, &mut buf);

                    if board.top_buttons.is_pressed(&mut last_button) == Some(ButtonId::C) {
                        talk_state = TalkState::WazaMenu;
                        talk::draw_talk_screen(&mut board.display, waza_cursor, None, None);
                    }
                }

                TalkState::TonaeruResult => {
                    if board.top_buttons.is_pressed(&mut last_button) == Some(ButtonId::C) {
                        talk_state = TalkState::WazaMenu;
                        talk::draw_talk_screen(&mut board.display, waza_cursor, None, None);
                    }
                }

                TalkState::TonaeruConfirm => {
                    let direction = board.five_way.read();
                    let is_new_press = direction.is_some() && last_direction.is_none();
                    if is_new_press {
                        match direction {
                            Some(Direction::Left) | Some(Direction::Up) => {
                                result_confirm_cursor = 0;
                                talk::draw_result_confirm(&mut board.display, result_confirm_cursor);
                            }
                            Some(Direction::Right) | Some(Direction::Down) => {
                                result_confirm_cursor = 1;
                                talk::draw_result_confirm(&mut board.display, result_confirm_cursor);
                            }
                            Some(Direction::Press) => {
                                if result_confirm_cursor == 0 {
                                    // もどる
                                    talk_state = TalkState::WazaMenu;
                                    talk::draw_talk_screen(&mut board.display, waza_cursor, None, None);
                                } else {
                                    // とどまる
                                    talk::draw_tonaeru_result(
                                        &mut board.display,
                                        tonaeru_result.0,
                                        tonaeru_result.1,
                                        tonaeru_result.2,
                                    );
                                    talk_state = TalkState::TonaeruResult;
                                }
                            }
                            _ => {}
                        }
                    }
                    last_direction = direction;
                }
            },

            AppState::Status => {
                if board.top_buttons.is_pressed(&mut last_button) == Some(ButtonId::C) {
                    state = AppState::Menu;
                    menu::draw_menu(&mut board.display, cursor);
                }
            }

            AppState::Magic => {
                if board.top_buttons.is_pressed(&mut last_button) == Some(ButtonId::C) {
                    state = AppState::Menu;
                    menu::draw_menu(&mut board.display, cursor);
                }
            }

            AppState::Item => {
                if board.top_buttons.is_pressed(&mut last_button) == Some(ButtonId::C) {
                    state = AppState::Menu;
                    menu::draw_menu(&mut board.display, cursor);
                }
            }

            AppState::Escape => {
                if board.top_buttons.is_pressed(&mut last_button) == Some(ButtonId::C) {
                    state = AppState::Menu;
                    menu::draw_menu(&mut board.display, cursor);
                }
            }
        }

        board.delay.delay_ms(30u16);
    }
}