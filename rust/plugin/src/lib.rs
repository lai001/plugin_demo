#[cfg(feature = "enable_ffi")]
pub type Engine = *mut std::ffi::c_void;

#[cfg(not(feature = "enable_ffi"))]
use engine::engine::Engine;

pub trait Plugin {
    #[cfg(feature = "enable_ffi")]
    fn tick(&self, engine: Engine);
    #[cfg(not(feature = "enable_ffi"))]
    fn tick(&self, engine: &mut Engine);
    fn get_cfg(&self) -> String;
}

#[link(name = "./target/debug/engine.dll")]
#[cfg(feature = "enable_ffi")]
extern "C" {
    pub fn Engine_dump(engine: Engine);
    pub fn Engine_add(engine: Engine, name: *const std::ffi::c_char, value: i32);
    pub fn Engine_clear(engine: Engine);
}
