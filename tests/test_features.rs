use as_ffi_bindings::{abort, AnyPtr, BufferPtr, Env, Read, StringPtr, Write};
use std::{error::Error, sync::Mutex};
use wasmer::{
    imports, Exports, Function, FunctionEnv, FunctionType, Imports, Instance, Module, Store, Type,
};

#[test]
fn read_strings() -> Result<(), Box<dyn Error>> {
    // Test
    // Read of a string returned by a wasm function

    let wasm_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/get_string.wasm"
    ));
    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;

    let dummy_abort_function_signature =
        FunctionType::new(vec![Type::I32, Type::I32, Type::I32, Type::I32], vec![]);
    let dummy_abort_host_function =
        Function::new(&mut store, &dummy_abort_function_signature, |_args| {
            eprintln!("Dummy abort");
            Ok(vec![])
        });

    let import_object = imports! {
        "env" => {
            "abort" => dummy_abort_host_function,
        },
    };

    let instance = Instance::new(&mut store, &module, &import_object)?;
    let memory = instance.exports.get_memory("memory").expect("get memory");

    let get_string = instance
        .exports
        .get_typed_function::<(), StringPtr>(&store, "getString")?;

    let str_ptr = get_string.call(&mut store)?;
    let string = str_ptr.read(memory, &store)?;
    assert_eq!(string, "$¢ह한𝌆");
    Ok(())
}

#[test]
fn read_alloc_strings() -> Result<(), Box<dyn Error>> {
    // Test
    // Allocate a StringPtr in wasm memory then read it

    // TODO: use wasm file
    let wasm_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/runtime_exported.wat"
    ));
    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;

    let dummy_abort_function_signature =
        FunctionType::new(vec![Type::I32, Type::I32, Type::I32, Type::I32], vec![]);
    let dummy_abort_host_function =
        Function::new(&mut store, &dummy_abort_function_signature, |_args| {
            eprintln!("Dummy abort");
            Ok(vec![])
        });

    let import_object = imports! {
        "env" => {
            "abort" => dummy_abort_host_function,
        },
    };

    let instance = Instance::new(&mut store, &module, &import_object)?;

    let mut env = Env::default();
    let memory = instance.exports.get_memory("memory")?;
    let fn_new = instance
        .exports
        .get_typed_function::<(i32, i32), i32>(&store, "__new")?;
    let fn_pin = instance
        .exports
        .get_typed_function::<i32, i32>(&store, "__pin")?;
    env.init_with(Some(memory.clone()), Some(fn_new), Some(fn_pin), None, None);

    let get_string = instance
        .exports
        .get_typed_function::<(), StringPtr>(&store, "getString")?;

    // FIXME: should we remove this as we already have a test for string read?
    //        A good TU would: Take an allocated string, uppercase it in SC; then we read it and compare it
    let str_ptr = get_string.call(&mut store)?;
    let string = str_ptr.read(memory, &store)?;
    assert_eq!(string, "hello test");

    // alloc then read
    let to_alloc = String::from("hello return");
    let str_ptr_2 = StringPtr::alloc(&to_alloc, &env, &mut store)?;
    let string = str_ptr_2.read(memory, &store)?;
    assert_eq!(string, to_alloc);

    Ok(())
}

#[test]
fn read_write_strings() -> Result<(), Box<dyn Error>> {
    // Test
    // TODO

    // TODO: use wasm files
    let wasm_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/runtime_exported.wat"
    ));
    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;

    let dummy_abort_function_signature =
        FunctionType::new(vec![Type::I32, Type::I32, Type::I32, Type::I32], vec![]);
    let dummy_abort_host_function =
        Function::new(&mut store, &dummy_abort_function_signature, |_args| {
            eprintln!("Dummy abort");
            Ok(vec![])
        });

    let import_object = imports! {
        "env" => {
            "abort" => dummy_abort_host_function,
        },
    };

    let instance = Instance::new(&mut store, &module, &import_object)?;
    // let memory = instance.exports.get_memory("memory").expect("get memory");

    let mut env = Env::default();
    let memory = instance.exports.get_memory("memory")?;
    let fn_new = instance
        .exports
        .get_typed_function::<(i32, i32), i32>(&store, "__new")?;
    env.init_with(Some(memory.clone()), Some(fn_new), None, None, None);

    let get_string = instance
        .exports
        .get_typed_function::<(), StringPtr>(&store, "getString")?;

    // TODO: remove this - read_strings unit test already test this?
    let mut str_ptr = get_string.call(&mut store)?;
    let string = str_ptr.read(memory, &store)?;
    assert_eq!(string, "hello test");

    str_ptr.write(&"hallo tast".to_string(), &env, &mut store)?;
    let str_ptr_2 = get_string.call(&mut store)?;
    let string = str_ptr_2.read(memory, &store)?;
    assert_eq!(string, "hallo tast");

    Ok(())
}

