use as_ffi_bindings::{abort, AnyPtr, BufferPtr, Env, Read, StringPtr, Write};
use std::{error::Error, sync::Mutex};
use wasmer::{imports, Function, Instance, Module, Store};

#[test]
fn read_strings() -> Result<(), Box<dyn Error>> {
    let wasm_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test_wat.wat"));
    let store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;
    let import_object = imports! {
        "env" => {
            "abort" => Function::new_native_with_env(&store, Env::default(), abort),
        },
    };

    let instance = Instance::new(&module, &import_object)?;
    let memory = instance.exports.get_memory("memory").expect("get memory");

    let get_string = instance
        .exports
        .get_native_function::<(), StringPtr>("getString")?;

    let str_ptr = get_string.call()?;
    let string = str_ptr.read(memory)?;

    assert_eq!(string, "$¢ह한𝌆");

    Ok(())
}

#[test]
fn read_alloc_strings() -> Result<(), Box<dyn Error>> {
    let wasm_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/runtime_exported.wat"
    ));
    let store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;

    let import_object = imports! {
        "env" => {
            "abort" => Function::new_native_with_env(&store, Env::default(), abort),
        },
    };

    let instance = Instance::new(&module, &import_object)?;
    let memory = instance.exports.get_memory("memory").expect("get memory");

    let mut env = Env::default();
    env.init(&instance)?;

    let get_string = instance
        .exports
        .get_native_function::<(), StringPtr>("getString")?;

    let str_ptr = get_string.call()?;
    let string = str_ptr.read(memory)?;

    assert_eq!(string, "hello test");

    let str_ptr_2 = StringPtr::alloc(&"hello return".to_string(), &env)?;
    let string = str_ptr_2.read(memory)?;
    assert_eq!(string, "hello return");

    Ok(())
}

#[test]
fn read_write_strings() -> Result<(), Box<dyn Error>> {
    let wasm_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/runtime_exported.wat"
    ));
    let store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;

    let import_object = imports! {
        "env" => {
            "abort" => Function::new_native_with_env(&store, Env::default(), abort),
        },
    };

    let instance = Instance::new(&module, &import_object)?;
    let memory = instance.exports.get_memory("memory").expect("get memory");

    let mut env = Env::default();
    env.init(&instance)?;

    let get_string = instance
        .exports
        .get_native_function::<(), StringPtr>("getString")?;

    let mut str_ptr = get_string.call()?;
    let string = str_ptr.read(memory)?;

    assert_eq!(string, "hello test");

    str_ptr.write(&"hallo tast".to_string(), &env)?;

    let str_ptr_2 = get_string.call()?;
    let string = str_ptr_2.read(memory)?;

    assert_eq!(string, "hallo tast");
    Ok(())
}

#[test]
fn read_buffers() -> Result<(), Box<dyn Error>> {
    let wasm_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/buffer.wasm"));
    let store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;
    let import_object = imports! {
        "env" => {
            "abort" => Function::new_native_with_env(&store, Env::default(), abort),
        },
    };

    let instance = Instance::new(&module, &import_object)?;
    let memory = instance.exports.get_memory("memory").expect("get memory");

    let get_string = instance
        .exports
        .get_native_function::<(), BufferPtr>("get_buffer")?;

    let str_ptr = get_string.call()?;
    let vec = str_ptr.read(memory)?;
    let expected: Vec<u8> = vec![0x01, 0x03, 0x03, 0xFF];
    assert_eq!(vec, expected);
    Ok(())
}

#[test]
fn alloc_buffer() -> Result<(), Box<dyn Error>> {
    let wasm_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/sort_buffer.wasm"
    ));
    let store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;

    let import_object = imports! {
        "env" => {
            "abort" => Function::new_native_with_env(&store, Env::default(), abort),
        },
    };

    let instance = Instance::new(&module, &import_object)?;
    let memory = instance.exports.get_memory("memory").expect("get memory");

    let mut env = Env::default();
    env.init(&instance).unwrap();

    let sort_buffer = instance
        .exports
        .get_native_function::<i32, ()>("sortBuffer")?;

    let input: Vec<u8> = vec![0x03, 0x02, 0x08, 0x00, 0x04, 0x01, 0x05];
    let buffer_ptr = BufferPtr::alloc(&input, &env)?;
    sort_buffer.call(buffer_ptr.offset() as i32)?;
    let sorted = buffer_ptr.read(memory)?;

    let expected: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x08];

    assert_eq!(sorted, expected);

    // Now checking with odd size
    let input: Vec<u8> = vec![0x03, 0x02, 0x00, 0x01, 0x09];
    let buffer_ptr = BufferPtr::alloc(&input, &env)?;
    assert_eq!(buffer_ptr.size(memory)?, 5);

    Ok(())
}

lazy_static::lazy_static! {
    // static variable containing the printed values in test [read_write_any]
    static ref ANY_PRINTED: std::sync::Arc<Mutex<Vec<i32>>> = std::sync::Arc::new(Mutex::new(Vec::new()));
}

#[test]
fn read_write_any() -> Result<(), Box<dyn Error>> {
    fn print(val: i32) {
        ANY_PRINTED.lock().unwrap().push(val);
    }
    let wasm_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/my_struct.wasm"));
    // First get the exported object from a first module instance
    let exported = {
        let store = Store::default();
        let import_object = imports! {
            "env" => {
                "abort" => Function::new_native_with_env(&store, Env::default(), abort),
            },
            "index" => {
                "print" => Function::new_native(&store, print),
            }
        };
        let module = Module::new(&store, wasm_bytes)?;
        let instance = Instance::new(&module, &import_object)?;
        let memory = instance.exports.get_memory("memory").expect("get memory");

        let mut env = Env::default();
        env.init(&instance)?;

        let get_struct = instance
            .exports
            .get_native_function::<(), AnyPtr>("get_struct")?;

        get_struct.call()?.export(memory)?
    };
    {
        let store = Store::default();
        let import_object = imports! {
            "env" => {
                "abort" => Function::new_native_with_env(&store, Env::default(), abort),
            },
            "index" => {
                "print" => Function::new_native(&store, print),
            }
        };
        let module = Module::new(&store, wasm_bytes)?;
        let instance = Instance::new(&module, &import_object)?;
        instance.exports.get_memory("memory").expect("get memory");

        let mut env = Env::default();
        env.init(&instance)?;

        let print_vals = instance.exports.get_native_function::<i32, ()>("dump")?;
        let ptr = AnyPtr::import(&exported, &env)?.offset();
        assert_eq!(
            exported.id,
            AnyPtr::new(ptr).export(env.memory.get_ref().unwrap())?.id
        );
        print_vals.call(ptr as i32)?;
    };

    let p = ANY_PRINTED.lock().unwrap();
    let v = p.clone();
    assert_eq!(v, vec![12, 13, 12, 13]);
    Ok(())
}
