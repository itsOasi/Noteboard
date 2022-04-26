use std::collections::HashMap;

use minifb::{Window, WindowOptions};
use ras
mod input;

struct Context{
    name: String,
    window: Window,
    buf: Vec<u32>,
    clear_color: (u8, u8, u8),
    width: usize,
    height: usize,
}


impl Context{
    pub fn new(name: String, width: usize, height: usize, clear_color:(u8, u8, u8) )->Context{
        println!("initializing {name}");
        let window: Window = Window::new(name.trim(), width, height, WindowOptions::default()).unwrap();

        Context { name, window, buf: vec![Context::color_to_buf(clear_color); width * height], width, height, clear_color}
    }

    // takes u8 values for each color channel and converts it into a format usable by the buffer
    fn color_to_buf(color: (u8, u8, u8)) -> u32 {
        let (r, g, b) = (color.0 as u32, color.1 as u32, color.2 as u32);
        (r << 16) | (g << 8) | b
    }

    // updates a singular pixel in the buffer
    pub fn set_buf_pixel(&mut self, color: (u8, u8, u8), pos: (usize, usize)){
        self.buf[pos.0 * pos.1] = Context::color_to_buf(color);
    }

    // sets all pixels in the buffer to the clear color
    pub fn clear_buf(&mut self){
        self.buf = vec![Context::color_to_buf(self.clear_color); self.width * self.height]
    }
}

fn main() {
    let name = input::str_console_prompt("enter name: ");
    let width = input::str_console_prompt("enter width: ").trim().parse::<usize>().unwrap();
    let height = input::str_console_prompt("enter height: ").trim().parse::<usize>().unwrap();
    let cname = input::str_console_prompt("enter color name: ");

    let color: HashMap<&str, (u8, u8, u8)> =  HashMap::from([
        ("black",(0, 0, 0)),
        ("red",(255, 0, 0)),
        ("orange",(255, 127, 0)),
        ("yellow", (255, 255, 0)),
        ("citrus", (127, 255, 0)),
        ("green", (0, 255, 0)),
        ("teal", (0, 255, 127)),
        ("cyan", (0, 255, 255)),
        ("teal", (0, 127, 255)),
        ("blue", (0, 0, 255)),
        ("blue-violet", (127, 0, 255)),
        ("magenta", (255, 0, 255)),
        ("red-violet", (255, 0, 127)),
        ("white", (255, 255, 255)),
    ]);

    let mut ctx = Context::new(name, width, height, color[cname.trim()]);
    
    while ctx.window.is_open(){
        ctx.window.update_with_buffer(&ctx.buf, ctx.width, ctx.height).unwrap();
    }
}
