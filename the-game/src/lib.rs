mod utils;

use wasm_bindgen::prelude::*;
use std::f32::consts::PI;

const TEXTURES: [u8; 73866] = *include_bytes!("../textures.bmp");
const TEXTURE_HEIGHT: usize = 64;
const TEXTURE_WIDTH: usize = 64;

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

pub struct RayHit {
    x: f32,
    y: f32,
    distance: f32,
    wall_type: u8
}

#[repr(u8)]
pub enum ImageError {
    OutOfBounds
}


const WALL_HEIGHT: f32 = 150_f32;
const RAY_COUNT: usize = 240;
const FOV: f32 = 75.0;
const MAP_W: usize = 16;
const MAP_H: usize = 16;
const MAP: [u8;256] = [1,1,1,1,1,1,1,1,1,1,2,2,1,1,1,1,
                       1,0,0,5,0,0,0,0,0,2,0,0,0,0,0,1,
                       1,0,0,4,0,0,0,0,0,2,0,0,0,0,0,1,
                       1,0,0,5,0,0,0,0,0,2,0,0,0,0,0,1,
                       1,0,0,5,0,0,0,0,0,2,0,0,0,0,0,1,
                       1,0,0,5,0,0,0,0,0,2,0,0,0,0,0,1,
                       1,0,0,5,0,0,0,0,0,2,0,0,0,0,0,1,
                       1,0,0,5,1,6,1,0,0,2,0,0,0,0,0,1,
                       1,0,0,0,0,0,0,0,0,2,0,0,2,2,0,1,
                       1,0,0,0,0,0,0,0,0,2,0,0,1,0,0,1,
                       1,0,0,0,0,0,0,0,0,0,0,0,1,0,0,1,
                       1,1,1,1,1,1,1,1,1,1,0,0,1,1,1,1,
                       1,0,0,0,0,0,0,0,0,0,2,0,0,0,0,1,
                       1,0,0,0,0,0,0,0,0,0,2,0,0,0,0,1,
                       1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
                       1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];


extern crate bmp;
use bmp::Image;

#[wasm_bindgen]
impl ImageData {

    pub fn new(width: usize, height: usize) -> ImageData {
        let mut textures = bmp::from_reader(&mut TEXTURES.as_slice());
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
           curx = curx + 0.5 * delta_x / line_len;
           cury = cury + 0.5 * delta_y / line_len;

           self.set_pixel(curx as usize, cury as usize, pixel).ok();
       }

