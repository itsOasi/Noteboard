/*
    The lowlevel::input module aims to gather any relavent forms
    of input and make them usable for the application.

    it intends to do so in a crossplatform manner while making it
    easy for the dev by abstracting the low level inputs into 
    common actions, such as 'select' and 'confirm'
*/


pub mod console{    
    use std::io;
    // gets string input from console
    pub fn get_line() -> String{
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Error");
        response
    }

    // shows prompt then calls str_console
    pub fn prompt(prompt: &str) -> String{
        println!("{}", prompt);
        get_line()
    }
}
// Controller represents a physical device that the user might use to control the application
// Schema binds controller inputs to string keys to be used throughout the system
