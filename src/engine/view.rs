use minifb::{Window, WindowOptions};
use rasterize;

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
    pub fn update(window: &mut Window, buffer: &Vec<u32>, width: usize, height: usize){
        window.update_with_buffer(buffer, width, height).unwrap();
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
    pub fn set_buffer_pixel(buffer: &mut Vec<u32>, pos: (usize, usize), color: (u8, u8, u8)){
        buffer[pos.0 * pos.1] = color_to_buf(color);
    }

    // gets the color from a pixel
    pub fn get_buffer_pixel(buffer: &mut Vec<u32>, pos: (usize, usize))->(u8, u8, u8){
        let color_int = buffer[pos.0 * pos.1];
        let r: u8 = ((color_int >> 16) & 0xff) as u8;
        let g = ((color_int >> 8) & 0xff) as u8;
        let b = ((color_int) & 0xff) as u8;
        (r, g, b)
    }

    // sets all pixels in the context buffer to the supplied color
    pub fn clear_buf(color: (u8, u8, u8), width: usize, height: usize)->Vec<u32>{
        vec![color_to_buf(color); width * height]
    }
}
