use std::{cell::RefCell, collections::HashMap, rc::Rc};

use loggix::{info, with_fields};

pub type SharedStore = Rc<RefCell<HashMap<Vec<u8>,Vec<u8>>>>;

const OP_SET: u8 = 1;
const OP_GET: u8 = 2;

pub struct Request {
    pub op: u8,
    pub key: Vec<u8>,
    pub value: Option<Vec<u8>>
}

pub async fn process(mut storage: SharedStore, req: Request){
    match req.op {
        OP_SET => {
            if let Some(value) = req.value {
                storage.borrow_mut().insert(req.key, value);
            }
        }
        OP_GET => {
            match storage.borrow().get(&req.key) {
                Some(data) => {
                },
                None => {
                }
            }
        },
        _ => {
            info!("protocol is not implemented yet");
        }
    }
}
