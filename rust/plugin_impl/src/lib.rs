#[cfg(not(feature = "enable_ffi"))]
extern crate engine;
#[cfg(not(feature = "enable_ffi"))]
extern crate plugin;

#[cfg(not(feature = "enable_ffi"))]
use engine::engine::Engine;
#[cfg(not(feature = "enable_ffi"))]
use engine::engine::Value;
#[cfg(not(feature = "enable_ffi"))]
use plugin::Plugin;

#[cfg(feature = "enable_ffi")]
use plugin::*;

struct CustomPlugin;

impl Plugin for CustomPlugin {
    #[cfg(not(feature = "enable_ffi"))]
    fn tick(&self, engine: &mut Engine) {
        engine.clear();
        engine.add(Value {
            name: "key".to_string(),
            value: 1024,
        });
        engine.dump();
    }

    #[cfg(feature = "enable_ffi")]
    fn tick(&self, engine: Engine) {
        unsafe {
            Engine_clear(engine);
            let name = "key";
            let value = 1024;
            let name = std::ffi::CString::new(name).unwrap();
            Engine_add(engine, name.as_c_str().as_ptr(), value);
            Engine_dump(engine);
        }
    }
    
    fn get_cfg(&self) -> String {
        #[cfg(feature = "enable_ffi")]
        return "enable_ffi".to_string();
        #[cfg(not(feature = "enable_ffi"))]
        return "not enable_ffi".to_string();
    }
}

#[no_mangle]
pub fn get_plugin() -> Box<dyn Plugin> {
    Box::new(CustomPlugin {})
}
