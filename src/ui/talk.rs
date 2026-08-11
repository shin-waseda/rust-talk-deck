use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::{ascii::FONT_9X15, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::{Baseline, Text};

use super::clear_screen;

pub fn draw_accel_screen<D: DrawTarget<Color = Rgb565>>(display: &mut D) {
    clear_screen(display);
    let text_style = MonoTextStyle::new(&FONT_9X15, Rgb565::WHITE);
    let _ = Text::with_baseline("Accelerometer (A: back)", Point::new(20, 60), text_style, Baseline::Top)
        .draw(display);
}

pub fn draw_accel_value<D: DrawTarget<Color = Rgb565>>(display: &mut D, text: &str) {
    let clear_style = PrimitiveStyleBuilder::new().fill_color(Rgb565::BLACK).build();
    let text_style = MonoTextStyle::new(&FONT_9X15, Rgb565::WHITE);
    let _ = Rectangle::new(Point::new(20, 110), Size::new(280, 20))
        .into_styled(clear_style)
        .draw(display);
    let _ = Text::with_baseline(text, Point::new(20, 110), text_style, Baseline::Top)
        .draw(display);
}