use heapless::String;

use wio_terminal::accelerometer::Accelerometer;
use wio_terminal::prelude::*;

use crate::drivers::board::Board;
use crate::drivers::controls::{ButtonId, Direction};
use crate::model::{Event, Model, Screen, TalkState};
use crate::ui::menu::{self, MENU_ITEMS};
use crate::ui::{escape, item, magic, status, talk};

pub fn run(board: &mut Board) -> ! {
    // スプラッシュ待機処理
    menu::draw_splash(&mut board.display);
    while board.five_way.read().is_none() {
        board.delay.delay_ms(30u16);
    }
    while board.five_way.read().is_some() {
        board.delay.delay_ms(30u16);
    }

    let mut model = Model::new();
    let mut last_direction: Option<Direction> = None;
    let mut last_button: Option<ButtonId> = None;
    let mut buf: String<64> = String::new();

    // 初回描画
    render(board, &model, &mut buf);

    loop {
        // 1. 入力をイベントに変換
        let event = poll_input(board, &mut last_direction, &mut last_button);

        // 2. モデル更新と副作用の実行
        if let Some(ev) = event {
            let prev_screen = model.screen; // ★ 更新前の画面を保持

            // わざメニューでの選択時の特殊な副作用（計測・待機処理）のハンドリング
            if let Screen::Talk(TalkState::WazaMenu) = prev_screen {
                if ev == Event::Select {
                    match model.waza_cursor {
                        0 => {
                            // ぜろのはどう：基準値更新
                            if let Ok(a) = board.accelerometer.accel_norm() {
                                model = model.update(Event::ZeroHadouExecuted((a.x, a.y, a.z)));
                                talk::draw_talk_screen(
                                    &mut board.display,
                                    model.waza_cursor,
                                    model.message,
                                    None,
                                );
                                board.delay.delay_ms(800u16);
                                model.message = None;
                            }
                        }
                        1 => {
                            // となえる：サンプリング実行
                            talk::draw_measuring(&mut board.display);
                            let res = talk::sample_tonaeru(board, model.accel_offset);
                            // サンプリング結果をイベントとしてモデルに渡す
                            model = model.update(Event::TonaeruCompleted(res));
                        }
                        3 => {
                            // にげる
                            model.screen = Screen::Escape;
                            escape::draw(&mut board.display);
                            board.delay.delay_ms(1500u16);
                            model.screen = Screen::Menu;
                        }
                        _ => {
                            // 上記以外（カーソル移動や「みる」など）は通常通りモデルを更新
                            model = model.update(ev);
                        }
                    }
                } else {
                    model = model.update(ev);
                }
            } else {
                // わざメニュー以外では通常通りモデルを更新
                model = model.update(ev);
            }

            render(board, &model, &mut buf);
        }

        // 3. リアルタイム更新（「みる」画面での加速度リアルタイム描画）
        if model.screen == Screen::Talk(TalkState::Miru) {
            talk::update_accel_screen(board, model.accel_offset, &mut buf);
        }

        board.delay.delay_ms(30u16);
    }
}

/// 入力デバイスの状態を読み取って Event を生成する関数
fn poll_input(
    board: &mut Board,
    last_direction: &mut Option<Direction>,
    last_button: &mut Option<ButtonId>,
) -> Option<Event> {
    if let Some(dir) = board.five_way.is_pressed(last_direction) {
        return match dir {
            Direction::Up => Some(Event::NavigateUp),
            Direction::Down => Some(Event::NavigateDown),
            Direction::Left => Some(Event::NavigateLeft),
            Direction::Right => Some(Event::NavigateRight),
            Direction::Press => Some(Event::Select),
        };
    }

    if let Some(btn) = board.top_buttons.is_pressed(last_button) {
        return match btn {
            ButtonId::A => Some(Event::Select),
            ButtonId::B => Some(Event::NavigateDown),
            ButtonId::C => Some(Event::Back),
        };
    }

    None
}

/// モデルの状態に応じた描画関数（View）
fn render(board: &mut Board, model: &Model, _buf: &mut String<64>) {
    match model.screen {
        Screen::Menu => {
            menu::draw_menu(&mut board.display, model.cursor);
        }
        Screen::MenuConfirm => {
            menu::draw_menu(&mut board.display, model.cursor);
            menu::draw_confirm_dialog(
                &mut board.display,
                MENU_ITEMS[model.cursor],
                model.confirm_cursor,
            );
        }
        Screen::Talk(TalkState::WazaMenu) => {
            talk::draw_talk_screen(
                &mut board.display,
                model.waza_cursor,
                model.message,
                None,
            );
        }
        Screen::Talk(TalkState::Miru) => {
            talk::draw_talk_screen(
                &mut board.display,
                model.waza_cursor,
                None,
                Some(2), // 「みる中」表示
            );
        }
        Screen::Talk(TalkState::TonaeruResult) => {
            talk::draw_tonaeru_result(
                &mut board.display,
                model.tonaeru_result.0,
                model.tonaeru_result.1,
                model.tonaeru_result.2,
            );
        }
        Screen::Talk(TalkState::TonaeruConfirm) => {
            talk::draw_tonaeru_result(
                &mut board.display,
                model.tonaeru_result.0,
                model.tonaeru_result.1,
                model.tonaeru_result.2,
            );
            talk::draw_result_confirm(&mut board.display, model.result_confirm_cursor);
        }
        Screen::Status => status::draw(&mut board.display),
        Screen::Magic => magic::draw(&mut board.display),
        Screen::Item => item::draw(&mut board.display),
        Screen::Escape => escape::draw(&mut board.display),
    }
}