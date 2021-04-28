use std::{
	collections::{BTreeMap, HashMap},
	io::Write,
	iter::Iterator,
};

use crossterm::{
	cursor, queue,
	style::{self, ContentStyle},
};

use super::{Pixel, Rect, Drawable, Coord, Pixels, Style, Canvas};

pub struct Buffer<S: Style> {
	pub width: u16,
	pub height: u16,
	pixels: Vec<Pixel<S>>,
	changes: BTreeMap<usize, Pixel<S>>,
	pub styles: HashMap<S, ContentStyle>,
}

impl<S: Style> Default for Buffer<S> {
	fn default() -> Self {
		Self {
			width: 0,
			height: 0,
			pixels: Vec::new(),
			changes: BTreeMap::new(),
			styles: HashMap::new(),
		}
	}
}

impl<S: Style> Buffer<S> {
    pub fn canvas<'a>(&'a mut self, frame: Rect) -> Canvas<'a, S> {
        Canvas::new(self, frame)
    }

	pub fn set_styles(&mut self, styles: HashMap<S, ContentStyle>) {
		self.styles = styles;
	}

	pub fn resize(&mut self, width: u16, height: u16) {
		self.width = width;
		self.height = height;
		self.pixels = (0..width * height).map(|_| Pixel::default()).collect();
	}

	pub fn draw_changes<W: Write>(&mut self, stdout: &mut W) -> Result<(), crossterm::ErrorKind> {
		let mut last_style = None;
		let mut last_coord = None;
		let mut text = "".to_string();

		for (k, v) in self.changes.iter() {
			if let Some(pixel) = self.pixels.get(*k) {
				if *pixel != *v {
					self.pixels[*k] = *v;
				} else {
					continue;
				}
			} else {
				continue;
			}

			if last_style != Some(v.style) || *k == 0 || last_coord != Some(*k - 1) {
				if !text.is_empty() {
					let style = match self.styles.get(&last_style.unwrap()) {
						Some(s) => *s,
						None => ContentStyle::default(),
					};

					queue!(stdout, style::PrintStyledContent(style.apply(text)))?;
				}

				let coord = Coord::from_index(*k, self.width);
				queue!(stdout, cursor::MoveTo(coord.x, coord.y))?;

				text = v.text.unwrap_or(' ').to_string();
				last_style = Some(v.style);
			} else {
				text = format!("{}{}", text, v.text.unwrap_or(' '));
			}

			last_coord = Some(*k);
		}

		if !text.is_empty() {
			let style = match self.styles.get(&last_style.unwrap()) {
				Some(s) => *s,
				None => ContentStyle::default(),
			};

			queue!(stdout, style::PrintStyledContent(style.apply(text)))?;
		}

		self.changes.clear();

		stdout.flush()?;

		Ok(())
	}
}

impl<S: Style> Drawable<S> for Buffer<S> {
	fn fill(&mut self, text: char, style: S) {
        self.rect(Rect::new(0, 0, self.width, self.height), text, style);
    }
	fn rect(&mut self, rect: Rect, text: char, style: S) {
		let text: String = (0..rect.width).map(|_| text).collect();
		for y in 0..rect.height {
			self.string(rect.x, rect.y + y, text.clone(), style);
		}
	}

	fn string(&mut self, x: u16, y: u16, text: String, style: S) {
		let index = Coord::from((x, y)).as_index(self.width);

		for (i, c) in text.chars().enumerate() {
			if i + index >= self.pixels.len() {
				break;
			}

			self.pixel_at_index(
				index + i,
				Pixel {
					text: Some(c),
					style,
				},
			);
		}
	}

	fn pixels(&mut self, x: u16, y: u16, pixels: &Pixels<S>) {
		let index = Coord::from((x, y)).as_index(self.width);

		for (i, p) in pixels.iter().enumerate() {
			if i + index >= self.pixels.len() {
				break;
			}

			self.pixel_at_index(index + i, *p);
		}
	}

    fn pixel(&mut self, x: u16, y: u16, pixel: Pixel<S>) {
		let index = Coord::from((x, y)).as_index(self.width);
        self.pixel_at_index(index, pixel);
    }

	fn pixel_at_index(&mut self, coord: usize, pixel: Pixel<S>) {
		if self.pixels[coord] != pixel {
			self.changes.insert(coord, pixel);
		} else {
			self.changes.remove(&coord);
		}
	}
}
