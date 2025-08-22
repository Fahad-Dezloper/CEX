use crate::types::Process;
pub struct Enginee;

impl Enginee {
    pub fn new() -> Self {
        Enginee
    }

    pub fn process(&self, msg: Process) {
        println!("Processing message: {:?} {:?}", msg.client_id, msg.message);
        // your trading logic here
        match msg.type_.as_str() {
            "CREATE_ORDER" => println!("Create Order"),
            "CANCEL_ORDER" => println!("CANCEL Order"),
            "GET_OPEN_ORDERS" => println!("GET_OPEN_ORDERS"),
            "ON_RAMP" => println!("ON_RAMP"),
            "GET_DEPTH" => println!("GET_DEPTH"),
            _ => println!("Unknown type: {}", msg.type_),
        }
    }
}