use std::hash::Hash;

pub trait Style: Clone + Copy + PartialEq + Default + Hash + Eq {}
