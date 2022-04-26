/*
    The lowlevel::input module aims to gather any relavent forms
    of input and make them usable for the application.

    it intends to do so in a crossplatform manner while making it
    easy for the dev by abstracting the low level inputs into 
    common actions, such as 'select' and 'confirm'
*/

use std::io;

// different types of input the device might express
enum ActionKind{
    Select,
    Confirm,
    Move(u16, u16)
}

// gets string input from console
pub fn str_console() -> String{
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Error");
    response
}

// shows prompt then calls str_console
pub fn str_console_prompt(prompt: &str) -> String{
    println!("{}", prompt);
    str_console()
}

// the InputHandler deals with more advanced input (keyboard, gamepad, etc.) and can listen to multiple input devices
struct InputHandler{
    devices: Vec<u8>
    
}
