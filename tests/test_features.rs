use as_ffi_bindings::{abort, BufferPtr, Env, Env0, Read, StringPtr, Write};
use std::{error::Error, sync::Mutex};
use wasmer::{
    imports, Exports, Function, FunctionEnv, FunctionType, Imports, Instance, Memory, MemoryType,
    Module, Pages, Store, Type, Value, WasmPtr,
};

#[test]
fn read_strings() -> Result<(), Box<dyn Error>> {
    // let wasm_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test_wat.wat"));
    let wasm_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/get_string.wasm"
    ));
    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;

    // let env = Env::default();

    let host_function_signature =
        FunctionType::new(vec![Type::I32, Type::I32, Type::I32, Type::I32], vec![]);
    let host_function = Function::new(&mut store, &host_function_signature, |_args| {
        println!("Void");
        Ok(vec![Value::I32(42)])
        //Ok(())
    });

    // let fenv = FunctionEnv::new(&mut store, Env::default());

    let import_object = imports! {
        "env" => {
            "abort" => host_function // Function::new_typed_with_env(&mut store, &env, abort),
        },
    };

    let instance = Instance::new(&mut store, &module, &import_object)?;
    let memory = instance.exports.get_memory("memory").expect("get memory");

    let get_string = instance
        .exports
        // .get_native_function::<(), StringPtr>("getString")?;
        .get_typed_function::<(), StringPtr>(&store, "getString")?;

    let str_ptr = get_string.call(&mut store)?;
    let string = str_ptr.read(memory, &store)?;

    assert_eq!(string, "$¢ह한𝌆");

    Ok(())
}

#[test]
fn read_alloc_strings() -> Result<(), Box<dyn Error>> {
    let wasm_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/runtime_exported.wat"
    ));
    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;

    let host_function_signature =
        FunctionType::new(vec![Type::I32, Type::I32, Type::I32, Type::I32], vec![]);
    let host_function = Function::new(&mut store, &host_function_signature, |_args| {
        println!("Void");
        Ok(vec![Value::I32(42)])
    });

    let import_object = imports! {
        "env" => {
            "abort" => host_function,
        },
    };

    let instance = Instance::new(&mut store, &module, &import_object)?;
    let memory = instance.exports.get_memory("memory").expect("get memory");

    let mut env = Env::default();
    env.init(&instance)?;

    // let get_string = instance
    //     .exports
    //     .get_native_function::<(), StringPtr>("getString")?;

    let get_string = instance
        .exports
        .get_typed_function::<(), StringPtr>(&store, "getString")?;

    let str_ptr = get_string.call(&mut store)?;
    let string = str_ptr.read(memory, &store)?;

    assert_eq!(string, "hello test");

    let str_ptr_2 = StringPtr::alloc(&"hello return".to_string(), &env, &memory, &mut store)?;
    let string = str_ptr_2.read(memory, &store)?;
    assert_eq!(string, "hello return");

    Ok(())
}

#[test]
fn read_write_strings() -> Result<(), Box<dyn Error>> {
    let wasm_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/runtime_exported.wat"
    ));
    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;

    let host_function_signature =
        FunctionType::new(vec![Type::I32, Type::I32, Type::I32, Type::I32], vec![]);
    let host_function = Function::new(&mut store, &host_function_signature, |_args| {
        println!("Void");
        Ok(vec![Value::I32(42)])
    });

    let import_object = imports! {
        "env" => {
            "abort" => host_function // Function::new_native_with_env(&store, Env::default(), abort),
        },
    };

    let instance = Instance::new(&mut store, &module, &import_object)?;
    let memory = instance.exports.get_memory("memory").expect("get memory");

    let mut env = Env::default();
    env.init(&instance)?;

    // let get_string = instance
    //     .exports
    //     .get_native_function::<(), StringPtr>("getString")?;
    let get_string = instance
        .exports
        .get_typed_function::<(), StringPtr>(&store, "getString")?;

    let mut str_ptr = get_string.call(&mut store)?;
    let string = str_ptr.read(memory, &store)?;

    assert_eq!(string, "hello test");

    str_ptr.write(&"hallo tast".to_string(), &env, memory, &mut store)?;

    let str_ptr_2 = get_string.call(&mut store)?;
    let string = str_ptr_2.read(memory, &store)?;

    assert_eq!(string, "hallo tast");
    Ok(())
}

