#[cfg_attr(feature = "cargo-clippy", allow(needless_lifetimes))]
#[macro_use]
extern crate clap;
extern crate image;
extern crate rand;
extern crate rayon;
extern crate palette;

pub mod images;
pub mod data;
pub mod clustering;
pub mod export;

use export::{export_json, export_yaml, export_sh, export_css};
use clustering::cluster_all;
use data::{CentroidPixel, ColorPixel};
use images::load_image;
use clustering::init::kmeans_init;
use clap::App;

use std::collections::VecDeque;

use std::fs;
use std::path::PathBuf;

fn check_path(path: &str) -> Option<String> {
    let _srcdir = fs::canonicalize(&PathBuf::from(path));
    if _srcdir.is_err(){
        eprintln!("Path error!");
        return None;
    }
    let srcdir = _srcdir.unwrap();
    if !(srcdir.exists()){
        eprintln!("Path doesn't exist!");
        return None;
    }
    return Some(String::from(srcdir.to_str().unwrap()));
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let check: Option<String> = check_path(matches.value_of("INPUT").unwrap());
    if check.is_none(){return}

    let full_path: String = check.unwrap();

    //println!("Path does exist: {:?}", full_path);
    let pixels:VecDeque<ColorPixel> = load_image(full_path.clone());
    let mut centroids: VecDeque<CentroidPixel> = kmeans_init(7_u8, &pixels);
    //let white = CentroidPixel {p:Pixel{base_colors:(100.0, 0.0, 270.0)}, sum:(0.0, 0.0, 0.0), count:0_u32};
    //let black = CentroidPixel {p:Pixel{base_colors:(0.0, 0.0, 0.0)}, sum:(0.0, 0.0, 0.0), count:0_u32};
    //centroids.insert(0, white);
    //centroids.insert(0, black);
    cluster_all(&pixels, &mut centroids, 50, 0.01);

    if matches.is_present("json")   { export_json(&centroids, full_path.clone(), "colors.json".to_string()); }
    if matches.is_present("yaml")   { export_yaml(&centroids, full_path.clone(), "colors.yml".to_string()); }
    if matches.is_present("sh")     { export_sh(&centroids, full_path.clone(), "colors.sh".to_string()); }
    if matches.is_present("css")    { export_css(&centroids, full_path.clone(), "colors.css".to_string()); }
}