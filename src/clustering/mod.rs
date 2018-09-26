pub mod distances;
pub mod init;
use self::distances::closest;
use data::{Centroid, Pixel};
use std::collections::VecDeque;
use std::f32;

fn cluster(pixels: &VecDeque<Pixel>, centroids: &mut VecDeque<Centroid>) {
	for (index, pixel) in pixels.iter().enumerate() {
		let (i, _dist) = closest(&pixel, &centroids, "cie00");
		let mut c = centroids.get_mut(i as usize).unwrap();
		c.add(index as u32, &pixel);
	}

	for c in centroids {
		if c.count > 0 {
			c.next();
		} else {
			println!("Centroid Error. Count = {}", c.count);
		}
	}
}

pub fn cluster_all(
	pixels: &VecDeque<Pixel>,
	centroids: &mut VecDeque<Centroid>,
	rounds: usize,
	delta: f32,
) {
	fn update(distance: &mut VecDeque<f32>, centroids: &VecDeque<Centroid>) -> f32 {
		let mut _delta = 0.0;
		for x in 0..centroids.len() {
			let val = (centroids[x].location.l.powi(2)
				+ centroids[x].location.c.powi(2)
				+ centroids[x].location.h.powi(2)).sqrt();
			_delta += ((val - distance[x]) / distance[x]).abs();
			distance[x] = val;
		}
		_delta / centroids.len() as f32
	}

	// display(&centroids, String::from("color"));
	let mut distance: VecDeque<f32> = VecDeque::with_capacity(centroids.len());
	for c in centroids.iter() {
		let val = (c.location.l.powi(2) + c.location.c.powi(2) + c.location.h.powi(2)).sqrt();
		distance.push_back(val);
	}
	cluster(pixels, centroids);
	let mut change = update(&mut distance, centroids);
	let mut x = 0;
	while x < rounds && change > delta {
		cluster(pixels, centroids);
		change = update(&mut distance, centroids);
		//println!("{}", change);
		//let mut filename = String::from("color");
		//filename.extend(x.to_string().chars());
		// display(&centroids, filename);
		x += 1;
	}
}
