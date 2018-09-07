pub use clustering::distances::closest;
use data::{CentroidPixel, ColorPixel, Pixel};
use rand::distributions::{IndependentSample, Range};
use rand::{thread_rng, Rng};
use std::collections::VecDeque;
use std::f32;

pub fn kmeans_pp_init(k: u8, pixels: &VecDeque<ColorPixel>) -> VecDeque<CentroidPixel> {
	let mut centroids: VecDeque<CentroidPixel> = VecDeque::with_capacity(k as usize);
	fn insert_centroid(cp: &ColorPixel, centroids: &mut VecDeque<CentroidPixel>) {
		let rgb = cp.p.base_colors;
		centroids.insert(
			0,
			CentroidPixel {
				p: Pixel { base_colors: rgb },
				sum: (0_f32, 0_f32, 0_f32),
				count: 0_u32,
			},
		);
	}

	let mut rng = thread_rng();
	let between = Range::new(0, pixels.len());
	let i = between.ind_sample(&mut rng);
	insert_centroid(&pixels[i], &mut centroids);

	for _x in 1..k {
		let mut distances: VecDeque<f32> = VecDeque::with_capacity(k as usize);
		let mut sum = 0_f32;
		for (p, _) in pixels.iter().enumerate() {
			let close = closest(&pixels[p].p, &centroids, "cie00").1;
			let square = close.powi(2);
			distances.insert(p, square);
			//let &mut d = distances.get_mut(p).unwrap();
			sum += square;
		}

		sum *= rng.next_f32();
		for (p, _) in pixels.iter().enumerate() {
			sum -= &distances[p];
			if sum < 0_f32 {
				insert_centroid(&pixels[p], &mut centroids);
				break;
			}
		}
	}
	centroids
}

pub fn kmeans_init(k: u8, pixels: &VecDeque<ColorPixel>) -> VecDeque<CentroidPixel> {
	let mut rng = thread_rng();
	let r = Range::new(0, pixels.len());
	let mut centroid: VecDeque<CentroidPixel> = VecDeque::with_capacity(k as usize);
	for _x in 0..k {
		let i = r.ind_sample(&mut rng);
		let p = pixels.get(i).unwrap();
		centroid.insert(
			0,
			CentroidPixel {
				p: Pixel {
					base_colors: p.p.base_colors,
				},
				sum: (0_f32, 0_f32, 0_f32),
				count: 0_u32,
			},
		);
	}
	centroid
}
