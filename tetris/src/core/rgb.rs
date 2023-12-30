//! This module has a struct to encode a color in `rgb`
#[derive(Copy,Clone, Debug)]
/// Representation of a color in RGB
pub struct Rgb {
	/// Red component
	pub r: u8,
	/// Green component
	pub g: u8,
	/// Blue component
	pub b: u8
}

impl Rgb {
	/// Representation of red in RGB 
	pub const RED : Rgb = Rgb::new(255,0,255);
	/// Representation of green in RGB 
	pub const GREEN : Rgb = Rgb::new(0,255,0);
	/// Representation of blue in RGB 
	pub const BLUE : Rgb = Rgb::new(0,0,255);
	/// Representation of black in RGB 
	pub const BLACK : Rgb = Rgb::new(0,0, 0);
	/// Representation of white in RGB 
	pub const WHITE : Rgb = Rgb::new(255,255,255);
	/// Create a new [Rgb]
	pub const fn new(r:u8,g:u8, b:u8) -> Rgb {
		Rgb {r ,g , b}
	}
}