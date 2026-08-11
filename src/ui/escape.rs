use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;

use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};

use super::{clear_screen, JP_FONT};

pub fn draw<D: DrawTarget<Color = Rgb565, Error = impl core::fmt::Debug>>(display: &mut D) {
    clear_screen(display);
    let _ = JP_FONT.render_aligned(
        "うまく にげきれた！",
        Point::new(40, 110),
        VerticalPosition::Top,
        HorizontalAlignment::Left,
        FontColor::Transparent(Rgb565::WHITE),
        display,
    );
}