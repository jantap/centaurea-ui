mod buffer;
mod style;
mod coord;
mod error;
mod canvas;
mod pixels;
mod rect;
mod scrollable;
pub mod util;
mod drawable;
mod widget;

pub use error::BufUiError;
pub use scrollable::Scrollable;
pub use rect::Rect;
pub use pixels::Pixels;
pub use drawable::Drawable;
pub use coord::Coord;
pub use buffer::Buffer;
pub use style::Style;
pub use widget::Widget;
pub use canvas::Canvas;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Pixel<S: Style> {
	pub text: Option<char>,
	pub style: S,
}
