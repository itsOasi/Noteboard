use super::view::{self, Context};

pub struct Model{
    name: String,
    width: usize,
    height: usize,
    clear_color: (u8, u8, u8),
    buffer: Vec<u32>
}

impl Model {
    pub fn new(name: String, width: usize, height: usize, clear_color:(u8, u8, u8) )->Model{
        
        Model{name, width, height, clear_color, buffer: vec![view::pixels::color_to_buf(clear_color); width * height]}
    }

    // continuously iterates over data and changes state accordingly
    pub fn run(&mut self){
        let context = Context::new(&self.name, self.width, self.height, self.clear_color);
        let width = context.width;
        let height = context.height;
        let color = context.clear_color;

        let mut window = context.window();
        while window.is_open(){
            self.buffer = view::pixels::clear_buf(color, width, height);
            view::Context::update(&mut window, &self.buffer, width, height);
        }    
    }
}