use super::{Rect, Pixels, Pixel, Style};

pub trait Drawable<S: Style> {
    fn fill(&mut self, text: char, style: S);

	fn rect(&mut self, rect: Rect, text: char, style: S);

	fn string(&mut self, x: u16, y: u16, text: String, style: S);

	fn pixels(&mut self, x: u16, y: u16, pixels: &Pixels<S>);

	fn pixel(&mut self, x: u16, y: u16, pixel: Pixel<S>);

	fn pixel_at_index(&mut self, index: usize, pixel: Pixel<S>);
}
