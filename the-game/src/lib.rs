mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

pub struct ImageData {
    width: usize,
    height: usize,
    data: Vec<u8>
}

pub enum ImageError {
    OutOfBounds
}

impl ImageData {

    pub fn new(width: usize, height: usize) -> Self {
        ImageData { width, height, data: vec![250; width * height * 4]}
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: &Pixel) -> Result<(), ImageError> {
        
        log(&format!("{}", y * self.width * 4 + x));
        if x > self.width || y > self.height {
            return Err(ImageError::OutOfBounds);
        }
        
        self.data[(x * self.height  + y) * 4] = pixel.r;
        self.data[(x * self.height  + y) * 4 + 1] = pixel.g;
        self.data[(x * self.height  + y) * 4 + 2] = pixel.b;
        self.data[(x * self.height  + y) * 4 + 3] = pixel.a;

        Ok(())
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

}
#[wasm_bindgen]
pub fn draw_image_data(w: u32, h: u32) -> *const u8 {
    
    let mut image = ImageData::new(w as usize, h as usize);

    for index in 0..10 {
        for yindex in 0..100 {
            image.set_pixel(index, yindex, &Pixel { r: 250, g:0, b:0, a:255}).ok(); 
        }
    }
    image.as_ptr() 
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, the-game!");
}
