pub mod talk;
pub mod menu;
pub mod status;
pub mod magic;
pub mod item;
pub mod escape;
pub mod placeholder;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};

use u8g2_fonts::{fonts, FontRenderer};

/// 日本語フォント (u8g2 の Unifont 日本語セット)
pub const JP_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_unifont_t_japanese1>();

/// 画面全体を黒でクリアする
pub fn clear_screen<D: DrawTarget<Color = Rgb565>>(display: &mut D) {
    let _ = Rectangle::with_corners(Point::new(0, 0), Point::new(320, 240))
        .into_styled(PrimitiveStyleBuilder::new().fill_color(Rgb565::BLACK).build())
        .draw(display);
}