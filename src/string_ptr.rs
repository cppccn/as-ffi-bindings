use crate::tools::{export_asr, export_mem};

use super::{Env, Memory, Read, Write};

use std::convert::{TryFrom, TryInto};
use wasmer::{AsStoreMut, AsStoreRef, FromToNativeWasmType, Store, WasmPtr};

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
    fn from_native(n: Self::Native) -> Self {
        Self::new(n as u32)
    }
    fn to_native(self) -> Self::Native {
        self.offset() as i32
    }
}

impl Read<String> for StringPtr {
    fn read(&self, memory: &Memory, store: &impl AsStoreRef) -> anyhow::Result<String> {
        let size = self.size(memory, store)?;
        let memory_view = memory.view(store);
        let wasm_slice_ = self.0.slice(&memory_view, size);

        if let Ok(wasm_slice) = wasm_slice_ {
            let mut res = vec![0; size as usize];
            wasm_slice.read_slice(&mut res)?;
            Ok(String::from_utf16_lossy(&res))
        } else {
            anyhow::bail!("Wrong offset: can't read buf")
        }
    }

    fn size(&self, memory: &Memory, store: &impl AsStoreRef) -> anyhow::Result<u32> {
        size(self, memory, store)

        /*
        let memory_view = memory.view(&store);
        // TODO: can we cast to WasmPtr<u8> ?
        let ptr = self.0.sub_offset(2)?; // 2 * u16 = 32 bits
        let slice_len_buf_ = ptr.slice(&memory_view, 2)?.read_to_vec()?;

        let slice_len_buf: Vec<u8> = slice_len_buf_
            .iter()
            .flat_map(|i| i.to_ne_bytes())
            .collect();

        let size = u32::from_ne_bytes(slice_len_buf.try_into().map_err(|v| {
            anyhow::Error::msg(format!("Unable to convert vec: {:?} to &[u8; 4]", v))
        })?);
        Ok(size / 2)
        */
    }
}

impl Write<String> for StringPtr {
    fn alloc(
        value: &String,
        env: &Env,
        store: &mut impl AsStoreMut,
    ) -> anyhow::Result<Box<StringPtr>> {
        let new = export_asr!(fn_new, env);
        let size = i32::try_from(value.len())?;

        // class id = 2
        // match AS `String` class id
        // see https://github.com/massalabs/massa-sc-runtime/blob/test-sc-using-as-25/src/tests/tests_runtime.rs#L314
        let offset = u32::try_from(new.call(store, size * 2, 2)?)?;
        write_str(offset, value, env, store)?;

        // pin
        let pin = export_asr!(fn_pin, env);
        pin.call(store, offset.try_into()?)?;

        Ok(Box::new(StringPtr::new(offset)))
    }

    fn write(
        &mut self,
        value: &String,
        env: &Env,
        store: &mut impl AsStoreMut,
    ) -> anyhow::Result<Box<StringPtr>> {
        let memory = export_mem!(env);
        let prev_size = size(self, memory, store)?;
        let new_size = u32::try_from(value.len())?;

        if prev_size == new_size {
            write_str(self.offset(), value, env, store)?;
            Ok(Box::new(*self))
        } else {
            // unpin old ptr
            let unpin = export_asr!(fn_unpin, env);
            unpin.call(store, self.offset().try_into()?)?;

            // collect (e.g. perform full gc collection)
            let collect = export_asr!(fn_collect, env);
            collect.call(store)?;

            // alloc with new size
            StringPtr::alloc(value, env, store)
        }
    }

    fn free(self, _env: &Env, _store: &mut Store) -> anyhow::Result<()> {
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
    store: &mut impl AsStoreMut,
) -> anyhow::Result<()> {
    let memory = export_mem!(env);
    let mem_view = memory.view(store);
    let value_encoded: Vec<u8> = value
        .encode_utf16()
        .flat_map(|item| item.to_ne_bytes())
        .collect();
    let from = u64::from(offset);
    mem_view.write(from, &value_encoded[..])?;
    Ok(())
}

fn size(string_ptr: &StringPtr, memory: &Memory, store: &impl AsStoreRef) -> anyhow::Result<u32> {
    let memory_view = memory.view(&store);
    // We need to offset: -32 bits, as ptr is WasmPtr<u16> we specify only an offset of 2
    let ptr = string_ptr.0.sub_offset(2)?; // 2 * u16 = 32 bits
    let slice_len_buf_ = ptr.slice(&memory_view, 2)?.read_to_vec()?;

    let slice_len_buf: Vec<u8> = slice_len_buf_
        .iter()
        .flat_map(|i| i.to_ne_bytes())
        .collect();

    let size =
        u32::from_ne_bytes(slice_len_buf.try_into().map_err(|v| {
            anyhow::Error::msg(format!("Unable to convert vec: {:?} to &[u8; 4]", v))
        })?);
    Ok(size / 2)
}
