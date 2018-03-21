extern crate image;
extern crate rand;


use std::f32;
use image::GenericImage;
use rand::Rng;

fn load_image() -> Vec<u8> {
    // Use the open function to load an image from a Path.
    // ```open``` returns a `DynamicImage` on success.
    let img = image::open("data/test.jpg").unwrap();
    let all_pixels = img.raw_pixels();
    // println!("{:?}", all_pixels);
    return all_pixels;
    /*
    let pixels: Vec<Pixel> = Vec::new();

    for p in 0..all_pixels.len()/3{
        pixels.insert(0, Pixel{r: all_pixels[3*p], g:all_pixels[3*p+1], b:all_pixels[3*p+2]});
    }
    return pixels;
    */
}
/*
fn get_point(points: Vec<u8>, index: u8) -> &[u8]{
    return points.get((3*index) as usize..(3*index+3) as usize).unwrap();
}
 */

fn gen_starting_centre(k: u8) -> Vec<u8> {
    let mut v: Vec<u8> = vec![];
	  let mut rng = rand::thread_rng();
    for _ in 0..k*3{
       v.push(rng.gen::<u8>());
    }
	  // println!("rand {:?}", v);
    return v;
}

fn euclidean_distance(p1: &[u8], p2: &[u8])-> f32{
    let mut s: f32 = 0.0_f32;
    for x in 0..3{
        s += ((p1[x] as i16 - p2[x] as i16).abs() as f32).powi(2);
    }
    s = s.sqrt();
    return s;
}

fn closest(p: &[u8], points: Vec<u8>) -> u8{
    let mut close_p: u8 = 0_u8;
    let mut close_dist = f32::MAX;
    for next_p in 0_u8..(points.len()/3) as u8{
        let temp_dist = euclidean_distance(p, points.get((3*next_p) as usize..(3*next_p+3) as usize).unwrap());
        if close_dist > temp_dist {
            close_p = next_p;
            close_dist = temp_dist;
        }
    }
    return close_p;
}

fn centroid(points: &[u8]) -> Vec<u8>{
    let mut r: u8 = 0_u8;
    let mut g: u8 = 0_u8;
    let mut b: u8 = 0_u8;
    let k: usize = points.len();
    for p in 0_u8..(k/3) as u8{
        let temp = points.get((3*p) as usize..(3*p+3) as usize).unwrap();
        println!("r: {}, t: {}", r, temp[0]);
        r += temp[0]/(k as u8);
        g += temp[1]/(k as u8);
        b += temp[2]/(k as u8);
    }
    return vec![r,g,b];
}

fn main() {

    let pixels: Vec<u8> = load_image();
    println!("starting {:?}", gen_starting_centre(5_u8));
    println!("distance {}", euclidean_distance(pixels.get(0..3).unwrap(),pixels.get(3..6).unwrap()));
    println!("p1: {:?}, p2: {:?}", pixels.get(0..3).unwrap(), pixels.get(3..6).unwrap());
    println!("centroid: {:?}", centroid( pixels.get(0..6).unwrap()));

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
