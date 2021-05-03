use super::{Buffer, Rect, Drawable, Pixel, Pixels, Coord, Style};

pub struct Canvas<'a, S: Style> {
    buffer: &'a mut Buffer<S>,
    pub frame: Rect,
}

impl<'a, S: Style> Canvas<'a, S> {
    pub fn new(buffer: &'a mut Buffer<S>, frame: Rect) -> Self {
        Self {
            buffer,
            frame,
        }
    }
}

impl<'a, S: Style> Drawable<S> for Canvas<'a, S> {
	fn fill(&mut self, text: char, style: S) {
        self.buffer.rect(self.frame, text, style);
    }

	fn rect(&mut self, rect: Rect, text: char, style: S) {
		let text: String = (0..rect.width).map(|_| text).collect();
		for y in 0..rect.height {
			self.buffer.string(self.frame.x + rect.x, self.frame.y + rect.y + y, text.clone(), style);
		}
	}

	fn string(&mut self, x: u16, y: u16, text: String, style: S) {
        let mut current_x = x;
        let mut current_y = y;

		for c in text.chars() {
			self.pixel(
                current_x,
                current_y,
				Pixel {
					text: Some(c),
					style,
				},
			);

            current_x += 1;
            if current_x >= self.frame.width {
                current_x = 0;
                current_y += 1;
            }
		}
	}

	fn pixels(&mut self, x: u16, y: u16, pixels: &Pixels<S>) {
        let mut current_x = x;
        let mut current_y = y;

		for p in pixels.iter(){
			self.pixel(
                current_x,
                current_y,
                *p
			);

            current_x += 1;
            if current_x >= self.frame.width {
                current_x = 0;
                current_y += 1;
            }
		}
	}

	fn pixel(&mut self, x: u16, y: u16, pixel: Pixel<S>) {
        self.buffer.pixel(self.frame.x + x, self.frame.y + y, pixel);
	}

	fn pixel_at_index(&mut self, index: usize, pixel: Pixel<S>) {
        let index = Coord::from_index(index, self.frame.width).as_index(self.buffer.width);
        self.buffer.pixel_at_index(index, pixel);
    }
}


