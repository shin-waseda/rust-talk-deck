use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::{ascii::{FONT_9X15, FONT_10X20}, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::{Baseline, Text};

use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

use super::{clear_screen, JP_FONT};

pub const MENU_ITEMS: [&str; 5] = ["加速度センサー", "ステータス", "まほう", "どうぐ", "にげる"];

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

    let text_style = MonoTextStyle::new(&FONT_9X15, Rgb565::WHITE);
    for (i, item) in MENU_ITEMS.iter().enumerate() {
        let y = 50 + (i as i32) * 28;
        let cursor_mark = if i == cursor { ">" } else { " " };
        let _ = Text::with_baseline(cursor_mark, Point::new(55, y), text_style, Baseline::Top)
            .draw(display);

        let _ = JP_FONT.render_aligned(
            *item,
            Point::new(75, y),
            VerticalPosition::Top,
            HorizontalAlignment::Left,
            FontColor::Transparent(Rgb565::WHITE),
            display,
        );
    }
}

pub fn draw_selected<D: DrawTarget<Color = Rgb565, Error = impl core::fmt::Debug>>(
    display: &mut D,
    cursor: usize,
) {
    let text_style = MonoTextStyle::new(&FONT_9X15, Rgb565::YELLOW);
    let _ = Text::with_baseline("Selected:", Point::new(50, 210), text_style, Baseline::Top)
        .draw(display);

    let _ = JP_FONT.render_aligned(
        MENU_ITEMS[cursor],
        Point::new(150, 210),
        VerticalPosition::Top,
        HorizontalAlignment::Left,
        FontColor::Transparent(Rgb565::YELLOW),
        display,
    );
}