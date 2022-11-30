use super::{Env, Read, StringPtr};
use wasmer::{FunctionEnvMut, Store};

// if get_string throws an exception abort for some reason is being called
pub fn abort(
    mut ctx: FunctionEnvMut<Env0>,
    // message: StringPtr,
    // filename: StringPtr,
    message: u32,
    filename: u32,
    line: u32,
    col: u32,
) {
    // -> Result<(), wasmer::RuntimeError> {

    /*
    let memory = env.memory;
    let message = match message.read(memory, store) {
        Ok(msg) => msg,
        Err(err) => return Err(wasmer::RuntimeError::new(err.to_string())),
    };
    let filename = match filename.read(memory, store) {
        Ok(filename) => filename,
        Err(err) => return Err(wasmer::RuntimeError::new(err.to_string())),
    };
    */

    println!("message: {}", message);
    let mut buf = vec![0; 50 as usize];
    let memory = ctx.data_mut().memory.as_ref().expect("mem??").clone();
    println!("memory data size: {:?}", memory.view(&ctx).data_size());
    memory.view(&ctx).read(message as u64, &mut buf).unwrap();
    println!("buf message: {:?}", buf);
    // let input_string = std::str::from_utf8(&buf).unwrap();
    // println!("i str: {}", input_string);
    memory.view(&ctx).read(filename as u64, &mut buf).unwrap();
    println!("buf filename: {:?}", buf);

    let message = "yo";
    let filename = "yi";
    eprintln!("Error: {} at {}:{} col: {}", message, filename, line, col);
    // Ok(())
}

macro_rules! export_asr {
    ($func_name:ident, $env:expr) => {
        match $env.$func_name.as_ref() {
            Some(res) => res,
            _ => anyhow::bail!("Failed to get func"),
        }
    };
}

use crate::env::Env0;
pub(crate) use export_asr;
