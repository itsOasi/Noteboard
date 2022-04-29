mod engine;
use std::collections::HashMap;
use engine::{control, model::Model};

fn main() {
    let name = control::console::prompt("enter name: ");
    let width = control::console::prompt("enter width: ").trim().parse::<usize>().unwrap();
    let height = control::console::prompt("enter height: ").trim().parse::<usize>().unwrap();

    let clear_color = (0, 0, 0);
    let mut model = Model::new(name, width, height, clear_color);
    model.run();
    
}