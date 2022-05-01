/*
    the engine::view module focuses on providing human readable representation of model data 
*/

use minifb::{Window, WindowOptions};
use rasterize;

pub mod context{
    // view::context provides a window for the application to display and manages its pixel buffer   
    use super::{Window, WindowOptions, draw::{self, Visual, DrawMode}, pixels::{self, clear_buf}};
    pub struct Context{
        pub name: String,
        pub clear_color: (u8, u8, u8),
        pub width: usize,
        pub height: usize,
        
        window: Window,
    }

    impl Context{
        // returns a new context for drawing to the screen
        pub fn new(name: &str, width: usize, height: usize, clear_color:(u8, u8, u8) )->Context{
            println!("initializing {name}");
            let window: Window = Window::new(name, width, height, WindowOptions::default()).unwrap();
            let string_name = String::from(name);
            Context { name:string_name, window, width, height, clear_color}
        }
        // returns the window attached to the context
        pub fn window(self)->Window{
            self.window
        }
        // checks to see if context window is open
        pub fn is_open(self)->bool{
            self.window().is_open()
        }
        // checks to see if context window is active
        pub fn is_active(self)->bool{
            self.window().is_active()
        }
    }
    
    pub fn update(window: &mut Window, visuals: &mut [Visual], width: usize, height: usize, clear_color: Option<(u8, u8, u8)>){
        let buffer = process(visuals, width, height, clear_color.unwrap_or((0, 0, 0)));
        window.update_with_buffer(&buffer, width, height).unwrap();
    }

    pub fn create_buffer(width:usize, height:usize, clear_color: (u8, u8, u8))->Vec<u32>{
        clear_buf(width, height, clear_color)
    }

    pub fn process(visuals: &mut [Visual], width: usize, height: usize, clear_color: (u8, u8, u8)) -> Vec<u32>{
        let mut buffer = create_buffer(width, height, clear_color);
        let len_vis = visuals.len();
        print!("{len_vis}");
        for vis in visuals{
            match vis.mode {
                DrawMode::Rect(width, height) => {
                    draw::rect(&mut buffer, vis.position, width, height, vis.color)
                }
                DrawMode::Text(_, _) => {
                    draw::text()
                }
            }
        }
        buffer
    }
}

pub mod pixels{
    /////////////////Pixel functions///////////////////////////////
    ////useful for creating fragment shaders///////////////////////
    // takes u8 values for each color channel and converts it into a format usable by the context buffer
    pub fn color_to_buf(color: (u8, u8, u8)) -> u32 {
        let (r, g, b) = (color.0 as u32, color.1 as u32, color.2 as u32);
        (r << 16) | (g << 8) | b
    }
    
    // updates a singular pixel in the context buffer
    pub fn set_buffer_pixel(buffer: &mut [u32], pos: (usize, usize), color: (u8, u8, u8)){
        buffer[pos.0 * pos.1] = color_to_buf(color);
    }

    // gets the color from a pixel
    pub fn get_buffer_pixel(buffer: &mut [u32], pos: (usize, usize))->(u8, u8, u8){
        let color_int = buffer[pos.0 * pos.1];
        let channels = color_int.to_ne_bytes();
        print!("{:?}", channels);
        (channels[0], channels[1], channels[2])
    }

    // sets all pixels in the context buffer to the supplied color
    pub fn clear_buf(width: usize, height: usize, color: (u8, u8, u8)) -> Vec<u32>{
        vec![color_to_buf(color); width * height]

    }
}

pub mod draw{
    // the draw module is a wrapper for the pixels module   
    use super::pixels::{set_buffer_pixel};

    pub enum DrawMode{
        Rect(usize, usize),
        Text(usize, String)
    }

    pub struct Visual{
        pub mode: DrawMode,
        pub position: (usize, usize),
        pub color: (u8, u8, u8),
    }

    impl Visual {
        pub fn new(mode:  DrawMode, position: (usize, usize), color: (u8, u8, u8))->Visual{
            Visual {mode, position, color}
        }
    }

    // rect draws a rectangle with a given height, width and color starting at a given upper left position
    pub fn rect(buffer: &mut [u32], upper_left: (usize, usize), width: usize, height: usize, color: (u8, u8, u8)){
        let mut pos = upper_left;
        while pos.0 < width {
            while pos.1 < height {
                set_buffer_pixel(buffer, pos, color);
                pos.1 += 1;
            }
            pos.0 += 1;
        }
    }

    // elipse draws an eliptical shape starting at the center and extending to the given radii
    pub fn elipse(buffer: &mut [u32], center: (usize, usize), r1: usize, r2: usize, color: (u8, u8, u8)){
        let mut pos = center;
        set_buffer_pixel(buffer, pos, color);
    }

    // draws text to the screen
    // pub fn text(buffer: &mut Vec<u32>, content: String, size: usize, pos: (usize, usize), color: (u8, u8, u8)){
    pub fn text(){
        print!("text not yet implemented");
    }
}