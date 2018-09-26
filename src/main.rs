#[macro_use]
extern crate clap;
extern crate image;
extern crate palette;
extern crate rand;
extern crate rayon;

pub mod clustering;
pub mod data;
pub mod export;
pub mod images;

use clap::App;
use clustering::cluster_all;
use clustering::init::kmeans_pp_init;
use data::{Centroid, Pixel};
use export::{
	export_css, export_json, export_sh, export_yaml, print_css, print_json, print_sh, print_yaml,
};
use images::load_image;

use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

fn check_path(path: &str) -> Option<String> {
	let _srcdir = fs::canonicalize(&PathBuf::from(path));
	if _srcdir.is_err() {
		eprintln!("Path error!");
		return None;
	}
	let srcdir = _srcdir.unwrap();
	if !(srcdir.exists()) {
		eprintln!("Path doesn't exist!");
		return None;
	}
	Some(String::from(srcdir.to_str().unwrap()))
}

fn main() {
	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml).get_matches();
	let check: Option<String> = check_path(matches.value_of("INPUT").unwrap());
	if check.is_none() {
		return;
	}
	let full_path: String = check.unwrap();
	let pixels: VecDeque<Pixel> = load_image(full_path.clone());
	let mut centroids: VecDeque<Centroid> = kmeans_pp_init(7_u8, &pixels);
	//let white = Centroid {p:Pixel{base_colors:(100.0, 0.0, 270.0)}, sum:(0.0, 0.0, 0.0), count:0_u32};
	//let black = Centroid {p:Pixel{base_colors:(0.0, 0.0, 0.0)}, sum:(0.0, 0.0, 0.0), count:0_u32};
	//centroids.insert(0, white);
	//centroids.insert(0, black);
	cluster_all(&pixels, &mut centroids, 50, 0.01);

	if let Some(print_matches) = matches.subcommand_matches("print") {
		if print_matches.is_present("json") {
			print_json(&centroids, &full_path);
		} else if print_matches.is_present("yaml") {
			print_yaml(&centroids, &full_path);
		} else if print_matches.is_present("sh") {
			print_sh(&centroids, &full_path);
		} else if print_matches.is_present("css") {
			print_css(&centroids, &full_path);
		} else {
			println!("print needs an argument: (j)son, (y)aml, (c)ss, (s)h");
		}
	} else {
		if matches.is_present("json") {
			export_json(&centroids, &full_path, "colors.json");
		}
		if matches.is_present("yaml") {
			export_yaml(&centroids, &full_path, "colors.yml");
		}
		if matches.is_present("sh") {
			export_sh(&centroids, &full_path, "colors.sh");
		}
		if matches.is_present("css") {
			export_css(&centroids, &full_path, "colors.css");
		}
	}
}
