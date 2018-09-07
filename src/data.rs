use std::f32;
use std::collections::VecDeque;

pub struct Pixel{
    pub base_colors : (f32, f32, f32),
}

pub struct CentroidPixel{
	pub p: Pixel,
	pub sum: (f32, f32, f32),
	pub count: u32,
}

pub struct KMeans{
	pub pixels: VecDeque<ColorPixel>, 
	pub centroids: VecDeque<CentroidPixel>,
}

pub struct ColorPixel {
    pub p: Pixel,
    pub count: u32,
}


trait Get {
    fn get_pixel(&mut self) -> &Pixel;
}

impl Get for CentroidPixel {
    fn get_pixel(&mut self) -> &Pixel{
        &self.p
    }
}

impl Get for ColorPixel {
    fn get_pixel(&mut self) -> &Pixel{
        &self.p
    }
}