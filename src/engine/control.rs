/*
    The engine::control module gathers input data for the model.

    it intends to do so in a crossplatform manner while making it
    easy for the dev by abstracting the low level inputs into 
    common actions, such as 'select' and 'confirm'
*/

// control::terminal interfaces with (drumroll, please) the terminal 
pub mod terminal{    
    use std::io;
    // gets string input
    pub fn get_line() -> String{
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Error");
        response
    }

    // shows prompt then gets input
    pub fn prompt(prompt: &str) -> String{
        println!("{}", prompt);
        get_line()
    }
}

// control::device interfaces with any physical devices that the user might use to control the application
pub mod device{
    // schema maps input values into key value pairs to be used in the model
    pub fn schema(){}
}

// control::resource deals with filehandling and memory management (if necessary)
pub mod resource{

}