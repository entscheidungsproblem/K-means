use data::CentroidPixel as CentroidPixel;
use palette::{Lch,LabHue,IntoColor,rgb};
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;

fn export_file(contents: String, location: &str){
    let mut writer = File::create(location).unwrap();
    let success = writer.write_all(&contents.into_bytes() );
    if success.is_err(){
        eprintln!("Error writing: {}", location);
    }
}

pub fn export_json(centroids: &VecDeque<CentroidPixel>, full_path: &str, out_location: &str){
    let mut output = Vec::new();
    writeln!(&mut output, "{{ \n\t\"wallpaper\":\t\"{}\",\n\t\"colors\":\t{{", full_path);
    
    for (counter, centroid) in centroids.iter().enumerate(){
        let _lch = centroid.p.base_colors;
        let lch: Lch = Lch::new(_lch.0, _lch.1, LabHue::from(_lch.2));
        let rgb_color: rgb::Rgb<rgb::Linear> = lch.into_rgb();
        if counter != 0 {writeln!(&mut output, ",");}
        write!(&mut output, "\t\t\"color{}\":\t\"#{:0width$X}{:0width$X}{:0width$X}\"", counter, (rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8, width=2);
    }
    write!(&mut output, "\n\t}}\n}}");
    export_file(String::from_utf8(output).unwrap(), out_location);
}

pub fn export_yaml(centroids: &VecDeque<CentroidPixel>, full_path: &str, out_location: &str){
    let mut output = Vec::new();
    writeln!(&mut output, "wallpaper: \"{}\"", full_path);
    writeln!(&mut output, "colors:");
    
    for (i, c) in centroids.iter().enumerate(){
        let _lch = c.p.base_colors;
        let lch: Lch = Lch::new(_lch.0, _lch.1, LabHue::from(_lch.2));
        let rgb_color: rgb::Rgb<rgb::Linear> = lch.into_rgb();
        writeln!(&mut output, "\tcolor{}: \"#{:0width$X}{:0width$X}{:0width$X}\"", i, (rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8, width=2);
    }   
    export_file(String::from_utf8(output).unwrap(), out_location);
}

pub fn export_sh(centroids: &VecDeque<CentroidPixel>, full_path: &str, out_location: &str){
    let mut output = Vec::new();
    writeln!(&mut output, "wallpaper=\'{}\'", full_path);
    
    for (i, c) in centroids.iter().enumerate(){
        let _lch = c.p.base_colors;
        let lch: Lch = Lch::new(_lch.0, _lch.1, LabHue::from(_lch.2));
        let rgb_color: rgb::Rgb<rgb::Linear> = lch.into_rgb();
        writeln!(&mut output, "color{}=\'#{:0width$X}{:0width$X}{:0width$X}\'", i, (rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8, width=2);
    }   
    export_file(String::from_utf8(output).unwrap(), out_location);
}

pub fn export_css(centroids: &VecDeque<CentroidPixel>, full_path: &str, out_location: &str){
    let mut output = Vec::new();
    writeln!(&mut output, ":root {{");
    writeln!(&mut output, "\t--wallpaper: url(\"{}\");", full_path);
    
    for (i, c) in centroids.iter().enumerate(){
        let _lch = c.p.base_colors;
        let lch: Lch = Lch::new(_lch.0, _lch.1, LabHue::from(_lch.2));
        let rgb_color: rgb::Rgb<rgb::Linear> = lch.into_rgb();
        writeln!(&mut output, "\t--color{}: #{:0width$X}{:0width$X}{:0width$X};", i, (rgb_color.red * 255.0) as u8, (rgb_color.green * 255.0) as u8, (rgb_color.blue * 255.0) as u8, width=2);
    }
    writeln!(&mut output, "}}");
    export_file(String::from_utf8(output).unwrap(), out_location);
}