use std::{collections::HashMap, ffi::CStr};

#[derive(Debug, Clone)]
pub struct Value {
    pub name: String,
    pub value: i32,
}

pub struct Engine {
    map: Box<HashMap<String, Value>>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            map: Box::new(HashMap::new()),
        }
    }

    pub fn dump(&mut self) {
        // println!("Engine::dump");
        let map = &self.map;
        for (k, v) in map.iter() {
            println!("{} : {:?}", k, v);
        }
    }

    pub fn add(&mut self, value: Value) {
        // println!("Engine::add");
        let map = &mut self.map;
        map.insert(value.name.clone(), value);
    }

    pub fn clear(&mut self) {
        // println!("Engine::clear");
        let map = &mut self.map;
        map.clear();
    }
}

#[no_mangle]
pub extern "C" fn Engine_dump(engine: *mut std::ffi::c_void) {
    let engine: &mut Engine = unsafe { std::mem::transmute(engine) };
    engine.dump();
}

#[no_mangle]
pub extern "C" fn Engine_add(
    engine: *mut std::ffi::c_void,
    name: *const std::ffi::c_char,
    value: i32,
) {
    let c_str_name = unsafe { CStr::from_ptr(name) };
    let name = c_str_name.to_str().expect("Should not be null");
    let engine: &mut Engine = unsafe { std::mem::transmute(engine) };
    let value = Value {
        name: name.to_string(),
        value,
    };
    engine.add(value.clone());
}

#[no_mangle]
pub extern "C" fn Engine_clear(engine: *mut std::ffi::c_void) {
    let engine: &mut Engine = unsafe { std::mem::transmute(engine) };
    engine.clear();
}
