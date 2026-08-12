use core::fmt::Write;
use heapless::String;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::{ascii::FONT_9X15, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle, StrokeAlignment};
use embedded_graphics::text::{Baseline, Text};

use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};
use wio_terminal::accelerometer::Accelerometer;
use wio_terminal::prelude::*;

use super::{clear_screen, JP_FONT};
use crate::drivers::board::Board;
use crate::model::WazaCommand;

pub fn draw_accel_value<D: DrawTarget<Color = Rgb565>>(display: &mut D, text: &str) {
    let clear_style = PrimitiveStyleBuilder::new().fill_color(Rgb565::BLACK).build();
    let text_style = MonoTextStyle::new(&FONT_9X15, Rgb565::WHITE);
    let _ = Rectangle::new(Point::new(20, 95), Size::new(280, 20))
        .into_styled(clear_style)
        .draw(display);
    let _ = Text::with_baseline(text, Point::new(20, 95), text_style, Baseline::Top)
        .draw(display);
}

pub fn update_accel_screen(
    board: &mut Board,
    accel_offset: (f32, f32, f32),
    buf: &mut String<64>,
) {
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
    draw_accel_value(&mut board.display, buf);
}

/// わざ画面全体(見出し + メッセージ + わざメニュー箱)を描画する
pub fn draw_talk_screen<D: DrawTarget<Color = Rgb565, Error = impl core::fmt::Debug>>(
    display: &mut D,
    waza_cursor: usize,
    message: Option<&str>,
    executing: Option<WazaCommand>, 
) {
    clear_screen(display);

    let heading_style = MonoTextStyle::new(&FONT_9X15, Rgb565::WHITE);
    let _ = Text::with_baseline("Accelerometer", Point::new(20, 60), heading_style, Baseline::Top)
        .draw(display);

    if let Some(msg) = message {
        let _ = JP_FONT.render_aligned(
            msg,
            Point::new(20, 95),
            VerticalPosition::Top,
            HorizontalAlignment::Left,
            FontColor::Transparent(Rgb565::YELLOW),
            display,
        );
    }

    draw_waza_box(display, waza_cursor, executing);
}

fn draw_waza_box<D: DrawTarget<Color = Rgb565, Error = impl core::fmt::Debug>>(
    display: &mut D,
    confirm_cursor: usize,
    executing: Option<WazaCommand>, 
) {
    let window_x = 10;
    let window_y = 130;
    let window_w = 130;
    let window_h = 100;

    let window_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::WHITE)
        .stroke_width(2)
        .fill_color(Rgb565::BLACK)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();

    let white_style = MonoTextStyle::new(&FONT_9X15, Rgb565::WHITE);
    let yellow_style = MonoTextStyle::new(&FONT_9X15, Rgb565::YELLOW);

    let _ = Rectangle::new(Point::new(window_x, window_y), Size::new(window_w, window_h))
        .into_styled(window_style)
        .draw(display);

    let item_name = "わざ";
    let _ = JP_FONT.render_aligned(
        item_name,
        Point::new(window_x + 15, window_y + 8),
        VerticalPosition::Top,
        HorizontalAlignment::Left,
        FontColor::Transparent(Rgb565::CYAN),
        display,
    );

    let item_start_y = window_y + 30;
    let line_height = 18;

    for (i, cmd) in WazaCommand::ALL.iter().enumerate() {
        let y = item_start_y + (i as i32 * line_height);
        let is_selected = i == confirm_cursor;
        let is_executing = executing == Some(*cmd);

        let label: &str = if is_executing { cmd.executing_label() } else { cmd.label() };

        let cursor_mark = if is_selected { ">" } else { " " };
        let text_style = if is_selected { yellow_style } else { white_style };
        let font_color = if is_executing {
            Rgb565::CYAN
        } else if is_selected {
            Rgb565::YELLOW
        } else {
            Rgb565::WHITE
        };

        let _ = Text::with_baseline(cursor_mark, Point::new(window_x + 8, y), text_style, Baseline::Top)
            .draw(display);
        let _ = JP_FONT.render_aligned(
            label,
            Point::new(window_x + 20, y),
            VerticalPosition::Top,
            HorizontalAlignment::Left,
            FontColor::Transparent(font_color),
            display,
        );
    }
}

pub fn draw_measuring<D: DrawTarget<Color = Rgb565, Error = impl core::fmt::Debug>>(
    display: &mut D,
) {
    clear_screen(display);
    let _ = JP_FONT.render_aligned(
        "けいそくちゅう...",
        Point::new(50, 110),
        VerticalPosition::Top,
        HorizontalAlignment::Left,
        FontColor::Transparent(Rgb565::WHITE),
        display,
    );
}

pub fn draw_tonaeru_result<D: DrawTarget<Color = Rgb565, Error = impl core::fmt::Debug>>(
    display: &mut D,
    x: i32,
    y: i32,
    z: i32,
) {
    clear_screen(display);
    let mut buf: String<64> = String::new();
    let _ = write!(buf, "x:{} y:{} z:{}", x, y, z);
    let text_style = MonoTextStyle::new(&FONT_9X15, Rgb565::WHITE);
    let _ = Text::with_baseline(&buf, Point::new(40, 100), text_style, Baseline::Top)
        .draw(display);
}