#[test]
fn read_buffers() -> Result<(), Box<dyn Error>> {
    let wasm_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/buffer.wasm"));
    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;

    // let abort_signature = FunctionType::new(vec![Type::I32, Type::I32, Type::i32, Type::I32], vec![]);
    /*
    let abort_function = Function::new(&mut store, &host_function_signature, |_args| {
        Ok(vec![Value::I32(42)])
    });
    */

    let host_function_signature =
        FunctionType::new(vec![Type::I32, Type::I32, Type::I32, Type::I32], vec![]);
    let host_function = Function::new(&mut store, &host_function_signature, |_args| {
        println!("Void");
        Ok(vec![Value::I32(42)])
        //Ok(())
    });

    // let fenv = FunctionEnv::new(&mut store, Env::default());

    let import_object = imports! {
        "env" => {
            "abort" => host_function, //Function::new_typed_with_env(&mut store, &env, abort),
        },
    };

    let instance = Instance::new(&mut store, &module, &import_object)?;
    let memory = instance.exports.get_memory("memory").expect("get memory");

    let get_buf = instance
        .exports
        .get_typed_function::<(), BufferPtr>(&store, "get_buffer")?;

    let buf_ptr = get_buf.call(&mut store)?;
    let vec = buf_ptr.read(memory, &store)?;

    // WORKING
    /*
    let get_buf = instance.exports.get_typed_function::<(), WasmPtr<u8>>(&store, "get_buffer")?;
    let res: WasmPtr<u8> = get_buf.call(&mut store)?;

    println!("res: {:?}", res);
    let memory_view = memory.view(&store);

    // let mut buf = Vec::with_capacity(4);
    // memory_view.read(res.pointer as u64, &mut buf);

    //println!("buf len: {}", buf.len());

    let res2 = res.sub_offset(4)?;
    println!("res subbed: {:?}", res2);

    let slice_len_buf = res2.slice(&memory_view, 4)?.read_to_vec()?;

    let slice_len = u32::from_ne_bytes( slice_len_buf.try_into().unwrap());
    println!("slice_len: {:?}", slice_len);

    // let slice_len = 4 * 2;
    let slice = res.slice(&memory_view, slice_len)?;
    println!("slice: {:?}", slice);
    //slice.read_slice(&mut buf);
    let vec = slice.read_to_vec();

    println!("buf: {:?}", vec);
    */

    // let res: Vec<u8> = slice
    //    .iter()
    //    .collect();

    // let buf = res.read(&memory_view);
    // println!("buf: {:?}", buf);
    // res.add_offset(1);
    // res.add_offset(1);
    // let buf = res.read(&memory_view);
    // println!("buf: {:?}", buf);
    // let buf = res.read(&memory_view);
    // println!("buf: {:?}", buf);
    // let buf = res.read(&memory_view);
    // println!("buf: {:?}", buf);

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
    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;

    let host_function_signature =
        FunctionType::new(vec![Type::I32, Type::I32, Type::I32, Type::I32], vec![]);
    let host_function = Function::new(&mut store, &host_function_signature, |_args| {
        println!("Void");
        Ok(vec![Value::I32(42)])
    });

    // let fenv = FunctionEnv::new(&mut store, Env::default());

    let import_object = imports! {
        "env" => {
            "abort" => host_function,
        },
    };

    let instance = Instance::new(&mut store, &module, &import_object)?;
    let memory = instance.exports.get_memory("memory").expect("get memory");

    let mut env = Env::default();
    env.init(&instance).unwrap();

    // let sort_buffer = instance
    //     .exports
    //     .get_native_function::<i32, ()>("sortBuffer")?;
    let sort_buffer = instance
        .exports
        .get_typed_function::<i32, ()>(&store, "sortBuffer")?;

    let input: Vec<u8> = vec![0x03, 0x02, 0x08, 0x00, 0x04, 0x01, 0x05];
    let buffer_ptr = BufferPtr::alloc(&input, &env, memory, &mut store)?;

    sort_buffer.call(&mut store, buffer_ptr.offset() as i32)?;
    let sorted = buffer_ptr.read(memory, &store)?;

    let expected: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x08];
    assert_eq!(sorted, expected);

    // Now checking with odd size
    let input: Vec<u8> = vec![0x03, 0x02, 0x00, 0x01, 0x09];
    let buffer_ptr = BufferPtr::alloc(&input, &env, memory, &mut store)?;
    assert_eq!(buffer_ptr.size(memory, &mut store)?, 5);

    Ok(())
}

#[test]
fn test_abort() -> Result<(), Box<dyn Error>> {
    let wasm_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/abort.wasm"));
    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;

    let host_function_signature =
        FunctionType::new(vec![Type::I32, Type::I32, Type::I32, Type::I32], vec![]);
    // let host_function = Function::new(&mut store, &host_function_signature, |_args| {
    //     println!("_args: {:?}", _args);
    //     println!("Void AA");
    //     Ok(vec![])
    // });

    let mut env = Env0::default();
    let fenv = FunctionEnv::new(&mut store, env);

    let memory: Memory = Memory::new(&mut store, MemoryType::new(Pages(1), None, false)).unwrap();
    fenv.as_mut(&mut store).memory = Some(memory.clone());
    let mut exports = Exports::new();
    exports.insert("memory", memory);
    exports.insert(
        "abort",
        Function::new_typed_with_env(&mut store, &fenv, abort),
    );

    let mut imports = Imports::new();
    imports.register_namespace("env", exports);

    let instance = Instance::new(&mut store, &module, &imports)?;
    // let memory = instance.exports.get_memory("memory").expect("get memory");

    // env.init(&instance).unwrap();

    // let sort_buffer = instance
    //     .exports
    //     .get_native_function::<i32, ()>("sortBuffer")?;
    let abort = instance
        .exports
        .get_typed_function::<(), ()>(&store, "to_abort")?;

    let res = abort.call(&mut store);
    println!("res: {:?}", res);
    Ok(())
}

lazy_static::lazy_static! {
    // static variable containing the printed values in test [read_write_any]
    static ref ANY_PRINTED: std::sync::Arc<Mutex<Vec<i32>>> = std::sync::Arc::new(Mutex::new(Vec::new()));
}
/*
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
*/
