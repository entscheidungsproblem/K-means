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


use self::palette::{Lch,LabHue,IntoColor,Srgb,rgb};

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
    //println!("Path does exist: {:?}", full_path);
    let pixels:VecDeque<ColorPixel> = load_image(full_path.clone());
    let mut centroids: VecDeque<CentroidPixel> = kmeans_init(7_u8, &pixels);
    //let white = CentroidPixel {p:Pixel{base_colors:(100.0, 0.0, 270.0)}, sum:(0.0, 0.0, 0.0), count:0_u32};
    //let black = CentroidPixel {p:Pixel{base_colors:(0.0, 0.0, 0.0)}, sum:(0.0, 0.0, 0.0), count:0_u32};
    //centroids.insert(0, white);
    //centroids.insert(0, black);
    cluster_all(&pixels, &mut centroids, 50, 0.001);
    export_yaml(centroids, full_path);
    

    
}

fn export_json(centroids: VecDeque<CentroidPixel>, full_path: String){
    print!("{{ \n\t\"wallpaper\":\t\"{}\",\n\t\"colors\":\t{{\n", full_path);
    
    let mut i = 0;
    for c in centroids{
        let _lch = c.p.base_colors;
		let lch: Lch = Lch::new(_lch.0, _lch.1, LabHue::from(_lch.2));
		let rgb_color: rgb::Rgb<rgb::Linear> = lch.into_rgb();
        //println!("{:?}", ((rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8));
        if (i != 0){println!(",");}
        print!("\t\t\"color{}\":\t\"#{:0width$X}{:0width$X}{:0width$X}\"", i, (rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8, width=2);
        
        i+=1;
    }
    print!("\n\t}}\n}}");
    
}

fn export_yaml(centroids: VecDeque<CentroidPixel>, full_path: String){
    println!("wallpaper: {}", full_path);
    println!("colors:");
    
    let mut i = 0;
    for c in centroids{
        let _lch = c.p.base_colors;
		let lch: Lch = Lch::new(_lch.0, _lch.1, LabHue::from(_lch.2));
		let rgb_color: rgb::Rgb<rgb::Linear> = lch.into_rgb();
        //println!("{:?}", ((rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8));
        println!("\tcolor{}: #{:0width$X}{:0width$X}{:0width$X}", i, (rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8, width=2);
        
        i+=1;
    }   
}