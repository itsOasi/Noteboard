mod engine;
use engine::{control, model::Model, view::draw};

fn main() {
    let name = control::terminal::prompt("enter name: ");
    let width = control::terminal::prompt("enter width: ").trim().parse::<usize>().unwrap();
    let height = control::terminal::prompt("enter height: ").trim().parse::<usize>().unwrap();

    // creates model
    let mut model = Model::new(name, width, height, None);
    
    // creates visual and adds it to model
    let vis = draw::Visual::new(draw::DrawMode::Rect(40, 40), (50, 50), (200, 50, 50));
    model.add_visual(vis);
    
    // runs model
    model.run();
    
}