use data::Centroid;
use palette::{rgb, IntoColor, LabHue, Lch};
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;

fn hex(rgb_color: rgb::Rgb<rgb::Linear>) -> String {
	format!(
		"#{:0width$X}{:0width$X}{:0width$X}",
		(rgb_color.red * 255.0) as u32,
		(rgb_color.green * 255.0) as u32,
		(rgb_color.blue * 255.0) as u32,
		width = 2
	)
}

fn export_file(contents: String, location: &str) {
	let mut writer = File::create(location).unwrap();
	let success = writer.write_all(&contents.into_bytes());
	if success.is_err() {
		eprintln!("Error writing: {}", location);
	}
}

fn print_file(contents: &str) {
	println!("{}", contents);
}

// Json

fn get_json(centroids: &Vec<Centroid>, full_path: &str) -> String {
	let mut output = Vec::new();
	writeln!(
		&mut output,
		"{{ \n\t\"wallpaper\":\t\"{}\",\n\t\"colors\":\t{{",
		full_path
	);
	for (counter, centroid) in centroids.iter().enumerate() {
		let _lch = &centroid.location;
		let lch: Lch = Lch::new(_lch.l, _lch.c, LabHue::from(_lch.h));
		let rgb_color: rgb::Rgb<rgb::Linear> = lch.into_rgb();
		if counter != 0 {
			writeln!(&mut output, ",");
		}
		write!(
			&mut output,
			"\t\t\"color{}\":\t\"{}\"",
			counter,
			hex(rgb_color)
		);
	}
	write!(&mut output, "\n\t}}\n}}");
	String::from_utf8(output).unwrap()
}

pub fn export_json(centroids: &Vec<Centroid>, full_path: &str, out_location: &str) {
	let output: String = get_json(centroids, full_path);
	export_file(output, out_location);
}

pub fn print_json(centroids: &Vec<Centroid>, full_path: &str) {
	let output: String = get_json(centroids, full_path);
	print_file(&output);
}

// Yaml

fn get_yaml(centroids: &Vec<Centroid>, full_path: &str) -> String {
	let mut output = Vec::new();
	writeln!(&mut output, "wallpaper: \"{}\"", full_path);
	writeln!(&mut output, "colors:");

	for (i, c) in centroids.iter().enumerate() {
		let _lch = &c.location;
		let lch: Lch = Lch::new(_lch.l, _lch.c, LabHue::from(_lch.h));
		let rgb_color: rgb::Rgb<rgb::Linear> = lch.into_rgb();
		writeln!(&mut output, "\tcolor{}: \"{}\"", i, hex(rgb_color));
	}
	String::from_utf8(output).unwrap()
}

pub fn export_yaml(centroids: &Vec<Centroid>, full_path: &str, out_location: &str) {
	let output: String = get_yaml(centroids, full_path);
	export_file(output, out_location);
}

pub fn print_yaml(centroids: &Vec<Centroid>, full_path: &str) {
	let output: String = get_yaml(centroids, full_path);
	print_file(&output);
}

// Shell

fn get_sh(centroids: &Vec<Centroid>, full_path: &str) -> String {
	let mut output = Vec::new();
	writeln!(&mut output, "wallpaper=\'{}\'", full_path);

	for (i, c) in centroids.iter().enumerate() {
		let _lch = &c.location;
		let lch: Lch = Lch::new(_lch.l, _lch.c, LabHue::from(_lch.h));
		let rgb_color: rgb::Rgb<rgb::Linear> = lch.into_rgb();
		writeln!(&mut output, "color{}=\'{}\'", i, hex(rgb_color));
	}
	String::from_utf8(output).unwrap()
}

pub fn export_sh(centroids: &Vec<Centroid>, full_path: &str, out_location: &str) {
	let output: String = get_sh(centroids, full_path);
	export_file(output, out_location);
}

pub fn print_sh(centroids: &Vec<Centroid>, full_path: &str) {
	let output: String = get_sh(centroids, full_path);
	print_file(&output);
}

// CSS

fn get_css(centroids: &Vec<Centroid>, full_path: &str) -> String {
	let mut output = Vec::new();
	writeln!(&mut output, ":root {{");
	writeln!(&mut output, "\t--wallpaper: url(\"{}\");", full_path);

	for (i, c) in centroids.iter().enumerate() {
		let _lch = &c.location;
		let lch: Lch = Lch::new(_lch.l, _lch.c, LabHue::from(_lch.h));
		let rgb_color: rgb::Rgb<rgb::Linear> = lch.into_rgb();
		writeln!(&mut output, "\t--color{}: {};", i, hex(rgb_color));
	}
	writeln!(&mut output, "}}");
	String::from_utf8(output).unwrap()
}

pub fn export_css(centroids: &Vec<Centroid>, full_path: &str, out_location: &str) {
	let output: String = get_css(centroids, full_path);
	export_file(output, out_location);
}

pub fn print_css(centroids: &Vec<Centroid>, full_path: &str) {
	let output: String = get_css(centroids, full_path);
	print_file(&output);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hex() {
		let lch: Lch = Lch::new(0.0, 0.0, LabHue::from(0.0));
		let rgb_color: rgb::Rgb<rgb::Linear> = lch.into_rgb();

		println!("hex = {:?}", rgb_color);
		assert_eq!("#000000", hex(rgb_color));
	}
}
