use crate::tools::export_asr;

use super::{Env, Memory, Read, Write};

use std::convert::{TryFrom, TryInto};
use wasmer::{Array, FromToNativeWasmType, Value, WasmPtr};

#[derive(Clone, Copy)]
pub struct StringPtr(WasmPtr<u16, Array>);

impl StringPtr {
    pub fn new(offset: u32) -> Self {
        Self(WasmPtr::new(offset))
    }
    pub fn offset(&self) -> u32 {
        self.0.offset()
    }
}

unsafe impl FromToNativeWasmType for StringPtr {
    type Native = i32;
    fn to_native(self) -> Self::Native {
        self.offset() as i32
    }
    fn from_native(n: Self::Native) -> Self {
        Self::new(n as u32)
    }
}

impl Read<String> for StringPtr {
    fn read(&self, memory: &Memory) -> anyhow::Result<String> {
        let size = self.size(memory)?;
        // we need size / 2 because assemblyscript counts bytes
        // while deref considers u16 elements
        if let Some(buf) = self.0.deref(memory, 0, size / 2) {
            let input: Vec<u16> = buf.iter().map(|b| b.get()).collect();
            Ok(String::from_utf16_lossy(&input))
        } else {
            anyhow::bail!("Wrong offset: can't read buf")
        }
    }

    fn size(&self, memory: &Memory) -> anyhow::Result<u32> {
        size(self.0.offset(), memory)
    }
}

impl Write<String> for StringPtr {
    fn alloc(value: &String, env: &Env) -> anyhow::Result<Box<StringPtr>> {
        let new = export_asr!(fn_new, env);
        let size = i32::try_from(value.len())?;

        let offset = u32::try_from(
            match new.call(&[Value::I32(size << 1), Value::I32(1)])?.get(0) {
                Some(val) => match val.i32() {
                    Some(i) => i,
                    _ => anyhow::bail!("Failed to allocate"),
                },
                _ => anyhow::bail!("Failed to allocate"),
            },
        )?;
        write_str(offset, value, env)?;

        // pin
        let pin = export_asr!(fn_pin, env);
        pin.call(&[Value::I32(offset.try_into()?)])?;

        Ok(Box::new(StringPtr::new(offset)))
    }

    fn write(&mut self, value: &String, env: &Env) -> anyhow::Result<Box<StringPtr>> {
        let memory = match env.memory.get_ref() {
            Some(mem) => mem,
            _ => anyhow::bail!("Cannot get memory"),
        };
        let prev_size = size(self.offset(), memory)?;
        let new_size = u32::try_from(value.len())? << 1;
        if prev_size == new_size {
            write_str(self.offset(), value, env)?;
            Ok(Box::new(*self))
        } else {
            // unpin old ptr
            let unpin = export_asr!(fn_unpin, env);
            unpin.call(&[Value::I32(self.offset().try_into()?)])?;

            // collect
            let collect = export_asr!(fn_collect, env);
            collect.call(&[])?;

            // alloc with new size
            StringPtr::alloc(value, env)
        }
    }

    fn free(self, env: &Env) -> anyhow::Result<()> {
        // unpin
        let unpin = export_asr!(fn_unpin, env);
        unpin.call(&[Value::I32(self.offset().try_into()?)])?;

        // collect
        let collect = export_asr!(fn_collect, env);
        collect.call(&[])?;
        Ok(())
    }
}

fn write_str(offset: u32, value: &str, env: &Env) -> anyhow::Result<()> {
    let utf16 = value.encode_utf16();
    let view = match env.memory.get_ref() {
        Some(mem) => mem.view::<u16>(),
        _ => anyhow::bail!("Uninitialized memory"),
    };
    // We count in 32 so we have to devide by 2
    let from = usize::try_from(offset)? / 2;
    for (bytes, cell) in utf16.into_iter().zip(view[from..from + value.len()].iter()) {
        cell.set(bytes);
    }
    Ok(())
}

fn size(offset: u32, memory: &Memory) -> anyhow::Result<u32> {
    if offset < 4 {
        anyhow::bail!("Wrong offset: less than 2")
    }
    // read -4 offset
    // https://www.assemblyscript.org/memory.html#internals
    if let Some(cell) = memory.view::<u32>().get(offset as usize / (32 / 8) - 1) {
        Ok(cell.get())
    } else {
        anyhow::bail!("Wrong offset: can't read size")
    }
}
