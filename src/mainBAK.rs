#![crate_type = "dylib"]
extern crate image;
extern crate rand;
extern crate rayon;
extern crate palette;

use palette::{Lch,LabHue,IntoColor,Srgb};

use rayon::prelude::*;
//use rayon::collections::hash_map;
//use rayon::iter::Chunks;
use std::f32;
use std::collections::{VecDeque, HashMap};
use std::process;
use image::RgbImage;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;

use std::fs::File;

/*
fn load_image() -> Vec<u8> {
    // Use the open function to load an image from a Path.
    // ```open``` returns a `DynamicImage` on success.
    let img = image::open("data/test.jpg").unwrap();
    let all_pixels = img.raw_pixels();
    // println!("{:?}", all_pixels);
    return all_pixels;
    let pixels: Vec<Pixel> = Vec::new();

    for p in 0..all_pixels.len()/3{
        pixels.insert(0, Pixel{r: all_pixels[3*p], g:all_pixels[3*p+1], b:all_pixels[3*p+2]});
    }
    return pixels;
}

fn get_point(points: Vec<u8>, index: u8) -> &[u8]{
    return points.get((3*index) as usize..(3*index+3) as usize).unwrap();
}
*/

fn kmeansPP_init(k: u8, pixels:&VecDeque<ColorPixel>) -> VecDeque <CentroidPixel> {

	let mut centroids:VecDeque<CentroidPixel> = VecDeque::with_capacity(k as usize);
	fn insert_centroid(cp: &ColorPixel, centroids:&mut VecDeque<CentroidPixel>) {
		let rgb = cp.p.base_colors;
		centroids.insert(0, CentroidPixel{p: Pixel{base_colors: rgb}, sum:(0_f32, 0_f32, 0_f32), count:0_u32 });
	}

	let mut rng = rand::thread_rng();
	let between = Range::new(0, pixels.len());
	let i = between.ind_sample(&mut rng);

	insert_centroid(&pixels[i], &mut centroids);


    for _x in 1..k{
    	let mut distances: VecDeque<f32> = VecDeque::with_capacity(k as usize);
    	let mut sum = 0_f32;
    	for p in 0..pixels.len(){
		let close = closest(&pixels[p], &centroids).1;
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


fn kmeans_init(k: u8, pixels:&VecDeque<ColorPixel>) -> VecDeque <CentroidPixel> {
    let mut rng = rand::thread_rng();
    let r = Range::new(0, pixels.len());
    /*
    let l = Range::new(0_f32, 100_f32);
    let c = Range::new(0_f32, 128_f32);
    let h = Range::new(-180_f32, 180_f32);
    */
    let mut centroid:VecDeque<CentroidPixel> = VecDeque::with_capacity(k as usize);
    for _x in 0..k{
    	let i = r.ind_sample(&mut rng); 
	let p = pixels.get(i).unwrap();

	centroid.insert(0, CentroidPixel{p:Pixel{base_colors:p.p.base_colors}, sum:(0_f32, 0_f32, 0_f32), count:0_u32} );
	//centroid.insert(0, CentroidPixel{p:Pixel{base_colors:(l.ind_sample(&mut rng), c.ind_sample(&mut rng), h.ind_sample(&mut rng))}, sum:(0_f32, 0_f32, 0_f32), count:0_u32} );
    }
    return centroid;
}

fn euclidean_distance(p1: &Pixel, p2: &Pixel) -> f32{
    let mut s: f32 = 0.0_f32;
    s += (p1.base_colors.0 - p2.base_colors.0).powi(2);
    s += (p1.base_colors.1 - p2.base_colors.1).powi(2);
    s += (p1.base_colors.2 - p2.base_colors.2).powi(2);
    //s += ((p1.l as i16 - p2.l as i16).abs() as f32).powi(2);
    //s += ((p1.c as i16 - p2.c as i16).abs() as f32).powi(2);
    //s += ((p1.h as i16 - p2.h as i16).abs() as f32).powi(2);
    s = s.sqrt();
    return s;
}

fn cie00_distance (p1: &Pixel, p2: &Pixel) -> f32{
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

fn cie94_distance (p1: &Pixel, p2: &Pixel) -> f32{
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

fn closest(p:&ColorPixel, points: &VecDeque<CentroidPixel>) -> (u8, f32){
    let mut close_p: u8 = 0_u8;
    let mut close_dist = f32::MAX;
    let mut counter = 0_u8;
    for next_p in points{
        let temp_dist = cie00_distance(&p.p, &next_p.p);
        if close_dist > temp_dist {
            close_p = counter;
            close_dist = temp_dist;
        }
	counter += 1;
    }
    return (close_p, close_dist);
}

fn display(centroids:&VecDeque<CentroidPixel>, name:String) {
	let k = centroids.len();
	let imgx = k*100;
	let imgy = 400;

	let mut centroid_rgb: VecDeque<(u8,u8,u8)> = VecDeque::with_capacity(centroids.len());
	for c in centroids{
		let _lch = c.p.base_colors;
		let lch: Lch = Lch::new(_lch.0, _lch.1, LabHue::from(_lch.2));
		let rgb: palette::rgb::Rgb<palette::rgb::Linear> = lch.into_rgb(); 
		centroid_rgb.insert(0, ((rgb.red * 255.0) as u8, (rgb.green * 255.0) as u8, (rgb.blue * 255.0) as u8));
		//println!("{:?}", centroid_rgb[0]);
	}
	//println!("------------------");
	// Create a new ImgBuf with width: imgx and height: imgy
	let mut imgbuf = image::ImageBuffer::new(imgx as u32, imgy as u32);
	// Iterate over the coordinates and pixels of the image
	for (x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
		let color_index = x / 100;
		let rgb = centroid_rgb[color_index as usize];
		*pixel = image::Rgb([rgb.0,rgb.1,rgb.2]);
		//println!("{:?}", pixel);
		//println!("{},{},{}", rgb.red,rgb.green,rgb.blue);
	}
	let mut filename = String::from("data/");
	filename.extend(name.chars());
	filename.extend(".png".chars());
	let fout = &mut File::create(filename).unwrap();

	// We must indicate the image's color type and what format to save as
	image::ImageRgb8(imgbuf).save(fout, image::PNG).unwrap();
}
/*

fn centroid(points: HashMap<Pixel, u32>, k: usize) -> HashMap<Pixel, u32> {
    let mut r: u8 = 0_u8;
    let mut g: u8 = 0_u8;
    let mut b: u8 = 0_u8;
    // let k: usize = points.len();
    for p in 0_u8..(k/3) as u8{
        let temp = points.get((3*p) as usize..(3*p+3) as usize).unwrap();
        println!("r: {}, t: {}", r, temp[0]);
        r += temp[0]/(k as u8);
        g += temp[1]/(k as u8);
        b += temp[2]/(k as u8);
    }
    return vec![r,g,b];
}

 */

//#[derive (PartialEq, Eq)]
struct Pixel{
    base_colors : (f32, f32, f32),
}

impl Pixel{
	fn distance<F: Fn(&Pixel, &Pixel)->f32>(&self, other: &Pixel, dist_func: F) -> f32 {
		return dist_func(self, other);
	}
}

struct CentroidPixel{
	p: Pixel,
	sum: (f32, f32, f32),
	count: u32,
}

struct k_means{
	pixels: VecDeque<ColorPixel>, 
	centroids: VecDeque<CentroidPixel>,
}



/*
impl Pixel{
    fn update(&self, new_colors: (f32, f32, f32)) {
        self.base_colors = new_colors;
    }
}
*/

struct ColorPixel {
    p: Pixel,
    //dist: f32,
    //centroid: u32,
    count: u32,
}

fn load_image( name: String) -> VecDeque<ColorPixel>{
	let mut filename = String::from("data/");
	filename.extend(name.chars());
	//filename.extend(".png".chars());
	
	let img1 = image::open(filename).unwrap();
    	//if img1.is_err(){process::exit(1); }
    	let img2:Option<&RgbImage> = img1.as_rgb8();
    	if img2.is_none(){ process::exit(1); }
    	let img3 = img2.unwrap();

    	let num_pixels: usize = (img3.height() * img3.width()) as usize;

    	let mut rgb_pixels: HashMap<(u8,u8,u8), u32> = HashMap::with_capacity(num_pixels);
    	for _pixel in img3.chunks(3){
    	  let _rgb = (_pixel[0], _pixel[1], _pixel[2]);
	      if !rgb_pixels.contains_key(&_rgb){
		        rgb_pixels.insert(_rgb,1);
	      }
	      else{
	      		let count = rgb_pixels.get_mut(&_rgb).unwrap();
			*count += 1;
			//rgb_pixels.remove(_rgb);
		        //rgb_pixels.insert(_rgb, count);
	      }
    }

    let mut pixels: VecDeque<ColorPixel> = VecDeque::with_capacity(num_pixels);
    for (_pixel, count) in rgb_pixels.iter() {
    	  let lch_color: Lch = Srgb::new((_pixel.0 as f32)/(256 as f32), (_pixel.1 as f32)/(256 as f32), (_pixel.2 as f32)/(256 as f32)).into();
    	  //let rgb  = Rgb::new_u8(_pixel[0], _pixel[1],  _pixel[2]);
	      //println!("{}", rgb);
	      //let lch = Srgb.into_linear().into_lab();
	      //println!("l: {}, c:{}, h:{}", lch_color.l, lch_color.chroma, lch_color.hue.to_degrees());
        let p = Pixel{base_colors:(lch_color.l, lch_color.chroma, lch_color.hue.to_degrees())};
	      pixels.insert(0, ColorPixel{p:p, count:*count});
    }
    //println!("{}", pixels);
    return pixels;
}

fn cluster (pixels: &VecDeque<ColorPixel>, centroids: & mut VecDeque<CentroidPixel>) {
	println!("-----Clustering-----");
	for pixel in pixels.iter(){
    		let (i, _dist) = closest(pixel, &centroids);
		let mut c = centroids.get_mut(i as usize).unwrap();
		let num = pixel.count;
		
		c.sum.0 += pixel.p.base_colors.0 * num as f32;
		c.sum.1 += pixel.p.base_colors.1 * num as f32;
		c.sum.2 += pixel.p.base_colors.2 * num as f32;
		c.count += num;
    	}

    for c in centroids{
	if c.count > 0 {
		//println!("{}, {}, {}", c.sum.0/c.count as f32, c.sum.1/c.count as f32, c.sum.2/c.count as f32);
		c.p.base_colors = (c.sum.0/c.count as f32, c.sum.1/c.count as f32, c.sum.2/c.count as f32);
		c.sum = (0_f32, 0_f32, 0_f32);
		c.count = 0_u32;
	}
	else{
		println!("Centroid Error. Count = {}", c.count);
	}
	// TODO else, bad centroid
    }
}

fn clustering (pixels: &VecDeque<ColorPixel>, centroids: & mut VecDeque<CentroidPixel>, rounds:usize, delta:f32) {
	fn update (distance: &mut VecDeque<f32>, centroids: &VecDeque<CentroidPixel>) -> f32 {
		let mut delta = 0.0;
		for x in 0..centroids.len(){
			let val = (centroids[x].p.base_colors.0.powi(2) + centroids[x].p.base_colors.1.powi(2) + centroids[x].p.base_colors.2.powi(2)).sqrt();
			delta += ((val - distance[x])/distance[x]).abs();
			distance[x] = val;
		}
		return delta/centroids.len() as f32;	
	}
	
	display(&centroids, String::from("color"));
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
		println!("{}", change);
		let mut filename = String::from("color");
		filename.extend(x.to_string().chars());
		display(&centroids, filename);
		x+=1;
	}
}


fn contrast_ratio(p1:&Pixel, p2:&Pixel) -> f32{
	let l1 = p1.base_colors.2;
	let l2 = p2.base_colors.2;
	if l1 > l2{
		return (l1 + 0.05)/(l2 + 0.05);
	}
	return (l2 + 0.05)/(l1 + 0.05);
}



//fn main() {
pub extern fn run() {
    let pixels:VecDeque<ColorPixel> = load_image(String::from("small.jpg"));
    //for pixel in pixels.iter(){
    //	println!("{:?}", pixel.p.base_colors);
    //}
   
    //println!("------------------");
    //let mut centroids2: VecDeque<CentroidPixel> = kmeansPP_init(8_u8, &pixels);
    //println!("{}", centroids2.len());
    let mut centroids: VecDeque<CentroidPixel> = kmeans_init(14_u8, &pixels);
    let white = CentroidPixel {p:Pixel{base_colors:(100.0, 0.0, 270.0)}, sum:(0.0, 0.0, 0.0), count:0_u32};
    let black = CentroidPixel {p:Pixel{base_colors:(0.0, 0.0, 0.0)}, sum:(0.0, 0.0, 0.0), count:0_u32};
    centroids.insert(0, white);
    centroids.insert(0, black);
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
    clustering(&pixels, &mut centroids, 50, 0.001);

    

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

