#[cfg_attr(feature = "cargo-clippy", allow(needless_lifetimes))]
#[macro_use]

use data::Pixel as Pixel;
use data::CentroidPixel as CentroidPixel;

use clap::{Arg, App, SubCommand};

use palette::{Lch,LabHue,IntoColor,Srgb,rgb};

use std::f32;
use std::collections::VecDeque;


pub fn export_json(centroids: VecDeque<CentroidPixel>, full_path: String){
    print!("{{ \n\t\"wallpaper\":\t\"{}\",\n\t\"colors\":\t{{\n", full_path);
    
    let mut i = 0;
    for c in centroids{
        let _lch = c.p.base_colors;
        let lch: Lch = Lch::new(_lch.0, _lch.1, LabHue::from(_lch.2));
        let rgb_color: rgb::Rgb<rgb::Linear> = lch.into_rgb();
        //println!("{:?}", ((rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8));
        if (i != 0){println!(",");}
        print!("\t\t\"color{}\":\t\"#{:0width$X}{:0width$X}{:0width$X}\"", i, (rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8, width=2);
        
        i+=1;
    }
    print!("\n\t}}\n}}");
    
}

pub fn export_yaml(centroids: VecDeque<CentroidPixel>, full_path: String){
    println!("wallpaper: {}", full_path);
    println!("colors:");
    
    let mut i = 0;
    for c in centroids{
        let _lch = c.p.base_colors;
        let lch: Lch = Lch::new(_lch.0, _lch.1, LabHue::from(_lch.2));
        let rgb_color: rgb::Rgb<rgb::Linear> = lch.into_rgb();
        //println!("{:?}", ((rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8));
        println!("\tcolor{}: #{:0width$X}{:0width$X}{:0width$X}", i, (rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8, width=2);
        
        i+=1;
    }   
}

pub fn export_sh(centroids: VecDeque<CentroidPixel>, full_path: String){
    println!("wallpaper: \'{}\'", full_path);
    
    let mut i = 0;
    for c in centroids{
        let _lch = c.p.base_colors;
        let lch: Lch = Lch::new(_lch.0, _lch.1, LabHue::from(_lch.2));
        let rgb_color: rgb::Rgb<rgb::Linear> = lch.into_rgb();
        //println!("{:?}", ((rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8));
        println!("color{}=\'#{:0width$X}{:0width$X}{:0width$X}\'", i, (rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8, width=2);
        
        i+=1;
    }   
}