#[test]
fn read_buffers() -> Result<(), Box<dyn Error>> {
    // Test
    // Read a buffer defined in a wasm function (StaticArray<u8> in AssemblyScript code)

    let wasm_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/get_buffer.wasm"
    ));
    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;

    let dummy_abort_function_signature =
        FunctionType::new(vec![Type::I32, Type::I32, Type::I32, Type::I32], vec![]);
    let dummy_abort_host_function =
        Function::new(&mut store, &dummy_abort_function_signature, |_args| {
            eprintln!("Dummy abort");
            Ok(vec![])
        });

    let import_object = imports! {
        "env" => {
            "abort" => dummy_abort_host_function,
        },
    };

    let instance = Instance::new(&mut store, &module, &import_object)?;
    let memory = instance.exports.get_memory("memory").expect("get memory");

    let get_buf = instance
        .exports
        .get_typed_function::<(), BufferPtr>(&store, "get_buffer")?;

    let buf_ptr = get_buf.call(&mut store)?;
    let vec = buf_ptr.read(memory, &store)?;
    let expected: Vec<u8> = vec![0x01, 0x03, 0x03, 0xFF];
    assert_eq!(vec, expected);

    let get_buf = instance
        .exports
        .get_typed_function::<(), BufferPtr>(&store, "get_buffer_2")?;

    let buf_ptr = get_buf.call(&mut store)?;
    let vec = buf_ptr.read(memory, &store)?;
    let expected: Vec<u8> = vec![0x01, 0x03, 0x03, 0xFE, 0xFF];
    assert_eq!(vec, expected);

    Ok(())
}

#[test]
fn alloc_buffer() -> Result<(), Box<dyn Error>> {
    // Test
    // Allocate a buffer in wasm memory, use a wasm function to sort it, then read & check it

    let wasm_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/sort_buffer.wasm"
    ));
    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;

    let dummy_abort_function_signature =
        FunctionType::new(vec![Type::I32, Type::I32, Type::I32, Type::I32], vec![]);
    let dummy_abort_host_function =
        Function::new(&mut store, &dummy_abort_function_signature, |_args| {
            eprintln!("Dummy abort");
            Ok(vec![])
        });

    let import_object = imports! {
        "env" => {
            "abort" => dummy_abort_host_function,
        },
    };

    let instance = Instance::new(&mut store, &module, &import_object)?;

    let mut env = Env::default();
    let memory = instance.exports.get_memory("memory")?;
    let fn_new = instance
        .exports
        .get_typed_function::<(i32, i32), i32>(&store, "__new")?;
    env.init_with(Some(memory.clone()), Some(fn_new), None, None, None);

    let sort_buffer = instance
        .exports
        .get_typed_function::<i32, ()>(&store, "sortBuffer")?;

    let mut input: Vec<u8> = vec![0x03, 0x02, 0x08, 0x00, 0x04, 0x01, 0x05];
    let buffer_ptr = BufferPtr::alloc(&input, &env, &mut store)?;

    sort_buffer.call(&mut store, buffer_ptr.offset() as i32)?;
    let sorted = buffer_ptr.read(memory, &store)?;

    input.sort();
    assert_eq!(sorted, input);

    // Now checking with odd size
    let input: Vec<u8> = vec![0x03, 0x02, 0x00, 0x01, 0x09];
    let buffer_ptr = BufferPtr::alloc(&input, &env, &mut store)?;
    assert_eq!(buffer_ptr.size(memory, &mut store)?, 5);

    Ok(())
}

#[test]
fn test_abort() -> Result<(), Box<dyn Error>> {
    // Test
    // Test a wasm function calling abort (which is a host function ~ Rust function)

    let wasm_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/abort.wasm"));
    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;

    let env = Env::default();
    let fenv = FunctionEnv::new(&mut store, env);

    let mut exports = Exports::new();
    exports.insert(
        "abort",
        Function::new_typed_with_env(&mut store, &fenv, abort),
    );

    let mut imports = Imports::new();
    imports.register_namespace("env", exports);

    let instance = Instance::new(&mut store, &module, &imports)?;

    // update FunctionEnv (so we can access memory in host function)
    let memory = instance.exports.get_memory("memory")?;
    let fn_new = instance
        .exports
        .get_typed_function::<(i32, i32), i32>(&store, "__new")?;
    let fn_pin = instance
        .exports
        .get_typed_function::<i32, i32>(&store, "__pin")?;
    let fn_unpin = instance
        .exports
        .get_typed_function::<i32, ()>(&store, "__unpin")?;
    let fn_collect = instance
        .exports
        .get_typed_function::<(), ()>(&store, "__collect")?;

    fenv.as_mut(&mut store).init_with(
        Some(memory.clone()),
        Some(fn_new),
        Some(fn_pin),
        Some(fn_unpin),
        Some(fn_collect),
    );

    let abort = instance
        .exports
        .get_typed_function::<(), ()>(&store, "to_abort")?;

    // TODO: find a way to check if abort is really called
    abort.call(&mut store).expect("Could not call abort");

    Ok(())
}

