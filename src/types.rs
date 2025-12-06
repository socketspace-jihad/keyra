use std::{cell::RefCell, collections::HashMap, rc::Rc};
pub type SharedStore = Rc<RefCell<HashMap<Vec<u8>,Vec<u8>>>>;


pub enum RequestType {
    Get(Vec<u8>),
    Set(Vec<u8>,Vec<u8>)
}

pub struct CoreResponse {
    request_type: RequestType 
}

pub struct CoreRequest {
    request_type: RequestType
}
