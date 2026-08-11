use core::fmt::Write;
use heapless::String;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::{ascii::FONT_9X15, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::{Baseline, Text};

use wio_terminal::accelerometer::Accelerometer;

use crate::drivers::board::Board;
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

pub fn update_talk_screen(
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