pub fn draw_result_confirm<D: DrawTarget<Color = Rgb565, Error = impl core::fmt::Debug>>(
    display: &mut D,
    confirm_cursor: usize,
) {
    let dialog_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::WHITE)
        .stroke_width(2)
        .fill_color(Rgb565::BLACK)
        .build();
    let _ = Rectangle::with_corners(Point::new(30, 150), Point::new(290, 210))
        .into_styled(dialog_style)
        .draw(display);

    let white_style = MonoTextStyle::new(&FONT_9X15, Rgb565::WHITE);
    let yellow_style = MonoTextStyle::new(&FONT_9X15, Rgb565::YELLOW);

    let is_back = confirm_cursor == 0;
    let back_mark = if is_back { ">" } else { " " };
    let _ = Text::with_baseline(
        back_mark, Point::new(45, 170),
        if is_back { yellow_style } else { white_style }, Baseline::Top,
    ).draw(display);
    let _ = JP_FONT.render_aligned(
        "もどる", Point::new(60, 170), VerticalPosition::Top, HorizontalAlignment::Left,
        FontColor::Transparent(if is_back { Rgb565::YELLOW } else { Rgb565::WHITE }), display,
    );

    let is_stay = confirm_cursor == 1;
    let stay_mark = if is_stay { ">" } else { " " };
    let _ = Text::with_baseline(
        stay_mark, Point::new(180, 170),
        if is_stay { yellow_style } else { white_style }, Baseline::Top,
    ).draw(display);
    let _ = JP_FONT.render_aligned(
        "とどまる", Point::new(195, 170), VerticalPosition::Top, HorizontalAlignment::Left,
        FontColor::Transparent(if is_stay { Rgb565::YELLOW } else { Rgb565::WHITE }), display,
    );
}

pub fn sample_tonaeru(board: &mut Board, accel_offset: (f32, f32, f32)) -> (i32, i32, i32) {
    let mut max_diff_x = 0.0f32;
    let mut max_diff_y = 0.0f32;
    let mut max_diff_z = 0.0f32;

    let mut max_jerk_x = 0.0f32;
    let mut max_jerk_y = 0.0f32;
    let mut max_jerk_z = 0.0f32;

    // 初期値を None にして、1回目のループで実測値を入れる（初期ズレ誤検知の防止）
    let mut prev_accel: Option<(f32, f32, f32)> = None;

    // 10ms × 150回 ＝ 1.5秒間サンプリング
    for _ in 0..150 {
        if let Ok(a) = board.accelerometer.accel_norm() {
            // ① 基準値（静止時）からの絶対ズレ量
            let diff_x = (a.x - accel_offset.0).abs();
            let diff_y = (a.y - accel_offset.1).abs();
            let diff_z = (a.z - accel_offset.2).abs();

            if diff_x > max_diff_x { max_diff_x = diff_x; }
            if diff_y > max_diff_y { max_diff_y = diff_y; }
            if diff_z > max_diff_z { max_diff_z = diff_z; }

            // ② 直前フレームからの変化量（Jerk）
            if let Some((px, py, pz)) = prev_accel {
                let jerk_x = (a.x - px).abs();
                let jerk_y = (a.y - py).abs();
                let jerk_z = (a.z - pz).abs();

                if jerk_x > max_jerk_x { max_jerk_x = jerk_x; }
                if jerk_y > max_jerk_y { max_jerk_y = jerk_y; }
                if jerk_z > max_jerk_z { max_jerk_z = jerk_z; }
            }

            prev_accel = Some((a.x, a.y, a.z));
        }
        board.delay.delay_ms(10u16);
    }

    // -------------------------------------------------------------
    // スケーリング計算
    // -------------------------------------------------------------
    let calc_value = |max_diff: f32, max_jerk: f32, axis_seed: u32| -> i32 {
        // ★ diff（振りの大きさ）を主軸にし、jerk（衝撃）の比率を下げる
        // これにより「手ブレ」で跳ね上がるのを防ぐ
        let swing_score = (max_diff * 1.5) + (max_jerk * 0.8);

        // デッドゾーン：持っているだけの手ブレ（スコア 0.35 未満）は 1桁（1〜9）
        if swing_score < 0.35 {
            let seed_hash = (axis_seed * 17 + (max_diff * 100.0) as u32) % 9 + 1;
            return seed_hash as i32;
        }

        // 正規化レンジ: 0.35 〜 6.0（持っているだけでは届かないが、振れば順調に伸びる範囲）
        let norm = ((swing_score - 0.35) / 5.65).clamp(0.0, 1.0);

        // 1.3乗カーブで「弱〜中振り」の差を出しやすくする
        let curve = libm::powf(norm, 1.3);

        // ベース値（100 〜 8800）
        let base_value = 100.0 + (curve * 8700.0);

        // ハッシュノイズ（乗算ノイズ 0.85 〜 1.15 の抑えめな揺らぎ）
        let bits = (swing_score * 100000.0) as u32 ^ (axis_seed * 0x9E3779B9);
        let hash = bits.wrapping_mul(0x85ebca6b) ^ (bits >> 13);
        let noise_factor = 0.85 + ((hash % 300) as f32 / 1000.0);

        let final_val = (base_value * noise_factor) as i32;

        final_val.clamp(1, 9999)
    };

    let x = calc_value(max_diff_x, max_jerk_x, 1);
    let y = calc_value(max_diff_y, max_jerk_y, 2);
    let z = calc_value(max_diff_z, max_jerk_z, 3);

    (x, y, z)
}