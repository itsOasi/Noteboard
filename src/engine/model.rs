use super::view::{self, context::Context, draw::Visual};

pub struct Model{
    name: String,
    width: usize,
    height: usize,
    clear_color: (u8, u8, u8),
    visuals: Vec<Visual>,
}

impl Model {
    pub fn new(name: String, width: usize, height: usize, clear_color:Option<(u8, u8, u8)> )->Model{
        Model{
            name, 
            width, 
            height, 
            clear_color: clear_color.unwrap_or((0,0,0)), 
            visuals: Vec::new()
        }
    }

    // continuously iterates over data and changes state accordingly
    pub fn run(&mut self){
        let context = Context::new(&self.name, self.width, self.height, self.clear_color);
        let width = context.width;
        let height = context.height;
        let color = context.clear_color;

        let mut window = context.window();
        while window.is_open(){
            view::context::update(&mut window, &mut self.visuals, width, height, None);
        }
    }

    pub fn add_visual(&mut self, vis: Visual){
        self.visuals.push(vis);
    }
}