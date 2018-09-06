#[cfg_attr(feature = "cargo-clippy", allow(needless_lifetimes))]
use data::CentroidPixel as CentroidPixel;
use palette::{Lch,LabHue,IntoColor,rgb};
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;

fn export_file(contents: String, location: String){
    let mut writer = File::create(location.clone()).unwrap();
    let success = writer.write_all(&contents.into_bytes() );
    if success.is_err(){
        eprintln!("Error writing: {}", location);
    }
}

pub fn export_json(centroids: &VecDeque<CentroidPixel>, full_path: String, out_location: String){
    let mut output = Vec::new();
    write!(&mut output, "{{ \n\t\"wallpaper\":\t\"{}\",\n\t\"colors\":\t{{\n", full_path);
    
    let mut i = 0;
    for c in centroids{
        let _lch = c.p.base_colors;
        let lch: Lch = Lch::new(_lch.0, _lch.1, LabHue::from(_lch.2));
        let rgb_color: rgb::Rgb<rgb::Linear> = lch.into_rgb();
        //println!("{:?}", ((rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8));
        if i != 0 {write!(&mut output, ",\n");}
        write!(&mut output, "\t\t\"color{}\":\t\"#{:0width$X}{:0width$X}{:0width$X}\"", i, (rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8, width=2);
        
        i+=1;
    }
    write!(&mut output, "\n\t}}\n}}");
    export_file(String::from_utf8(output).unwrap(), out_location);
}

pub fn export_yaml(centroids: &VecDeque<CentroidPixel>, full_path: String, out_location: String){
    let mut output = Vec::new();
    write!(&mut output, "wallpaper: \"{}\"\n", full_path);
    write!(&mut output, "colors:\n");
    
    let mut i = 0;
    for c in centroids{
        let _lch = c.p.base_colors;
        let lch: Lch = Lch::new(_lch.0, _lch.1, LabHue::from(_lch.2));
        let rgb_color: rgb::Rgb<rgb::Linear> = lch.into_rgb();
        //println!("{:?}", ((rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8));
        write!(&mut output, "\tcolor{}: \"#{:0width$X}{:0width$X}{:0width$X}\"\n", i, (rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8, width=2);
        
        i+=1;
    }   
    export_file(String::from_utf8(output).unwrap(), out_location);
}

pub fn export_sh(centroids: &VecDeque<CentroidPixel>, full_path: String, out_location: String){
    let mut output = Vec::new();
    write!(&mut output, "wallpaper=\'{}\'\n", full_path);
    
    let mut i = 0;
    for c in centroids{
        let _lch = c.p.base_colors;
        let lch: Lch = Lch::new(_lch.0, _lch.1, LabHue::from(_lch.2));
        let rgb_color: rgb::Rgb<rgb::Linear> = lch.into_rgb();
        //println!("{:?}", ((rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8));
        write!(&mut output, "color{}=\'#{:0width$X}{:0width$X}{:0width$X}\'\n", i, (rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8, width=2);
        
        i+=1;
    }   
    export_file(String::from_utf8(output).unwrap(), out_location);
}

pub fn export_css(centroids: &VecDeque<CentroidPixel>, full_path: String, out_location: String){
    let mut output = Vec::new();
    write!(&mut output, ":root {{\n");
    write!(&mut output, "\t--wallpaper: url(\"{}\");\n", full_path);
    
    let mut i = 0;
    for c in centroids{
        let _lch = c.p.base_colors;
        let lch: Lch = Lch::new(_lch.0, _lch.1, LabHue::from(_lch.2));
        let rgb_color: rgb::Rgb<rgb::Linear> = lch.into_rgb();
        //println!("{:?}", ((rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8));
        write!(&mut output, "\t--color{}: #{:0width$X}{:0width$X}{:0width$X};\n", i, (rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8, width=2);
        
        i+=1;
    }
    write!(&mut output, "}}\n");
    export_file(String::from_utf8(output).unwrap(), out_location);
}