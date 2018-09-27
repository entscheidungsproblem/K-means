use clustering::cluster_all;
use clustering::init::kmeans_pp_init;
use clustering::distances::distance;
use palette::{rgb, IntoColor, LabHue, Lch};
use std::collections::VecDeque;
use std::f32;

use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

pub struct Pixel {
	pub l: f32,
	pub c: f32,
	pub h: f32,
	pub r: Option<u32>,
	pub g: Option<u32>,
	pub b: Option<u32>,
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
			self.r = Some((_rgb.red * 255.0) as u32);
			self.g = Some((_rgb.green * 255.0) as u32);
			self.b = Some((_rgb.blue * 255.0) as u32);
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

pub struct kmeans {
	pub pixels: VecDeque<Pixel>,
	pub centroids: Vec<Centroid>,
	dist_func: String,
}

impl kmeans {
	pub fn new(size: u32, pixels: VecDeque<Pixel> , dist_func: String) -> kmeans {
		let centroids = kmeans_pp_init(size, &pixels);
		kmeans {pixels, centroids, dist_func}
	}
	pub fn sort(&mut self, dark: bool) {
		let white = Pixel::new(100.0, 0.0, 270.0, 0_u32);
		let black = Pixel::new(0.0, 0.0, 0.0, 0_u32);
		let dist = &self.dist_func;
		if dark {
			self.centroids.sort_unstable_by_key(|k| distance(&white, &k.location, dist) as u32);
		} else {
			self.centroids.sort_unstable_by_key(|k| distance(&black, &k.location, dist) as u32);
		}
	}
	pub fn filter(&mut self, mut pixels: VecDeque<Pixel>) -> VecDeque<Pixel>{
		let delta = 0.1;
		let black = Pixel::new(0.0, 0.0, 0.0, 0_u32);
		let mut to_remove = Vec::new();
		for (index, pixel) in pixels.iter().enumerate(){
			if distance(&black, &pixel, &self.dist_func) < delta {
				to_remove.push(index);
			}
		}
		// Remove items with largest indices first
		to_remove.reverse();
		for remove in to_remove {
			pixels.remove(remove);
		}
		pixels
	}
	fn merge(&mut self, index1: u32, index2: u32) {
		
	}
	fn split(&mut self, index: u32) {
		
	}
	pub fn balance(&mut self) {

	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sort_dark() {
		let c1: Centroid = Centroid::new(100.0, 0.0, 0.0);	// White
		let c2: Centroid = Centroid::new(0.0, 0.0, 0.0);	// Black
		let c3: Centroid = Centroid::new(50.0, 0.0, 0.0);	// Grey
		let mut centroids = Vec::new();
		centroids.push(c3);
		centroids.push(c2);
		centroids.push(c1);
		let mut km: kmeans = kmeans{pixels:VecDeque::new(), centroids, dist_func:String::from("cie00")};
		km.sort(true);
		let mut output: String = String::new();;
		for mut c in km.centroids{
			writeln!(&mut output, "{}", c.location.print_hex()).unwrap();
		}
		assert_eq!(output, "FFFEFF\n2E2E2E\n000000\n");
	}

	#[test]
	fn sort_light() {
		let c1: Centroid = Centroid::new(100.0, 0.0, 0.0);	// White
		let c2: Centroid = Centroid::new(0.0, 0.0, 0.0);	// Black
		let c3: Centroid = Centroid::new(50.0, 0.0, 0.0);	// Grey
		let mut centroids = Vec::new();
		centroids.push(c3);
		centroids.push(c2);
		centroids.push(c1);
		let mut km: kmeans = kmeans{pixels:VecDeque::new(), centroids, dist_func:String::from("cie00")};
		km.sort(false);
		let mut output: String = String::new();;
		for mut c in km.centroids{
			writeln!(&mut output, "{}", c.location.print_hex()).unwrap();
		}
		assert_eq!(output, "000000\n2E2E2E\nFFFEFF\n");
	}
}
