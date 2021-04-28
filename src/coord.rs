
pub struct Coord {
    pub x: u16,
    pub y: u16,
}

impl From<(u16, u16)> for Coord {
    fn from(xy: (u16, u16)) -> Self {
        Self { x: xy.0, y: xy.1 }
    }
}

impl Coord {
	pub fn from_index(index: usize, width: u16) -> Self {
		let y = (index as f32 / width as f32).floor() as usize;
		let x = index - (y * width as usize);
        Self { x: x as u16, y: y as u16 }
	}

	pub fn as_index(&self, width: u16) -> usize {
		(self.y * width + self.x) as usize
	}
}
