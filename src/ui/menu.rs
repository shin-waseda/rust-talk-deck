use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::{ascii::{FONT_9X15, FONT_10X20}, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::{Baseline, Text};

use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

use super::{clear_screen, JP_FONT};

pub const MENU_ITEMS: [&str; 5] = ["はなす", "じょうたい", "まほう", "どうぐ", "にげる"];

pub fn draw_splash<D: DrawTarget<Color = Rgb565>>(display: &mut D) {
    clear_screen(display);
    let title_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    let _ = Text::with_baseline("Hello, World!", Point::new(60, 110), title_style, Baseline::Top)
        .draw(display);
    let hint_style = MonoTextStyle::new(&FONT_9X15, Rgb565::WHITE);
    let _ = Text::with_baseline("press any button", Point::new(55, 150), hint_style, Baseline::Top)
        .draw(display);
}

pub fn draw_menu<D: DrawTarget<Color = Rgb565, Error = impl core::fmt::Debug>>(
    display: &mut D,
    cursor: usize,
) {
    clear_screen(display);
    let window_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::WHITE)
        .stroke_width(2)
        .fill_color(Rgb565::new(0, 0, 10))
        .build();
    let _ = Rectangle::with_corners(Point::new(40, 30), Point::new(280, 200))
        .into_styled(window_style)
        .draw(display);

    let white_style = MonoTextStyle::new(&FONT_9X15, Rgb565::WHITE);
    let yellow_style = MonoTextStyle::new(&FONT_9X15, Rgb565::YELLOW);
    
    for (i, item) in MENU_ITEMS.iter().enumerate() {
        let y = 50 + (i as i32) * 28;
        let is_selected = i == cursor;

        let cursor_mark = if is_selected { ">" } else { " " };
        let text_style = if is_selected { yellow_style } else { white_style };
        let font_color = if is_selected { Rgb565::YELLOW } else { Rgb565::WHITE };

        let _ = Text::with_baseline(cursor_mark, Point::new(55, y), text_style, Baseline::Top)
            .draw(display);

        let _ = JP_FONT.render_aligned(
            *item,
            Point::new(75, y),
            VerticalPosition::Top,
            HorizontalAlignment::Left,
            FontColor::Transparent(font_color),
            display,
        );
    }
}

pub fn draw_confirm_dialog<D: DrawTarget<Color = Rgb565, Error = impl core::fmt::Debug>>(
    display: &mut D,
    item_name: &str,
    confirm_cursor: usize, 
) {
    let dialog_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::WHITE)
        .stroke_width(2)
        .fill_color(Rgb565::BLACK)
        .build();
    let _ = Rectangle::with_corners(Point::new(30, 150), Point::new(290, 225))
        .into_styled(dialog_style)
        .draw(display);

    // テキストスタイル（白と黄色）
    let white_style = MonoTextStyle::new(&FONT_9X15, Rgb565::WHITE);
    let yellow_style = MonoTextStyle::new(&FONT_9X15, Rgb565::YELLOW);

    // 1. 選択項目名（水色）
    let _ = JP_FONT.render_aligned(
        item_name,
        Point::new(45, 160),
        VerticalPosition::Top,
        HorizontalAlignment::Left,
        FontColor::Transparent(Rgb565::CYAN),
        display,
    );

    // 「にしますか？」
    let _ = JP_FONT.render_aligned(
        "にしますか？",
        Point::new(140, 160), 
        VerticalPosition::Top,
        HorizontalAlignment::Left,
        FontColor::Transparent(Rgb565::WHITE),
        display,
    );

    // 2. 「はい」の描画（選択中なら黄色）
    let is_yes = confirm_cursor == 0;
    let yes_cursor_mark = if is_yes { ">" } else { " " };
    let yes_style = if is_yes { yellow_style } else { white_style };
    let yes_color = if is_yes { Rgb565::YELLOW } else { Rgb565::WHITE };

    let _ = Text::with_baseline(yes_cursor_mark, Point::new(80, 195), yes_style, Baseline::Top)
        .draw(display);
    let _ = JP_FONT.render_aligned(
        "はい",
        Point::new(95, 195),
        VerticalPosition::Top,
        HorizontalAlignment::Left,
        FontColor::Transparent(yes_color),
        display,
    );

    // 3. 「いいえ」の描画（選択中なら黄色）
    let is_no = confirm_cursor == 1;
    let no_cursor_mark = if is_no { ">" } else { " " };
    let no_style = if is_no { yellow_style } else { white_style };
    let no_color = if is_no { Rgb565::YELLOW } else { Rgb565::WHITE };

    let _ = Text::with_baseline(no_cursor_mark, Point::new(180, 195), no_style, Baseline::Top)
        .draw(display);
    let _ = JP_FONT.render_aligned(
        "いいえ",
        Point::new(195, 195),
        VerticalPosition::Top,
        HorizontalAlignment::Left,
        FontColor::Transparent(no_color),
        display,
    );
}