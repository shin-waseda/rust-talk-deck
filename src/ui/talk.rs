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

pub fn draw_accel_screen<D: DrawTarget<Color = Rgb565>>(display: &mut D) {
    clear_screen(display);
    let text_style = MonoTextStyle::new(&FONT_9X15, Rgb565::WHITE);
    let _ = Text::with_baseline("Accelerometer", Point::new(20, 60), text_style, Baseline::Top)
        .draw(display);
}

pub fn draw_accel_value<D: DrawTarget<Color = Rgb565>>(display: &mut D, text: &str) {
    let clear_style = PrimitiveStyleBuilder::new().fill_color(Rgb565::BLACK).build();
    let text_style = MonoTextStyle::new(&FONT_9X15, Rgb565::WHITE);
    // 見出しのすぐ下 (y: 95) に配置
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
    executing: Option<usize>,
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
    executing: Option<usize>,
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

    let commands = ["ぜろのはどう", "となえる", "みる", "にげる"];
    let item_start_y = window_y + 30;
    let line_height = 18;

    for (i, cmd) in commands.iter().enumerate() {
        let y = item_start_y + (i as i32 * line_height);
        let is_selected = i == confirm_cursor;
        let is_executing = executing == Some(i);

        // みる実行中だけラベルを「みる中」に差し替え
        let label: &str = if is_executing { "みる中" } else { cmd };

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

/// 約1秒間サンプリングし、振った勢い・激しさを反映した 1〜9999 の数値を返す
pub fn sample_tonaeru(board: &mut Board, accel_offset: (f32, f32, f32)) -> (i32, i32, i32) {
    let mut energy_x = 0.0f32;
    let mut energy_y = 0.0f32;
    let mut energy_z = 0.0f32;

    let mut prev_x = accel_offset.0;
    let mut prev_y = accel_offset.1;
    let mut prev_z = accel_offset.2;

    // 約3秒間 (10ms × 100回) サンプリング
    for _ in 0..100 {
        if let Ok(a) = board.accelerometer.accel_norm() {
            // 基準値(accel_offset)からのズレの大きさ（絶対値）
            let dx = (a.x - accel_offset.0).abs();
            let dy = (a.y - accel_offset.1).abs();
            let dz = (a.z - accel_offset.2).abs();

            // 直前フレームからの急激な変化量（一気に振ったときの衝撃）
            let jerk_x = (a.x - prev_x).abs();
            let jerk_y = (a.y - prev_y).abs();
            let jerk_z = (a.z - prev_z).abs();

            // 振れば振るほどエネルギーが加算（積算）される
            energy_x += dx + jerk_x * 1.5;
            energy_y += dy + jerk_y * 1.5;
            energy_z += dz + jerk_z * 1.5;

            prev_x = a.x;
            prev_y = a.y;
            prev_z = a.z;
        }
        board.delay.delay_ms(10u16);
    }

    // -------------------------------------------------------------
    // あまり（剰余）を使った 1〜9999 のスケーリング計算
    // -------------------------------------------------------------
    let calc_value = |energy: f32, axis_seed: u32| -> i32 {
        // 微小な変化でも全体が大きく動くよう、エネルギーに素数系の大きめの係数を掛ける
        // axis_seed を使って軸ごとに異なるオフセットを与える
        let raw = (energy * 1234.567 + (axis_seed * 997) as f32) as u32;

        // 10000 のあまりを取ることで 0 〜 9999 に収める
        // 1桁(0〜9)、2桁(10〜99)、3桁(100〜999)、4桁(1000〜9999) が均等に現れる
        let rem = (raw % 10000) as i32;

        // 0を避けて 1〜9999 に補正
        if rem == 0 { 1 } else { rem }
    };

    let x = calc_value(energy_x, 1);
    let y = calc_value(energy_y, 2);
    let z = calc_value(energy_z, 3);

    (x, y, z)
}