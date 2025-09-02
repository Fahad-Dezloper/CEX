use crate::types::ProcessInput;


pub struct Engine{}


impl Engine { 
    pub fn new() -> Self {
        println!("wassup my man");
        Self {}
    }

    pub fn process(&self, msg: ProcessInput){
        println!("wassup mf {}", msg.client_id);
    }
}