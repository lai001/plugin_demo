use anyhow::anyhow;
use engine::engine::Engine;
use plugin::Plugin;

fn load_plugin(index: i32, engine: &mut Engine) -> anyhow::Result<()> {
    let target = format!("./plugin_impl{}.dll", index);
    let source = "./plugin_impl.dll".to_string();
    if std::path::Path::new(&target).exists() {
        std::fs::remove_file(&target)?;
    }
    std::fs::copy(source, &target)?;
    let lib = unsafe { libloading::Library::new(&target) }?;
    pub type CreatePlugin = fn() -> Box<dyn Plugin>;
    let func: CreatePlugin = *unsafe { lib.get("get_plugin".as_bytes()) }?;
    let plugin = func();

    // let _ = std::io::Write::flush(&mut std::io::stdout());
    #[cfg(feature = "enable_ffi")]
    {
        println!("enable_ffi, {}", plugin.get_cfg());
        plugin.tick(engine as *mut Engine as _);
    }
    #[cfg(not(feature = "enable_ffi"))]
    {
        println!("not enable_ffi, {}", plugin.get_cfg());
        plugin.tick(engine);
    }
    Ok(())
}

fn set_current_dir() -> anyhow::Result<()> {
    let current = std::env::current_exe()?;
    let current_dir = current.parent().ok_or(anyhow!(""))?;
    Ok(std::env::set_current_dir(current_dir)?)
}

pub fn run() {
    let _ = set_current_dir();
    let mut index = 0;
    let mut engine = Engine::new();
    loop {
        let result: anyhow::Result<()> = (|| {
            std::io::stdin()
                .read_line(&mut String::new())
                .map_err(|err| anyhow!("{}", err))?;

            load_plugin(index, &mut engine)
        })();
        match result {
            Ok(_) => {
                index += 1;
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
}
