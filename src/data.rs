use palette::{rgb, IntoColor, LabHue, Lch};
use std::collections::VecDeque;
use std::f32;

pub struct Pixel {
	pub l: f32,
	pub c: f32,
	pub h: f32,
	pub r: Option<u8>,
	pub g: Option<u8>,
	pub b: Option<u8>,
	pub count: u32,
}

impl Pixel {
	pub fn new(_l: f32, _c: f32, _h: f32, _count: u32) -> Pixel {
		Pixel {
			l: _l,
			c: _c,
			h: _h,
			r: None,
			g: None,
			b: None,
			count: _count,
		}
	}
	pub fn gen_rgb(&mut self) {
		if self.r.is_none() {
			let _lch: Lch = Lch::new(self.l, self.c, LabHue::from(self.h));
			let _rgb: rgb::Rgb<rgb::Linear> = _lch.into_rgb();
			//let _rgb: rgb::Rgb = rgb::Rgb::from(_lch).from_linear();
			self.r = Some((_rgb.red * 255.0) as u8);
			self.g = Some((_rgb.green * 255.0) as u8);
			self.b = Some((_rgb.blue * 255.0) as u8);
		}
	}
	pub fn print_hex(&mut self) -> String {
		self.gen_rgb();
		return format!(
			"{:0width$X}{:0width$X}{:0width$X}",
			self.r.unwrap(),
			self.g.unwrap(),
			self.b.unwrap(),
			width = 2
		);
	}
}

pub struct Centroid {
	pub location: Pixel,
	pub pixels: VecDeque<u32>,
	pub sum: (f32, f32, f32),
	pub count: u32,
}

impl Centroid {
	pub fn new(_l: f32, _c: f32, _h: f32) -> Centroid {
		Centroid {
			location: Pixel::new(_l, _c, _h, 0_u32),
			pixels: VecDeque::new(),
			sum: (0_f32, 0_f32, 0_f32),
			count: 0_u32,
		}
	}
	pub fn add(&mut self, index: u32, pixel: &Pixel) {
		self.pixels.push_front(index);
		self.sum = (
			self.sum.0 + pixel.l,
			self.sum.1 + pixel.c,
			self.sum.2 + pixel.h,
		);
		self.count += pixel.count;
	}
	pub fn update(&mut self, _l: f32, _c: f32, _h: f32) {
		self.location.l = _l;
		self.location.c = _c;
		self.location.h = _h;
		self.location.r = None;
		self.location.g = None;
		self.location.b = None;
	}
	pub fn next(&mut self) {
		let count = self.count as f32;
		let sum = (self.sum.0 / count, self.sum.1 / count, self.sum.2 / count);
		self.update(sum.0, sum.1, sum.2);
		self.sum = (0_f32, 0_f32, 0_f32);
		self.count = 0_u32;
	}
}

struct kmeans {
	pub pixels: VecDeque<Pixel>,
	pub centroids: VecDeque<Centroid>,
}
