use std::{collections::HashMap, sync::{Arc, LazyLock, RwLock}};

use dashmap::DashMap;

pub trait FromTypes: Sized {
    fn from_types(T: &Types) -> Result<Self,String>;
}

#[derive(Debug,Clone,serde::Serialize,serde::Deserialize)]
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

static STORAGE: LazyLock<DashMap<String, Arc<Types>>> = LazyLock::new(||{
    DashMap::new()
});

pub fn get(key: &String)->Option<Arc<Types>>{
    let value = STORAGE.get(key);
    if let Some(data) = value {
        return Some(data.value().clone());
    }
    return None;
}

pub fn set(key: String, value: Types){
    STORAGE.insert(key, Arc::new(value));
}
