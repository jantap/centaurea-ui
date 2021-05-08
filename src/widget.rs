use super::{Buffer, Rect, Style, BufUiError};

pub trait Widget<S: Style> {
	fn render(&mut self, buffer: &mut Buffer<S>) -> Result<(), BufUiError>;
	fn resize(&mut self, area: Rect) -> Result<(), BufUiError>;
    fn area(&self) -> Rect;
}
