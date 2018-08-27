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

use clustering::cluster_all;

use data::Pixel as Pixel;
use data::CentroidPixel as CentroidPixel;
//use data::k_means as k_means;
use data::ColorPixel as ColorPixel;

use images::load_image as load_image;

use clustering::init::kmeans_init as kmeans_init;

use clap::{Arg, App, SubCommand};


//use self::palette::{Lch,LabHue,IntoColor,Srgb};

//use rayon::prelude::*;
//use rayon::collections::hash_map;
//use rayon::iter::Chunks;
use std::f32;
use std::collections::{VecDeque, HashMap};
use std::process;
use self::image::RgbImage;
use self::rand::distributions::{IndependentSample, Range};
use self::rand::Rng;

use std::fs::File;

use std::fs;
use std::path::{PathBuf, Path};


fn main() {


    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let _srcdir = fs::canonicalize(&PathBuf::from(matches.value_of("INPUT").unwrap()));

    if _srcdir.is_err(){
        println!("Path error!");
        return;
    }
    let srcdir = _srcdir.unwrap();
    if !(srcdir.exists()){
        println!("Path doesn't exist!");
        return;
    }
    let full_path = String::from(srcdir.to_str().unwrap());
    println!("Path does exist: {:?}", full_path);
    let pixels:VecDeque<ColorPixel> = load_image(full_path);
    let mut centroids: VecDeque<CentroidPixel> = kmeans_init(6_u8, &pixels);
    //let white = CentroidPixel {p:Pixel{base_colors:(100.0, 0.0, 270.0)}, sum:(0.0, 0.0, 0.0), count:0_u32};
    //let black = CentroidPixel {p:Pixel{base_colors:(0.0, 0.0, 0.0)}, sum:(0.0, 0.0, 0.0), count:0_u32};
    //centroids.insert(0, white);
    //centroids.insert(0, black);
    cluster_all(&pixels, &mut centroids, 50, 0.001);

    //for c in centroids{
        //let rgb_color = lch_to_rgb(c.p.base_colors);
        //let lch_color: Lch = Lch::new(c.p.base_colors.0, c.p.base_colors.1, Lch::from_degrees(c.p.base_colors.2));
        //Pixel{base_colors:(lch_color.l, lch_color.chroma, lch_color.hue.to_degrees())};
        //println!("{:?}, {:?}", c.p.base_colors, rgb_color);
    //}

    
}
