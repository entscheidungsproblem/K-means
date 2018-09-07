pub mod distances;
pub mod init;
use self::distances::closest as closest;
use data::{CentroidPixel, ColorPixel};
use std::f32;
use std::collections::VecDeque;

fn cluster (pixels: &VecDeque<ColorPixel>, centroids: & mut VecDeque<CentroidPixel>) {
	for pixel in pixels.iter(){
    	let (i, _dist) = closest(&pixel.p, &centroids, "cie00");
		let mut c = centroids.get_mut(i as usize).unwrap();
		let num = pixel.count;
		
		c.sum.0 += pixel.p.base_colors.0 * num as f32;
		c.sum.1 += pixel.p.base_colors.1 * num as f32;
		c.sum.2 += pixel.p.base_colors.2 * num as f32;
		c.count += num;
	}

    for c in centroids{
		if c.count > 0 {
			c.p.base_colors = (c.sum.0/c.count as f32, c.sum.1/c.count as f32, c.sum.2/c.count as f32);
			c.sum = (0_f32, 0_f32, 0_f32);
			c.count = 0_u32;
		}
		else{
			println!("Centroid Error. Count = {}", c.count);
		}
    }
}

pub fn cluster_all (pixels: &VecDeque<ColorPixel>, centroids: & mut VecDeque<CentroidPixel>, rounds:usize, delta:f32) {
	fn update (distance: &mut VecDeque<f32>, centroids: &VecDeque<CentroidPixel>) -> f32 {
		let mut _delta = 0.0;
		for x in 0..centroids.len(){
			let val = (centroids[x].p.base_colors.0.powi(2) + centroids[x].p.base_colors.1.powi(2) + centroids[x].p.base_colors.2.powi(2)).sqrt();
			_delta += ((val - distance[x])/distance[x]).abs();
			distance[x] = val;
		}
		_delta/centroids.len() as f32
	}
	
	// display(&centroids, String::from("color"));
	let mut distance: VecDeque<f32> = VecDeque::with_capacity(centroids.len());
	for c in centroids.iter(){
		let val = (c.p.base_colors.0.powi(2) + c.p.base_colors.1.powi(2) + c.p.base_colors.2.powi(2)).sqrt();
		distance.push_back(val);
	}
    	cluster(pixels, centroids);
	let mut change = update(&mut distance, centroids);
	let mut x = 0;
	while x < rounds && change > delta{
    		cluster(pixels, centroids);
		change = update(&mut distance, centroids);
		//println!("{}", change);
		//let mut filename = String::from("color");
		//filename.extend(x.to_string().chars());
		// display(&centroids, filename);
		x+=1;
	}
}