lazy_static::lazy_static! {
    // static variable containing the printed values in test [read_write_any]
    static ref ANY_PRINTED: std::sync::Arc<Mutex<Vec<i32>>> = std::sync::Arc::new(Mutex::new(Vec::new()));
}

#[test]
#[ignore]
fn read_write_any() -> Result<(), Box<dyn Error>> {
    fn print(val: i32) {
        ANY_PRINTED.lock().unwrap().push(val);
    }
    let wasm_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/my_struct.wasm"));

    // First get the exported object from a first module instance
    let exported = {
        let mut store = Store::default();
        let dummy_abort_function_signature =
            FunctionType::new(vec![Type::I32, Type::I32, Type::I32, Type::I32], vec![]);
        let dummy_abort_host_function =
            Function::new(&mut store, &dummy_abort_function_signature, |_args| {
                eprintln!("Dummy abort");
                Ok(vec![])
            });

        let import_object = imports! {
            "env" => {
                "abort" => dummy_abort_host_function,
            },
            "index" => {
                "print" => Function::new_typed(&mut store, print),
            }
        };
        let module = Module::new(&store, wasm_bytes)?;
        let instance = Instance::new(&mut store, &module, &import_object)?;
        // let memory = instance.exports.get_memory("memory").expect("get memory");

        let mut env = Env::default();
        let memory = instance.exports.get_memory("memory")?;
        let fn_new = instance
            .exports
            .get_typed_function::<(i32, i32), i32>(&store, "__new")?;
        let fn_pin = instance
            .exports
            .get_typed_function::<i32, i32>(&store, "__pin")?;
        let fn_unpin = instance
            .exports
            .get_typed_function::<i32, ()>(&store, "__unpin")?;
        let fn_collect = instance
            .exports
            .get_typed_function::<(), ()>(&store, "__collect")?;

        env.init_with(
            Some(memory.clone()),
            Some(fn_new),
            Some(fn_pin),
            Some(fn_unpin),
            Some(fn_collect),
        );

        let get_struct = instance
            .exports
            .get_typed_function::<(), AnyPtr>(&store, "get_struct")?;

        get_struct.call(&mut store)?.export(memory, &store)?
    };

    {
        let mut store = Store::default();
        let dummy_abort_function_signature =
            FunctionType::new(vec![Type::I32, Type::I32, Type::I32, Type::I32], vec![]);
        let dummy_abort_host_function =
            Function::new(&mut store, &dummy_abort_function_signature, |_args| {
                eprintln!("Dummy abort");
                Ok(vec![])
            });

        let import_object = imports! {
            "env" => {
                "abort" => dummy_abort_host_function,
            },
            "index" => {
                "print" => Function::new_typed(&mut store, print),
            }
        };
        let module = Module::new(&store, wasm_bytes)?;
        let instance = Instance::new(&mut store, &module, &import_object)?;

        let mut env = Env::default();
        let memory = instance.exports.get_memory("memory")?;
        let fn_new = instance
            .exports
            .get_typed_function::<(i32, i32), i32>(&store, "__new")?;
        let fn_pin = instance
            .exports
            .get_typed_function::<i32, i32>(&store, "__pin")?;
        let fn_unpin = instance
            .exports
            .get_typed_function::<i32, ()>(&store, "__unpin")?;
        let fn_collect = instance
            .exports
            .get_typed_function::<(), ()>(&store, "__collect")?;

        env.init_with(
            Some(memory.clone()),
            Some(fn_new),
            Some(fn_pin),
            Some(fn_unpin),
            Some(fn_collect),
        );

        let print_vals = instance
            .exports
            .get_typed_function::<i32, ()>(&store, "dump")?;
        let ptr = AnyPtr::import(&exported, &env, &memory, &mut store)?.offset();
        assert_eq!(exported.id, AnyPtr::new(ptr).export(memory, &store)?.id);
        print_vals.call(&mut store, ptr as i32)?;
    };

    let p = ANY_PRINTED.lock().unwrap();
    let v = p.clone();
    assert_eq!(v, vec![12, 13, 12, 13]);

    Ok(())
}
