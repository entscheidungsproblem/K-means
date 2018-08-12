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



use self::palette::{Lch,LabHue,IntoColor,Srgb};

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

fn main() {

    let pixels:VecDeque<ColorPixel> = load_image(String::from("test.jpg"));
    //for pixel in pixels.iter(){
    //	println!("{:?}", pixel.p.base_colors);
    //}
   
    //println!("------------------");
    //let mut centroids2: VecDeque<CentroidPixel> = kmeansPP_init(8_u8, &pixels);
    //println!("{}", centroids2.len());
    let mut centroids: VecDeque<CentroidPixel> = kmeans_init(6_u8, &pixels);
    let white = CentroidPixel {p:Pixel{base_colors:(100.0, 0.0, 270.0)}, sum:(0.0, 0.0, 0.0), count:0_u32};
    let black = CentroidPixel {p:Pixel{base_colors:(0.0, 0.0, 0.0)}, sum:(0.0, 0.0, 0.0), count:0_u32};
    //centroids.insert(0, white);
    //centroids.insert(0, black);
    //for c in centroids.iter(){
    //	println!("starting {:?}", c.p.base_colors);
    //}
	//println!("{}", centroids.len());
    /*display(&centroids, String::from("color"));
    for x in 1..100{
    	cluster(&pixels, &mut centroids);
	println!("{}", centroids.len());
	//for c in centroids.iter(){
	//	println!("{}th cluster {:?}", x, c.p.base_colors);
	//}
	let mut filename = String::from("color");
	filename.extend(x.to_string().chars());
	display(&centroids, filename);
    }*/
    cluster_all(&pixels, &mut centroids, 50, 0.001);

    //println!("distance {}", euclidean_distance(pixels.get(0..3).unwrap(),pixels.get(3..6).unwrap()));
    // println!("p1: {:?}, p2: {:?}", pixels.get(0..3).unwrap(), pixels.get(3..6).unwrap());
    //println!("centroid: {:?}", centroid( pixels.get(0..6).unwrap(), 2 ));

    /* println!("{:?}", pixels);
    for x in 0..(pixels.len()/3){
    		println!("{:?}", pixels.get((3*x) as usize..(3*x + 3) as usize).unwrap());
    }*/

    // let all_pixels = img.raw_pixels();
    // let mut _pixel  = all_pixels.chunks(3);
    // gen_starting_centre(5_u8);
    // let p1 = vec![1, 20, 30];
    // let p2 = vec![100, 200, 130];
    // euclidean_distance(p1,p2);
	  // for p in pixel{
	  // println!("{:?}", p.next().unwrap());
}
