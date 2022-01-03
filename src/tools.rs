use super::{Env, Read, StringPtr};

// if get_string throws an exception abort for some reason is being called
pub fn abort(
    env: &Env,
    message: StringPtr,
    filename: StringPtr,
    line: i32,
    col: i32,
) -> Result<(), wasmer::RuntimeError> {
    let memory = match env.memory.get_ref() {
        Some(mem) => mem,
        _ => return Err(wasmer::RuntimeError::new("Cannot get memory")),
    };
    let message = match message.read(memory) {
        Ok(msg) => msg,
        Err(err) => return Err(wasmer::RuntimeError::new(err.to_string())),
    };
    let filename = match filename.read(memory) {
        Ok(filename) => filename,
        Err(err) => return Err(wasmer::RuntimeError::new(err.to_string())),
    };
    eprintln!("Error: {} at {}:{} col: {}", message, filename, line, col);
    Ok(())
}

macro_rules! export_asr {
    ($func_name:ident, $env:expr) => {
        match $env.$func_name.as_ref() {
            Some(res) => res,
            _ => anyhow::bail!("Failed to get func"),
        }
    };
}
pub(crate) use export_asr;
