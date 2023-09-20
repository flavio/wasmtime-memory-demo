use anyhow::Result;
use std::{thread, time};
use wasmtime;

fn main() -> Result<()> {
    // let wat = r#"
    //     (module
    //       (func (export "main")
    //             (result i32)
    //         i32.const 42
    //         return))
    // "#;

    // disable the cache, this removes a process that handles the cache
    let mut config = wasmtime::Config::default();
    config.disable_cache();

    let engine = wasmtime::Engine::new(&config)?;
    //    let module = wasmtime::Module::new(&engine, wat)?;
    let module = wasmtime::Module::from_file(&engine, "../guest/guest.wasm")?;

    let mut store = wasmtime::Store::new(&engine, ());
    let instance = wasmtime::Instance::new(&mut store, &module, &[])?;

    let meaning_of_life_fn: wasmtime::TypedFunc<(), i32> =
        instance.get_typed_func(&mut store, "meaning_of_life")?;
    let result = meaning_of_life_fn.call(&mut store, ())?;

    println!("The wasm module said: {}", result);

    println!("Taking 1h power nap");
    let one_hour = time::Duration::from_secs(3600);
    thread::sleep(one_hour);
    println!("AWAKE!");

    Ok(())
}
