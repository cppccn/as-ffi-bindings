use crate::tools::export_asr;

use super::{Env, Memory, Read, Write};

use std::convert::{TryFrom, TryInto};
use wasmer::{AsStoreRef, FromToNativeWasmType, Store, Value, WasmPtr};

#[derive(Clone, Copy)]
pub struct StringPtr(WasmPtr<u16>);

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
    fn read(&self, memory: &Memory, store: &impl AsStoreRef) -> anyhow::Result<String> {
        let size = self.size(memory, store)?;
        let memory_view = memory.view(store);
        let wasm_slice_ = self.0.slice(&memory_view, size);

        if let Ok(wasm_slice) = wasm_slice_ {
            let mut res: Vec<u16> = Vec::with_capacity(size as usize);
            res.resize(size as usize, 0);
            wasm_slice.read_slice(&mut res)?;
            Ok(String::from_utf16_lossy(&res))
        } else {
            anyhow::bail!("Wrong offset: can't read buf")
        }
    }

    fn size(&self, memory: &Memory, store: &impl AsStoreRef) -> anyhow::Result<u32> {
        let memory_view = memory.view(&store);
        // TODO: can we cast to WasmPtr<u8> ?
        let ptr = self.0.sub_offset(2)?; // 2 * u16 = 32 bits
        let slice_len_buf_ = ptr.slice(&memory_view, 2)?.read_to_vec()?;

        let slice_len_buf: Vec<u8> = slice_len_buf_
            .iter()
            .map(|i| i.to_ne_bytes())
            .flatten()
            .collect();

        let size = u32::from_ne_bytes(slice_len_buf.try_into().map_err(|v| {
            anyhow::Error::msg(format!("Unable to convert vec: {:?} to &[u8; 4]", v))
        })?);
        Ok(size / 2)
    }
}

impl Write<String> for StringPtr {
    fn alloc(
        value: &String,
        env: &Env,
        memory: &Memory,
        store: &mut Store,
    ) -> anyhow::Result<Box<StringPtr>> {
        let new = export_asr!(fn_new, env);
        let size = i32::try_from(value.len())?;

        // TODO: why 1?
        // Call __new with parameter: size & class id
        /*
        let offset = u32::try_from(
            match new
                .call(store, &[Value::I32(size * 2), Value::I32(1)])?
                .get(0)
            {
                Some(val) => match val.i32() {
                    Some(i) => i,
                    _ => anyhow::bail!("Failed to allocate"),
                },
                _ => anyhow::bail!("Failed to allocate"),
            },
        )?;
        */
        let offset = u32::try_from(new.call(store, size * 2, 1)?)?;
        write_str(offset, value, env, memory, store)?;

        // pin
        let pin = export_asr!(fn_pin, env);
        // pin.call(store, &[Value::I32(offset.try_into()?)])?;
        pin.call(store, offset.try_into()?)?;

        Ok(Box::new(StringPtr::new(offset)))
    }

    fn write(
        &mut self,
        value: &String,
        env: &Env,
        memory: &Memory,
        store: &mut Store,
    ) -> anyhow::Result<Box<StringPtr>> {
        let prev_size = size(&self, memory, store)?;
        let new_size = u32::try_from(value.len())?;

        if prev_size == new_size {
            write_str(self.offset(), value, env, memory, store)?;
            Ok(Box::new(*self))
        } else {
            // unpin old ptr
            let unpin = export_asr!(fn_unpin, env);
            // unpin.call(store, &[Value::I32(self.offset().try_into()?)])?;
            unpin.call(store, self.offset().try_into()?)?;

            // collect (e.g. perform full gc collection)
            let collect = export_asr!(fn_collect, env);
            collect.call(store)?;

            // alloc with new size
            StringPtr::alloc(value, env, memory, store)
        }
    }

    fn free(self, env: &Env, store: &mut Store) -> anyhow::Result<()> {
        todo!()
        // unpin
        /*
        let unpin = export_asr!(fn_unpin, env);
        unpin.call(store, &[Value::I32(self.offset().try_into()?)])?;

        // collect
        let collect = export_asr!(fn_collect, env);
        collect.call(store, &[])?;
        Ok(())
        */
    }
}

fn write_str(
    offset: u32,
    value: &str,
    env: &Env,
    memory: &Memory,
    store: &mut Store,
) -> anyhow::Result<()> {
    // let mem_view = env.memory.view(store);
    let mem_view = memory.view(store);

    let value_encoded: Vec<u8> = value
        .encode_utf16()
        .map(|item| item.to_ne_bytes())
        .flatten()
        .collect();

    // TODO: improve this msg
    // We count in 32 so we have to divide by 2
    // let from = u64::from(offset) / 2;
    let from = u64::from(offset);
    mem_view.write(from, &value_encoded[..])?;

    Ok(())
}

fn size0(offset: u32, memory: &Memory, store: &Store) -> anyhow::Result<u32> {
    if offset < 4 {
        anyhow::bail!("Wrong offset: less than 2")
    }
    // read -4 offset
    // https://www.assemblyscript.org/memory.html#internals
    /*
    if let Some(cell) = memory.view::<u32>().get(offset as usize / (32 / 8) - 1) {
        Ok(cell.get())
    } else {
        anyhow::bail!("Wrong offset: can't read size")
    }
    */

    /*
    let mut size_ = Vec::with_capacity(4);
    memory
        .view(store)
        .read(offset as u64 / (32 / 8) - 1, &mut size_[..])?;
    Ok(u32::from_ne_bytes(size_.try_into().unwrap()))
    */
    todo!()
}

fn size(string_ptr: &StringPtr, memory: &Memory, store: &Store) -> anyhow::Result<u32> {
    let memory_view = memory.view(&store);
    let ptr = string_ptr.0.sub_offset(2)?; // 2 * u16 = 32 bits
    let slice_len_buf_ = ptr.slice(&memory_view, 2)?.read_to_vec()?;

    let slice_len_buf: Vec<u8> = slice_len_buf_
        .iter()
        .map(|i| i.to_ne_bytes())
        .flatten()
        .collect();

    // TODO: no unwrap
    let size = u32::from_ne_bytes(slice_len_buf.try_into().unwrap());
    Ok(size / 2)
}
