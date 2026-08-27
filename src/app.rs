use heapless::String;

use wio_terminal::accelerometer::Accelerometer;
use wio_terminal::prelude::*;

use crate::drivers::board::Board;
use crate::drivers::controls::{ButtonId, Direction};
use crate::model::{Effect, Event, Model, Screen, TalkState, WazaCommand};
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
            // イベントを発行して「更新後のモデル」と「実行すべき副作用(Effect)」を取得
            let res = model.update(ev);
            model = res.model;

            // 副作用が存在する場合の処理ハンドリング
            if let Some(effect) = res.effect {
                match effect {
                    Effect::ExecuteZeroHadou => {
                        if let Ok(a) = board.accelerometer.accel_norm() {
                            let res = model.update(Event::ZeroHadouExecuted((a.x, a.y, a.z)));
                            model = res.model;

                            render(board, &model, &mut buf);
                            board.delay.delay_ms(800u16);

                            let res = model.update(Event::ClearMessage); // ★直接代入をやめてupdate経由に
                            model = res.model;
                            render(board, &model, &mut buf);
                        }
                    }
                    Effect::ExecuteTonaeru => {
                        talk::draw_measuring(&mut board.display);
                        let sample_res = talk::sample_tonaeru(board, model.accel_offset);

                        // word.idx には 10000 語 + 1 の番兵(ファイル末尾)が入っている。
                        // 実際に使う単語の有効インデックスは 0..9999 である。
                        const WORD_COUNT: u32 = 10_000;
                        let mut words: [String<64>; 3] = [String::new(), String::new(), String::new()];
                        let indices = [
                            (sample_res.0.max(0) as u32) % WORD_COUNT,
                            (sample_res.1.max(0) as u32) % WORD_COUNT,
                            (sample_res.2.max(0) as u32) % WORD_COUNT,
                        ];

                        if talk::lookup_words(board, indices, &mut words).is_err() {
                            let fallback = ["ビットフィールド", "重ね探索", "スライドパッド"];
                            for (item, word) in words.iter_mut().zip(fallback.iter()) {
                                item.clear();
                                let _ = item.push_str(word);
                            }
                        }

                        let res = model.update(Event::TonaeruCompleted { result: sample_res, words });
                        model = res.model;
                    }
                    Effect::ExecuteEscape => {
                        render(board, &model, &mut buf); 
                        board.delay.delay_ms(1500u16);
                        let res = model.update(Event::Back);
                        model = res.model;
                    }
                    Effect::SystemReset => {
                        // 【メインメニューの「にげる」】
                        // 画面を表示して、ハードリセットをかける
                        render(board, &model, &mut buf);
                        board.delay.delay_ms(1500u16);
                        
                        // システム再起動
                        cortex_m::peripheral::SCB::sys_reset();
                    }
                }
            }

            // 最終状態の描画
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
                Some(WazaCommand::Miru),
            );
        }
        Screen::Talk(TalkState::TonaeruResult) => {
            talk::draw_tonaeru_result(&mut board.display, model.tonaeru_result, &model.tonaeru_words);
        }
        Screen::Talk(TalkState::TonaeruConfirm) => {
            talk::draw_tonaeru_result(&mut board.display, model.tonaeru_result, &model.tonaeru_words);
            talk::draw_result_confirm(&mut board.display, model.result_confirm_cursor);
        }
        Screen::Status => status::draw(&mut board.display),
        Screen::Magic => magic::draw(&mut board.display),
        Screen::Item => item::draw(&mut board.display),
        Screen::Escape => escape::draw(&mut board.display),
    }
}