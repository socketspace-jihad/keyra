use std::{cell::RefCell, collections::HashMap, future::Future, io::Error, rc::Rc, sync::{Arc, LazyLock, Mutex, RwLock}};

use capnp::capability::{self, Promise};

mod data_capnp {
    include!(concat!(env!("OUT_DIR"),"/data_capnp.rs"));
}

use data_capnp::{data::{GetParams, GetResults, ListParams, ListResults, ReceiveParams, ReceiveResults, Server}, types::kind::{self, Reader}};


static STORAGE: LazyLock<Arc<RwLock<HashMap<String,Types>>>> = LazyLock::new(||{
    Arc::new(RwLock::new(HashMap::new()))
});

pub trait FromTypes: Sized {
    fn from_types(T: &Types) -> Result<Self,String>;
}

pub struct Data;

impl Server for Data {
    fn receive(&self,params: ReceiveParams<>,res: ReceiveResults<>) ->  capnp::capability::Promise<(), capnp::Error> {
        let key: capnp::text::Reader  = params.get().unwrap().get_key().unwrap();
        println!("key {:?}",key.to_string());
        
        let value: Reader= params.get().unwrap().get_value().unwrap().get_kind();
        match value.which().unwrap() {
           kind::Which::StringVal(data)=>{
               let v = Types::STRING(data.unwrap().to_string().unwrap());
               (*STORAGE.write().unwrap()).insert(key.to_string().unwrap().clone(), v);
           },
           _ => println!("not implemented yet!"),
        }
        capnp::capability::Promise::ok(())

    }

    fn list(&self, _: ListParams<>, res: ListResults<>) -> Promise<(),capnp::Error> {
        println!("list executed");
        let storage = STORAGE.read().unwrap();
        for (k,v) in storage.iter() {
            println!("{k:?}: {v:?}");
        };
        capnp::capability::Promise::ok(())
    }

    fn get(&self,key: GetParams<>,mut res: GetResults<>) -> capnp::capability::Promise<(),capnp::Error> {
        let k = key.get().unwrap().get_key().unwrap().to_string().unwrap();
        let value = (*STORAGE.read().unwrap()).get(&k).cloned();
        if let Some(val) = value {
            match val {
                Types::STRING(data)=>{
                    res.get().init_value().init_kind().set_string_val(data);
                },
                _ => {
                    println!("not implemented yet!");
                }
            }
        }

        capnp::capability::Promise::ok(())
    }

}

#[derive(Debug,Clone)]
pub enum Types {
    STRING(String),
    I64(i64),
    I32(i32),
    I16(i16),
    I8(i8),
    U64(u64),
    U32(u32),
    U16(u16),
    U8(u8),
    F64(f64),
    F32(f32),
    MAP(HashMap<String,Types>),
    ARRAY(Vec<Types>)
}

impl Types {
    pub fn is_array(self)->Option<Vec<Types>> {
        if let Types::ARRAY(data) = self {
            return Some(data);
        }
        return None;
    }
}

impl FromTypes for String {
    fn from_types(T: &Types) -> Result<Self,String> {
        if let Types::STRING(s) = T {
            return Ok(s.clone());
        }
        return Err("cannot convert to string".to_string());
    }
}

impl FromTypes for i64 {
    fn from_types(T: &Types) -> Result<Self,String> {
        if let Types::I64(s) = T {
            return Ok(s.clone());
        }
        return Err("cannot convert to i64".to_string());
    }
}

impl FromTypes for HashMap<String,Types> {
    fn from_types(T: &Types) -> Result<Self,String> {
        if let Types::MAP(s) = T {
            return Ok(s.clone());
        }   
        return Err("cannot convert to hash-map".to_string());
    }
}

impl FromTypes for Vec<Types>{
    fn from_types(T: &Types) -> Result<Self,String> {
        if let Types::ARRAY(s) = T {
            return Ok(s.clone());
        }
        return Err("cannot convert to array".to_string());
    }
}

impl Types {
    pub fn to<T: FromTypes>(&self) -> Result<T,String>{
        return T::from_types(self);
    }
}

pub async fn set(key: &String, val: &Types) -> Result<(),Error> {
    (*STORAGE.write().unwrap()).insert(key.clone(), val.clone());
    return Ok(());
}

pub async fn get(key: &String) -> Option<Types> {
    (*STORAGE.read().unwrap()).get(key).cloned()
}
