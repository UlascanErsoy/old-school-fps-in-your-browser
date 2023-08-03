mod utils;

use wasm_bindgen::prelude::*;
use std::f32::consts::PI;

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

const MAP_W: usize = 16;
const MAP_H: usize = 16;
const MAP: [u8;256] = [1,1,1,1,2,2,2,2,2,2,2,2,1,1,1,1,
                       1,0,0,3,0,0,0,0,0,2,0,0,0,0,0,1,
                       1,0,0,3,0,0,0,0,0,2,0,0,0,0,0,1,
                       1,0,0,3,0,0,0,0,0,2,0,0,0,0,0,1,
                       1,0,0,3,0,0,0,0,0,2,0,0,0,0,0,1,
                       1,0,0,3,0,0,0,0,0,2,0,0,0,0,0,1,
                       1,0,0,3,0,0,0,0,0,2,0,0,0,0,0,1,
                       1,0,0,3,1,1,1,0,0,2,0,0,0,0,0,1,
                       1,0,0,0,0,0,0,0,0,2,0,0,0,0,0,1,
                       1,0,0,0,0,0,0,0,0,2,0,0,0,0,0,1,
                       1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                       1,1,1,1,1,1,1,1,1,1,1,0,0,1,1,1,
                       1,0,0,0,0,0,0,0,0,0,2,0,0,0,0,1,
                       1,0,0,0,0,0,0,0,0,0,2,0,0,0,0,1,
                       1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                       1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];
                   
#[wasm_bindgen]
impl ImageData {

    pub fn new(width: usize, height: usize) -> ImageData {
        ImageData { width, height, data: vec![255; width * height * 4]}
    }

    fn clear(&mut self) {
        self.data = vec![255; self.width * self.height * 4];
    }

    fn set_pixel(&mut self, x: usize, y: usize, pixel: &Pixel) -> Result<(), ImageError> {
        
        if x > self.width || y > self.height {
            return Err(ImageError::OutOfBounds);
        }
        
        self.data[(y * self.width + x) * 4] = pixel.r;
        self.data[(y * self.width + x) * 4 + 1] = pixel.g;
        self.data[(y * self.width + x) * 4 + 2] = pixel.b;
        self.data[(y * self.width + x) * 4 + 3] = pixel.a;

        Ok(())
    }

    fn draw_line(&mut self, x: usize, y: usize,
                            x2: usize, y2:usize,
                            pixel: &Pixel) -> Result<(), ImageError> {

       if x > self.width || x2 > self.width || 
           y2 > self.height || y > self.height {
            return Err(ImageError::OutOfBounds);
       }
        
       let ((smallx,smally) , (bigx,bigy)) = if x2 > x { ((x,y),(x2,y2)) } else { ((x2,y2),(x,y)) };
       
       let delta_x  = bigx as f32 - smallx as f32;
       let delta_y  = bigy as f32 - smally as f32;

       let line_len = f32::sqrt(delta_x * delta_x + delta_y * delta_y);
       let steps = (line_len / 0.5).ceil() as usize;
        
       let mut curx = smallx as f32;
       let mut cury = smally as f32;

       for _ in 0..=steps {
           log(&format!("{delta_x} {delta_y} {line_len} {bigy} {smally} {steps} {}", bigy - smally));
           curx = curx + 0.5 * delta_x / line_len;
           cury = cury + 0.5 * delta_y / line_len;

           self.set_pixel(curx as usize, cury as usize, pixel).ok();
       }

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
#[derive(Clone,Copy)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32
}

impl Player {

    fn look_at(&self) -> Option<(f32, f32)> {
        let mut c = 0_f32;
        while c <= 20_f32 {
            let x = self.x + c * (self.angle * 2_f32 * PI).cos();
            let y = self.y + c * (self.angle * 2_f32 * PI).sin();
            
            if MAP[y as usize * MAP_W + x as usize] != 0 {
                return Some((x , y));
            }
            c += 0.1;
        }

        None
    }

}

#[wasm_bindgen]
pub struct Game {
    image_buffer: ImageData,
    pub player: Player 
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game { 
            image_buffer: ImageData::new(320, 240),
            player: Player { x: 2_f32, y: 3_f32, angle: 0_f32 }
        }
    }

    fn draw_hud(&mut self) {
        let block_size = 4;
        for idx in 0..block_size*MAP_W {
            for ydx in 0..block_size*MAP_H {
                let id = (ydx / 4) * MAP_W + idx / 4;
                if MAP[id] != 0 {
                    self.image_buffer.set_pixel(idx, ydx, &Pixel { r: 0, g:0, b:0 ,a:255}).ok();
                }else{
                    self.image_buffer.set_pixel(idx, ydx, &Pixel { r: 50, g:50, b:150 ,a:255}).ok();
                }
            }
        }
        
        if let Some((lx, ly)) = self.player.look_at() {
            self.image_buffer
                .set_pixel(lx as usize * block_size,
                           ly as usize * block_size,
                           &Pixel {r: 0, g:255, b:0, a:255})
                .ok();
            log(&format!("{}x{} -> {},{}",
                           self.player.x as usize * block_size, 
                           self.player.y as usize * block_size, 
                           lx as usize * block_size, 
                           ly as usize * block_size
                           ));

            self.image_buffer
                .draw_line(self.player.x as usize * block_size, 
                           self.player.y as usize * block_size, 
                           lx as usize * block_size, 
                           ly as usize * block_size,
                           &Pixel {r: 0, g:255, b:0, a:255}).ok();

        }

        self.image_buffer
            .set_pixel(self.player.x as usize * block_size,
                       self.player.y as usize * block_size,
                       &Pixel {r: 255, g:0, b:0, a:255})
            .ok();

    }
    
    

    pub fn render(&mut self) -> *const u8 {
        self.image_buffer.clear();
        self.draw_hud();

        self.player.angle += 0.01;

        self.image_buffer.as_ptr()
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
