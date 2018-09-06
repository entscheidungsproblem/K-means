use data::{Pixel, CentroidPixel, ColorPixel};
use palette::{Lch,LabHue,IntoColor,Srgb,rgb};
use image::{RgbImage, ImageBuffer, Rgb, ImageRgb8, PNG, open};
use std::f32;
use std::collections::{VecDeque, HashMap};
use std::process;
use std::fs::File;



pub fn display(centroids:&VecDeque<CentroidPixel>, name:String) {
	let k = centroids.len();
	let imgx = k*100;
	let imgy = 400;

	let mut centroid_rgb: VecDeque<(u8,u8,u8)> = VecDeque::with_capacity(centroids.len());
	for c in centroids{
		let _lch = c.p.base_colors;
		let lch: Lch = Lch::new(_lch.0, _lch.1, LabHue::from(_lch.2));
		let rgb: rgb::Rgb<rgb::Linear> = lch.into_rgb(); 
		centroid_rgb.insert(0, ((rgb.red * 255.0) as u8, (rgb.green * 255.0) as u8, (rgb.blue * 255.0) as u8));
		//println!("{:?}", centroid_rgb[0]);
	}
	//println!("------------------");
	// Create a new ImgBuf with width: imgx and height: imgy
	let mut imgbuf = ImageBuffer::new(imgx as u32, imgy as u32);
	// Iterate over the coordinates and pixels of the image
	for (x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
		let color_index = x / 100;
		let rgb = centroid_rgb[color_index as usize];
		*pixel = Rgb([rgb.0,rgb.1,rgb.2]);
		//println!("{:?}", pixel);
		//println!("{},{},{}", rgb.red,rgb.green,rgb.blue);
	}
	let mut filename = String::from("data/");
	filename.extend(name.chars());
	filename.extend(".png".chars());
	let fout = &mut File::create(filename).unwrap();

	// We must indicate the image's color type and what format to save as
	ImageRgb8(imgbuf).save(fout, PNG).unwrap();
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

pub fn load_image( name: String) -> VecDeque<ColorPixel>{
	//let mut filename = String::from("data/");
	//filename.extend(name.chars());
	//filename.extend(".png".chars());
	
	let img1 = open(name).unwrap();
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