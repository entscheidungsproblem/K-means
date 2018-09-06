pub use clustering::distances::closest as closest;


use data::Pixel as Pixel;
use data::CentroidPixel as CentroidPixel;
//use data::k_means as k_means;
use data::ColorPixel as ColorPixel;

//use palette::{Lch,LabHue,IntoColor,Srgb};

//use rayon::prelude::*;
//use rayon::collections::hash_map;
//use rayon::iter::Chunks;
use std::f32;
use std::collections::VecDeque;
//use std::process;
//use image::RgbImage;
use rand::distributions::{IndependentSample, Range};
use rand::{Rng, thread_rng};

//use std::fs::File;




pub fn kmeans_pp_init(k: u8, pixels:&VecDeque<ColorPixel>) -> VecDeque <CentroidPixel> {

	let mut centroids:VecDeque<CentroidPixel> = VecDeque::with_capacity(k as usize);
	fn insert_centroid(cp: &ColorPixel, centroids:&mut VecDeque<CentroidPixel>) {
		let rgb = cp.p.base_colors;
		centroids.insert(0, CentroidPixel{p: Pixel{base_colors: rgb}, sum:(0_f32, 0_f32, 0_f32), count:0_u32 });
	}

	let mut rng = thread_rng();
	let between = Range::new(0, pixels.len());
	let i = between.ind_sample(&mut rng);

	insert_centroid(&pixels[i], &mut centroids);

    for _x in 1..k{
    	let mut distances: VecDeque<f32> = VecDeque::with_capacity(k as usize);
    	let mut sum = 0_f32;
    	for p in 0..pixels.len(){
		let close = closest(&pixels[p].p, &centroids, "cie00").1;
		let square = close.powi(2);
		distances.insert(p, square);
		//let &mut d = distances.get_mut(p).unwrap();
		sum += square;
	}
	
	sum *= rng.next_f32();
	for p in 0..pixels.len(){
		sum -= distances.get(p).unwrap();
		if sum < 0_f32{
			insert_centroid(&pixels[p], &mut centroids);
			break;
		}
	}

    }
    return centroids;
}

pub fn kmeans_init(k: u8, pixels:&VecDeque<ColorPixel>) -> VecDeque <CentroidPixel> {
    let mut rng = thread_rng();
    let r = Range::new(0, pixels.len());
    let mut centroid:VecDeque<CentroidPixel> = VecDeque::with_capacity(k as usize);
    for _x in 0..k{
    	let i = r.ind_sample(&mut rng); 
	let p = pixels.get(i).unwrap();

	centroid.insert(0, CentroidPixel{p:Pixel{base_colors:p.p.base_colors}, sum:(0_f32, 0_f32, 0_f32), count:0_u32} );
    }
    return centroid;
}
