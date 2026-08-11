use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::{ascii::FONT_9X15, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::{Baseline, Text};

use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

use super::{clear_screen, JP_FONT};

pub fn draw_placeholder<D: DrawTarget<Color = Rgb565, Error = impl core::fmt::Debug>>(
    display: &mut D,
    title: &str,
) {
    clear_screen(display);

    let box_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::WHITE)
        .stroke_width(2)
        .build();
    let _ = Rectangle::with_corners(Point::new(40, 30), Point::new(280, 200))
        .into_styled(box_style)
        .draw(display);

    let _ = JP_FONT.render_aligned(
        title,
        Point::new(60, 90),
        VerticalPosition::Top,
        HorizontalAlignment::Left,
        FontColor::Transparent(Rgb565::WHITE),
        display,
    );

    let hint_style = MonoTextStyle::new(&FONT_9X15, Rgb565::WHITE);
    let _ = Text::with_baseline("(A: back)", Point::new(20, 210), hint_style, Baseline::Top)
        .draw(display);
}