use super::{Env, Memory, Read, Write};
use std::convert::{TryFrom, TryInto};
use std::mem::size_of;
use wasmer::{AsStoreMut, AsStoreRef, FromToNativeWasmType, Store, WasmPtr};

use crate::tools::{export_asr, export_mem};

#[derive(Clone, Copy)]
pub struct BufferPtr(WasmPtr<u8>);

impl BufferPtr {
    pub fn new(offset: u32) -> Self {
        Self(WasmPtr::new(offset))
    }
    pub fn offset(&self) -> u32 {
        self.0.offset()
    }
}

unsafe impl FromToNativeWasmType for BufferPtr {
    type Native = i32;
    fn from_native(n: Self::Native) -> Self {
        Self::new(n as u32)
    }
    fn to_native(self) -> Self::Native {
        self.offset() as i32
    }
}

impl Read<Vec<u8>> for BufferPtr {
    fn read(&self, memory: &Memory, store: &impl AsStoreRef) -> anyhow::Result<Vec<u8>> {
        let size = self.size(memory, store)?;
        let memory_view = memory.view(store);
        let wasm_slice_ = self.0.slice(&memory_view, size);
        if let Ok(wasm_slice) = wasm_slice_ {
            let mut res = vec![0; size as usize];
            wasm_slice.read_slice(&mut res)?;
            Ok(res)
        } else {
            anyhow::bail!("Wrong offset: can't read buf")
        }
    }

    fn size(&self, memory: &Memory, store: &impl AsStoreRef) -> anyhow::Result<u32> {
        let memory_view = memory.view(&store);
        let u32_size = size_of::<u32>() as u32;
        let ptr = self.0.sub_offset(u32_size)?;
        let slice_len_buf = ptr.slice(&memory_view, u32_size)?.read_to_vec()?;
        Ok(u32::from_ne_bytes(slice_len_buf.try_into().map_err(
            |v| anyhow::Error::msg(format!("Unable to convert vec: {:?} to &[u8; 4]", v)),
        )?))
    }
}

impl Write<Vec<u8>> for BufferPtr {
    fn alloc(
        value: &Vec<u8>,
        env: &Env,
        store: &mut impl AsStoreMut,
    ) -> anyhow::Result<Box<BufferPtr>> {
        let new = export_asr!(fn_new, env);
        let size = i32::try_from(value.len())?;

        // class id = 4
        // match AS `StaticArray<T>` class id
        // see https://github.com/massalabs/massa-sc-runtime/blob/test-sc-using-as-25/src/tests/tests_runtime.rs#L314
        let offset = u32::try_from(new.call(store, size, 1)?)?;

        write_buffer(offset, value, env, store)?;
        Ok(Box::new(BufferPtr::new(offset)))
    }

    fn write(
        &mut self,
        _value: &Vec<u8>,
        _env: &Env,
        _store: &mut impl AsStoreMut,
    ) -> anyhow::Result<Box<Self>> {
        todo!()
        /*
        let memory = match env.memory {
            Some(mem) => mem,
            None => anyhow::bail!("Could not keep memory!"),
        };

        let prev_size = size(self.offset(), memory, store)?;
        let new_size = u32::try_from(value.len())?;
        if prev_size == new_size {
            write_buffer(self.offset(), value, env, store)?;
            Ok(Box::new(*self))
        } else {
            // unpin old ptr
            let unpin = export_asr!(fn_pin, env);
            unpin.call(store, &[Value::I32(self.offset().try_into()?)])?;

            // collect
            let collect = export_asr!(fn_collect, env);
            collect.call(store, &[])?;

            // alloc with new size
            BufferPtr::alloc(value, env, store)
        }
        */
    }

    fn free(self, _env: &Env, _store: &mut Store) -> anyhow::Result<()> {
        todo!("Release the memory from this string")
    }
}

fn write_buffer(
    offset: u32,
    value: &[u8],
    env: &Env,
    store: &mut impl AsStoreMut,
) -> anyhow::Result<()> {
    let memory = export_mem!(env);
    let mem_view = memory.view(store);
    let from = u64::from(offset);
    mem_view.write(from, value)?;
    Ok(())
}

/*
fn size(offset: u32, memory: &Memory, store: &Store) -> anyhow::Result<u32> {
    if offset < 4 {
        anyhow::bail!("Wrong offset: less than 2")
    }
    // read -4 offset
    // https://www.assemblyscript.org/runtime.html#memory-layout
    let mut size_ = Vec::with_capacity(4);
    println!("Read at offset: {}", offset);
    memory.view(store).read(offset as u64 / (32 / 8) -1, &mut size_[..])?;
    //memory.view(store).read(offset as u64 - 4 - 1, &mut size_[..])?;
    println!("size: {:?}", size_);
    Ok(u32::from_ne_bytes(size_.try_into().unwrap()))
}
*/
