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
//use rand::distributions::{IndependentSample, Range};
//use rand::Rng;
//use std::fs::File;


pub fn closest(p:&Pixel, points: &VecDeque<CentroidPixel>, dist_func: &str) -> (u8, f32) {
    let mut close_p: u8 = 0_u8;
    let mut close_dist = f32::MAX;
    let mut counter = 0_u8;
    for next_p in points{
		let temp_dist: f32 = match dist_func {
			"euclidean" => euclidean_distance(p, &next_p.p),
			"cie00" => cie00_distance(p, &next_p.p),
			"cie94" => cie94_distance(p, &next_p.p),
			"contrast" => contrast_ratio(p, &next_p.p),
			_ => -1f32,
		};

		//let temp_dist = cie00_distance(&p.p, &next_p.p);
        if close_dist > temp_dist {
            close_p = counter;
            close_dist = temp_dist;
        }
	counter += 1;
    }
    return (close_p, close_dist);
}

fn euclidean_distance (p1: &Pixel, p2: &Pixel) -> f32 {
    let mut s: f32 = 0.0_f32;
    s += (p1.base_colors.0 - p2.base_colors.0).powi(2);
    s += (p1.base_colors.1 - p2.base_colors.1).powi(2);
    s += (p1.base_colors.2 - p2.base_colors.2).powi(2);
    s = s.sqrt();
    return s;
}

fn cie00_distance (p1: &Pixel, p2: &Pixel) -> f32 {
	let twenth_five_pow7 = 6103515625.0;
	let l2 = p2.base_colors.0;
	let a2 = p2.base_colors.1;
	let b2 = p2.base_colors.2;
	
	let l1 = p1.base_colors.0;
	let a1 = p1.base_colors.1;
	let b1 = p1.base_colors.2;

	let l_bar_prime = (l1 + l2)/2.0;
	let c1 = (a1.powi(2) + b1.powi(2)).sqrt();
	let c2 = (a2.powi(2) + b2.powi(2)).sqrt();
	let c_bar = (c1 + c2)/2.0;
	let g = (1.0 - (c_bar.powi(7)/(c_bar.powi(7)+ twenth_five_pow7)).sqrt())/2.0;
	let a1_prime = a1 * (1.0 + g);
	let a2_prime = a2 * (1.0 + g);
	let c1_prime = (a1_prime.powi(2) + b1.powi(2)).sqrt(); 
	let c2_prime = (a2_prime.powi(2) + b2.powi(2)).sqrt(); 
	let c_bar_prime = (c1_prime + c2_prime)/2.0;

	let mut h1_prime = (b1/a1_prime).atan();
	if h1_prime < 0.0 {
		h1_prime += 360.0;
	}
	let mut h2_prime = (b2/a2_prime).atan();
	if h2_prime < 0.0 {
		h2_prime += 360.0;
	}
	let mut h_bar_prime = h1_prime + h2_prime;
	if (h1_prime - h2_prime).abs() > 180.0{
		h_bar_prime += 360.0;
	}
	h_bar_prime /= 2.0;

	let t = 1.0 - 0.17*(h_bar_prime - 30.0).cos() + 0.24*(2.0*h_bar_prime).cos() + 0.32*(3.0*h_bar_prime + 6.0).cos() - 0.2*(4.0*h_bar_prime - 63.0).cos();
	let mut _delta_h_prime = h2_prime - h1_prime;
	if _delta_h_prime.abs() > 180.0 && h2_prime <= h1_prime {
		_delta_h_prime += 360.0;
	}
	else if _delta_h_prime.abs() > 180.0  {
		_delta_h_prime -= 360.0;
	}

	let delta_l = l2 - l1;
	let delta_c_prime = c2_prime - c1_prime;
	
	let delta_h_prime = 2.0* (c1_prime * c2_prime).sqrt()*(_delta_h_prime/2.0).sin();
	let sl = 1.0 + (0.015*(l_bar_prime - 50.0).powi(2) )/(20.0 + (l_bar_prime - 50.0).powi(2)).sqrt();
	
	let k1 = 0.045;
	let k2 = 0.015;
	
	let sc = 1_f32 + k1*c_bar_prime;
	let sh = 1_f32 + k2*c_bar_prime*t;

	let kl = 1_f32;
	let kc = 1_f32;
	let kh = 1_f32;

	let delta_degrees = 30.0 * (-((h_bar_prime - 275.0)/25.0).powi(2)).exp();
	let rc = 2.0*(c_bar_prime.powi(7)/(c_bar_prime.powi(7) + twenth_five_pow7)).sqrt();
	let rt = -rc*(2.0*delta_degrees).sin();

	return ( (delta_l/(kl*sl)).powi(2) + (delta_c_prime/(kc*sc)).powi(2) +  (delta_h_prime/(kh*sh)).powi(2) + rt*(delta_c_prime/(kc*sc))*(delta_h_prime/(kh*sh)) ).sqrt();
}

fn cie94_distance (p1: &Pixel, p2: &Pixel) -> f32 {
	let delta_l = p1.base_colors.0 - p2.base_colors.0;
	let c1 = (p1.base_colors.1.powi(2) + p1.base_colors.2.powi(2)).sqrt();
	let c2 = (p2.base_colors.1.powi(2) + p2.base_colors.2.powi(2)).sqrt();
	let delta_c = c1 - c2;
	let delta_a = (p1.base_colors.1 - p2.base_colors.1).powi(2); 
	let delta_b = (p1.base_colors.2 - p2.base_colors.2).powi(2); 
	let delta_h = delta_a+delta_b-delta_c.powi(2);
	let kl = 1_f32;
	let kc = 1_f32;
	let kh = 1_f32;
	let k1 = 0.045;
	let k2 = 0.015;
	let sl = 1_f32;
	let sc = 1_f32 + k1*c1;
	let sh = 1_f32 + k2*c1;

	return ( (delta_l/(kl*sl).powi(2)) + (delta_c/(kc*sc).powi(2)) +  delta_h/((kh*sh).powi(2))  ).sqrt();
}

fn contrast_ratio (p1:&Pixel, p2:&Pixel) -> f32 {
	let l1 = p1.base_colors.2;
	let l2 = p2.base_colors.2;
	if l1 > l2{
		return (l1 + 0.05)/(l2 + 0.05);
	}
	return (l2 + 0.05)/(l1 + 0.05);
}