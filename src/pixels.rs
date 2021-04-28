use super::{Style, Pixel};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Pixels<S: Style>(Vec<Pixel<S>>);

impl<S: Style> Pixels<S> {
	pub fn next(mut self, style: S, s: char) -> Self {
		self.0.push(Pixel {
			style,
			text: Some(s),
		});
		self
	}
	pub fn string(mut self, style: S, s: &str) -> Self {
		for c in s.chars() {
			self.0.push(Pixel {
				style,
				text: Some(c),
			});
		}
		self
	}
	pub fn get_mut(&mut self, index: usize) -> Option<&mut Pixel<S>> {
		self.0.get_mut(index)
	}
	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Pixel<S>> + '_ {
		self.0.iter_mut()
	}
	pub fn iter(&self) -> impl Iterator<Item = &Pixel<S>> + '_ {
		self.0.iter()
	}
}

impl<S: Style> From<Vec<Pixel<S>>> for Pixels<S> {
	fn from(s: Vec<Pixel<S>>) -> Self {
		Self { 0: s }
	}
}
impl<S: Style> From<Pixels<S>> for Vec<Pixel<S>> {
	fn from(s: Pixels<S>) -> Self {
		s.0
	}
}
