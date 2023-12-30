#[derive(Copy,Clone, Debug)]
pub struct Rgb {
	pub r: u8,
	pub g: u8,
	pub b: u8
}

impl Rgb {
	pub const RED : Rgb = Rgb::new(255,0,255);
	pub const GREEN : Rgb = Rgb::new(0,255,0);
	pub const BLUE : Rgb = Rgb::new(0,0,255);
	pub const BLACK : Rgb = Rgb::new(0,0, 0);
	pub const WHITE : Rgb = Rgb::new(255,255,255);
	pub const fn new(r:u8,g:u8, b:u8) -> Rgb {
		Rgb {r ,g , b}
	}
}