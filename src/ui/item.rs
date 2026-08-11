use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb565;

use crate::ui::placeholder::draw_placeholder;

pub fn draw<D: DrawTarget<Color = Rgb565, Error = impl core::fmt::Debug>>(display: &mut D) {
    draw_placeholder(display, "どうぐ");
    // ここに「どうぐ」画面固有の描画を今後足していく
}