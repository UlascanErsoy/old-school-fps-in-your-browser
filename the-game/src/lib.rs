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

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageData {
    width: usize,
    height: usize,
    data: Vec<u8>
}

#[repr(u8)]
pub enum ImageError {
    OutOfBounds
}

#[wasm_bindgen]
impl ImageData {

    pub fn new(width: usize, height: usize) -> ImageData {
        ImageData { width, height, data: vec![255; width * height * 4]}
    }

    fn set_pixel(&mut self, x: usize, y: usize, pixel: &Pixel) -> Result<(), ImageError> {
        
        //log(&format!("{}", y * self.width * 4 + x));
        if x > self.width || y > self.height {
            return Err(ImageError::OutOfBounds);
        }
        
        self.data[(y * self.width + x) * 4] = pixel.r;
        self.data[(y * self.width + x) * 4 + 1] = pixel.g;
        self.data[(y * self.width + x) * 4 + 2] = pixel.b;
        self.data[(y * self.width + x) * 4 + 3] = pixel.a;

        Ok(())
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub fn draw_test_image(&mut self, x: u32, y: u32) -> *const u8 {

        self.data = vec![255; self.width * self.height * 4];
        for index in x..x+30 {
            for yindex in y..y+30 {
                self.set_pixel(index as usize,
                               yindex as usize,
                               &Pixel { r: (250  * yindex as usize / self.height) as u8,
                                        g: (250  * index as usize/ self.width) as u8,
                                        b: 0, 
                                        a: 255
                                    }).ok(); 
            }
        }

        self.as_ptr() 
    }
}

#[wasm_bindgen]
pub fn draw_image_data(w: u32, h: u32) -> *const u8 {
    
    let mut image = ImageData::new(w as usize, h as usize);

    for index in 0..w {
        for yindex in 0..h {
            image.set_pixel(index as usize, yindex as usize, &Pixel { r:255 , g:255, b:0, a:255}).ok(); 
        }
    }
    image.as_ptr() 
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, the-game!");
}
