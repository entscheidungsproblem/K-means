use data::{Centroid, Pixel};
use std::collections::VecDeque;
use std::f32;

pub fn distance(p1: &Pixel, p2: &Pixel, dist_func: &str) -> f32 {
	let out = match dist_func {
			"euclidean" => euclidean_distance(p1, p2),
			"cie00" => cie00_distance(p1, p2),
			"cie94" => cie94_distance(p1, p2),
			"contrast" => contrast_ratio(p1, p2),
			_ => -1f32,
	};
	out
}

pub fn closest(p: &Pixel, points: &Vec<Centroid>, dist_func: &str) -> (u32, f32) {
	let mut close_p: u32 = 0_u32;
	let mut close_dist = f32::MAX;
	for (counter, next_p) in points.iter().enumerate() {
		let temp_dist: f32 = distance(p, &next_p.location, dist_func);
		if close_dist > temp_dist {
			close_p = counter as u32;
			close_dist = temp_dist;
		}
	}
	(close_p, close_dist)
}

fn euclidean_distance(p1: &Pixel, p2: &Pixel) -> f32 {
	let mut s: f32 = 0.0;
	s += (p1.l - p2.l).powi(2);
	s += (p1.c - p2.c).powi(2);
	s += (p1.h - p2.h).powi(2);
	s = s.sqrt();
	s
}

fn div(num: f32, den: f32) -> f32 {
	if num == 0.0 && den == 0.0 {
		1_f32
	} else {
		num / den
	}
}

fn cie00_distance(p1: &Pixel, p2: &Pixel) -> f32 {
	let twenth_five_pow7 = 6_103_515_625.0;
	let l2 = p2.l;
	let a2 = p2.c;
	let b2 = p2.h;

	let l1 = p1.l;
	let a1 = p1.c;
	let b1 = p1.h;

	let l_bar_prime = (l1 + l2) / 2.0;
	let c1 = (a1.powi(2) + b1.powi(2)).sqrt();
	let c2 = (a2.powi(2) + b2.powi(2)).sqrt();

	let c_bar = (c1 + c2) / 2.0;
	let g = (1.0 - (c_bar.powi(7) / (c_bar.powi(7) + twenth_five_pow7)).sqrt()) / 2.0;
	let a1_prime = a1 * (1.0 + g);
	let a2_prime = a2 * (1.0 + g);
	let c1_prime = (a1_prime.powi(2) + b1.powi(2)).sqrt();
	let c2_prime = (a2_prime.powi(2) + b2.powi(2)).sqrt();
	let c_bar_prime = (c1_prime + c2_prime) / 2.0;

	let mut h1_prime = div(b1, a1_prime).atan();
	if h1_prime < 0.0 {
		h1_prime += 360.0;
	}
	let mut h2_prime = div(b2, a2_prime).atan();
	if h2_prime < 0.0 {
		h2_prime += 360.0;
	}
	let mut h_bar_prime = h1_prime + h2_prime;
	if (h1_prime - h2_prime).abs() > 180.0 {
		h_bar_prime += 360.0;
	}
	h_bar_prime /= 2.0;

	let t = 1.0 - 0.17 * (h_bar_prime - 30.0).cos()
		+ 0.24 * (2.0 * h_bar_prime).cos()
		+ 0.32 * (3.0 * h_bar_prime + 6.0).cos()
		- 0.2 * (4.0 * h_bar_prime - 63.0).cos();
	let mut _delta_h_prime = h2_prime - h1_prime;
	if _delta_h_prime.abs() > 180.0 && h2_prime <= h1_prime {
		_delta_h_prime += 360.0;
	} else if _delta_h_prime.abs() > 180.0 {
		_delta_h_prime -= 360.0;
	}

	let delta_l = l2 - l1;
	let delta_c_prime = c2_prime - c1_prime;

	let delta_h_prime = 2.0 * (c1_prime * c2_prime).sqrt() * (_delta_h_prime / 2.0).sin();
	let sl =
		1.0 + (0.015 * (l_bar_prime - 50.0).powi(2)) / (20.0 + (l_bar_prime - 50.0).powi(2)).sqrt();

	let k1 = 0.045;
	let k2 = 0.015;

	let sc = 1_f32 + k1 * c_bar_prime;
	let sh = 1_f32 + k2 * c_bar_prime * t;

	let kl = 1_f32;
	let kc = 1_f32;
	let kh = 1_f32;

	let delta_degrees = 30.0 * (-((h_bar_prime - 275.0) / 25.0).powi(2)).exp();
	let rc = 2.0 * (c_bar_prime.powi(7) / (c_bar_prime.powi(7) + twenth_five_pow7)).sqrt();
	let rt = -rc * (2.0 * delta_degrees).sin();

	((delta_l / (kl * sl)).powi(2)
		+ (delta_c_prime / (kc * sc)).powi(2)
		+ (delta_h_prime / (kh * sh)).powi(2)
		+ rt * (delta_c_prime / (kc * sc)) * (delta_h_prime / (kh * sh)))
		.sqrt()
}

fn cie94_distance(p1: &Pixel, p2: &Pixel) -> f32 {
	let delta_l = p1.l - p2.l;
	let c1 = (p1.c.powi(2) + p1.h.powi(2)).sqrt();
	let c2 = (p2.c.powi(2) + p2.h.powi(2)).sqrt();
	let delta_c = c1 - c2;
	let delta_a = (p1.c - p2.c).powi(2);
	let delta_b = (p1.h - p2.h).powi(2);
	let delta_h = delta_a + delta_b - delta_c.powi(2);
	let kl = 1_f32;
	let kc = 1_f32;
	let kh = 1_f32;
	let k1 = 0.045;
	let k2 = 0.015;
	let sl = 1_f32;
	let sc = 1_f32 + k1 * c1;
	let sh = 1_f32 + k2 * c1;

	((delta_l / (kl * sl).powi(2)) + (delta_c / (kc * sc).powi(2)) + delta_h / ((kh * sh).powi(2)))
		.sqrt()
}

fn contrast_ratio(p1: &Pixel, p2: &Pixel) -> f32 {
	let l1 = p1.l;
	let l2 = p2.l;
	let mut num = 0.05;
	let mut den = 0.05;
	if l1 > l2 {
		num += l1;
		den += l2;
	} else {
		num += l2;
		den += l1;
	}
	num / den
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_euclidean_distance() {
		let p1: Pixel = Pixel::new(0_f32, 0_f32, 0_f32, 0_u32);
		let p2: Pixel = Pixel::new(0_f32, 0_f32, 0_f32, 0_u32);
		assert_eq!(0_f32, euclidean_distance(&p1, &p2));
	}

	#[test]
	fn test_cie00_distance() {
		let p1: Pixel = Pixel::new(0_f32, 0_f32, 0_f32, 0_u32);
		let p2: Pixel = Pixel::new(0_f32, 0_f32, 0_f32, 0_u32);
		let x = cie00_distance(&p1, &p2);
		assert_eq!(0_f32, x);
	}

	#[test]
	fn test_cie94_distance() {
		let p1: Pixel = Pixel::new(0_f32, 0_f32, 0_f32, 0_u32);
		let p2: Pixel = Pixel::new(0_f32, 0_f32, 0_f32, 0_u32);
		assert_eq!(0_f32, cie94_distance(&p1, &p2));
	}
}
