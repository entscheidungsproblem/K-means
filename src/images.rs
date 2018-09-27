use data::{Centroid, Pixel};
use image::{open, ImageBuffer, ImageRgb8, Rgb, RgbImage, PNG};
use palette::{rgb, IntoColor, LabHue, Lch, Srgb};
use std::collections::{HashMap, VecDeque};
use std::f32;
use std::fs::File;
use std::process;

pub fn display(centroids: &Vec<Centroid>, name: &str) {
	let k = centroids.len();
	let imgx = k * 100;
	let imgy = 400;
	let mut centroid_rgb: VecDeque<(u8, u8, u8)> = VecDeque::with_capacity(centroids.len());
	for c in centroids {
		let _lch = &c.location;
		let lch: Lch = Lch::new(_lch.l, _lch.c, LabHue::from(_lch.h));
		let rgb: rgb::Rgb<rgb::Linear> = lch.into_rgb();
		centroid_rgb.insert(
			0,
			(
				(rgb.red * 255.0) as u8,
				(rgb.green * 255.0) as u8,
				(rgb.blue * 255.0) as u8,
			),
		);
	}
	// Create a new ImgBuf with width: imgx and height: imgy
	let mut imgbuf = ImageBuffer::new(imgx as u32, imgy as u32);
	// Iterate over the coordinates and pixels of the image
	for (x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
		let color_index = x / 100;
		let rgb = centroid_rgb[color_index as usize];
		*pixel = Rgb([rgb.0, rgb.1, rgb.2]);
	}
	let mut filename = String::from("data/");
	filename.push_str(&name);
	filename.push_str(".png");
	let fout = &mut File::create(filename).unwrap();
	// We must indicate the image's color type and what format to save as
	ImageRgb8(imgbuf).save(fout, PNG).unwrap()
}

/*
fn centroid(points: HashMap<Pixel, u32>, k: usize) -> HashMap<Pixel, u32> {
	let mut r: u32 = 0_u32;
	let mut g: u32 = 0_u32;
	let mut b: u32 = 0_u32;
	// let k: usize = points.len();
	for p in 0_u32..(k/3) as u32{
		let temp = points.get((3*p) as usize..(3*p+3) as usize).unwrap();
		println!("r: {}, t: {}", r, temp[0]);
		r += temp[0]/(k as u32);
		g += temp[1]/(k as u32);
		b += temp[2]/(k as u32);
	}
	return vec![r,g,b];
}
 */

pub fn load_image(name: String) -> VecDeque<Pixel> {
	let img1 = open(name).unwrap();
	let img2: Option<&RgbImage> = img1.as_rgb8();
	if img2.is_none() {
		process::exit(1);
	}
	let img3 = img2.unwrap();

	let num_pixels: usize = (img3.height() * img3.width()) as usize;

	let mut rgb_pixels: HashMap<(u8, u8, u8), u32> = HashMap::with_capacity(num_pixels);
	for _pixel in img3.chunks(3) {
		let _rgb = (_pixel[0], _pixel[1], _pixel[2]);
		rgb_pixels.entry(_rgb).or_insert(0);
		let count = rgb_pixels.get_mut(&_rgb).unwrap();
		*count += 1;
	}

	let mut pixels: VecDeque<Pixel> = VecDeque::with_capacity(num_pixels);
	for (_pixel, count) in &rgb_pixels {
		let lch_color: Lch = Srgb::new(
			f32::from(_pixel.0) / (256_f32),
			f32::from(_pixel.1) / (256_f32),
			f32::from(_pixel.2) / (256_f32),
		).into();
		let p = Pixel::new(
			lch_color.l,
			lch_color.chroma,
			lch_color.hue.to_degrees(),
			*count,
		);
		pixels.push_front(p);
	}
	pixels
}