       Ok(())

    }

    fn draw_line_texture(&mut self, x: usize, y: usize,
                            x2: usize, y2:usize,
                            image: &Image, text_x: usize) -> Result<(), ImageError> {

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

       for step in 0..=steps {
           curx = curx + 0.5 * delta_x / line_len;
           cury = cury + 0.5 * delta_y / line_len;

           let text_y = ((TEXTURE_HEIGHT - 1) as f32 - ((step as f32 / steps as f32) 
                        * (TEXTURE_HEIGHT - 1) as f32)) as usize;


           let px = image.get_pixel(text_x as u32,text_y as u32);
           self.set_pixel(curx as usize, cury as usize, &Pixel { r: px.r, g: px.g, b:px.b, a:255 }).ok();
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

#[wasm_bindgen]
impl Player {

    fn look_at(&self, offset: Option<f32>) -> Option<RayHit> {
        let mut c = 0_f32;
        let angle = match offset {
            Some(off) => self.angle + off,
            None => self.angle
        };
        while c <= 20_f32 {
            let x = self.x + c * (angle * PI / 180.0).cos();
            let y = self.y + c * (angle * PI / 180.0).sin();
            
            if MAP[y as usize * MAP_W + x as usize] != 0 {
                return Some(RayHit { x,y, 
                                    distance:c,
                                    wall_type:MAP[y as usize * MAP_W + x as usize]}
                                    );
            }
            c += 0.1;
        }

        None
    }


}


#[wasm_bindgen]
pub struct Game {
    textures: Image,
    image_buffer: ImageData,
    pub player: Player 
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game { 
            textures: bmp::from_reader(&mut TEXTURES.as_slice()).unwrap(),
            image_buffer: ImageData::new(320, 240),
            player: Player { x: 7_f32, y: 3_f32, angle: 30_f32 }
        }
    }

    pub fn update_player(&mut self, speed: f32, angle: f32, sideways: bool) {
            self.player.angle += angle;
            if sideways {
                self.player.x += speed * (self.player.angle * PI / 180.0 ).sin();
                self.player.y += speed * (self.player.angle * PI / 180.0 ).cos();
            } else {
                self.player.x += speed * (self.player.angle * PI / 180.0 ).cos();
                self.player.y += speed * (self.player.angle * PI / 180.0 ).sin();

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
        
        if let Some(ray) = self.player.look_at(Some(-30.0)) {

            self.image_buffer
                .draw_line(self.player.x as usize * block_size, 
                           self.player.y as usize * block_size, 
                           ray.x as usize * block_size, 
                           ray.y as usize * block_size,
                           &Pixel {r: 0, g:255, b:0, a:255}).ok();

        }
        if let Some(ray) = self.player.look_at(Some(30.0)) {

            self.image_buffer
                .draw_line(self.player.x as usize * block_size, 
                           self.player.y as usize * block_size, 
                           ray.x as usize * block_size, 
                           ray.y as usize * block_size,
                           &Pixel {r: 0, g:255, b:255, a:255}).ok();

        }

        self.image_buffer
            .set_pixel(self.player.x as usize * block_size,
                       self.player.y as usize * block_size,
                       &Pixel {r: 255, g:0, b:0, a:255})
            .ok();

    }

    fn render_view(&mut self) {
       let step_delta = FOV / self.image_buffer.width as f32; 
       let hmid = (self.image_buffer.height / 2) as f32;

       for ray_index in 0..self.image_buffer.width {
           let offset = -FOV / 2.0 + ray_index as f32 * step_delta;
           match self.player.look_at(Some(offset)) {
                Some(ray) => {
                    let height = WALL_HEIGHT / (ray.distance * (offset * PI / 180.0).cos());
                    let tex_x = ray.x - (f32::floor(ray.x + 0.5) as i32) as f32;
                    let tex_y = ray.y - (f32::floor(ray.y + 0.5) as i32) as f32;

                    let tex_vert = if f32::abs(tex_y) <= f32::abs(tex_x) { tex_x } else { tex_y };
                    let mut tex_coord= (TEXTURE_WIDTH as f32 * tex_vert) as i32;
                    
                    tex_coord = if tex_coord < 0 { tex_coord + TEXTURE_WIDTH as i32 } else { tex_coord };
                    tex_coord += (ray.wall_type - 1) as i32 * TEXTURE_WIDTH as i32;

                    self.image_buffer
                        .draw_line_texture(ray_index, (hmid - height / 2.0) as usize,
                                   ray_index, (hmid + height / 2.0) as usize,
                                   &self.textures, tex_coord as usize 
                                   ).ok();
                },
                None => ()
           };
       }
    }

    fn render_sky(&mut self) {

        for x in 0..self.image_buffer.width {
            for y in 0..self.image_buffer.height {
                if y < self.image_buffer.height / 2 {
                    self.image_buffer.set_pixel(x,y, &Pixel { r: 50, g:50 , b:180, a:255}).ok();
                }else{
                    self.image_buffer.set_pixel(x,y, &Pixel { r: 50, g:50, b:40, a:255}).ok();

                }
            }
        }
    }

    pub fn render(&mut self) -> *const u8 {
        self.image_buffer.clear();
        self.render_sky();
        self.render_view();
        self.draw_hud();
        self.image_buffer.as_ptr()
    }
}
