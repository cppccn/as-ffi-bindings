use super::{Env, Memory, Read, Write};
use crate::{BufferPtr, StringPtr};
use std::convert::{TryFrom, TryInto};
use std::mem;
use wasmer::{AsStoreMut, AsStoreRef, FromToNativeWasmType, Store, WasmPtr};

use crate::tools::export_asr;

// todo: should I implement Any ?
#[derive(Clone, Copy)]
pub struct AnyPtr(WasmPtr<u8>);
pub struct AnyPtrExported {
    pub id: u32,
    pub content: Vec<u8>,
}

impl AnyPtrExported {
    pub fn serialize(self) -> Vec<u8> {
        let ret = self.id.to_be_bytes().to_vec();
        [ret, self.content].concat()
    }

    pub fn deserialize(b: &[u8]) -> anyhow::Result<Self> {
        if b.len() < 4 {
            anyhow::bail!("any pointer to small")
        }
        Ok(Self {
            id: u32::from_be_bytes(b[..4].try_into()?),
            content: b[4..].to_vec(),
        })
    }
}

pub enum Type {
    String(Box<StringPtr>),
    Buffer(Box<BufferPtr>),
    Any(Box<AnyPtr>),
}

impl Type {
    pub fn offset(&self) -> u32 {
        match self {
            Type::String(ptr) => ptr.offset(),
            Type::Buffer(ptr) => ptr.offset(),
            Type::Any(ptr) => ptr.offset(),
        }
    }
}

impl AnyPtr {
    pub fn new(offset: u32) -> Self {
        Self(WasmPtr::new(offset))
    }
    pub fn to_type(self, memory: &Memory, store: &Store) -> anyhow::Result<Type> {
        let t = ptr_id(&self, memory, store)?;
        if t == 0 {
            Ok(Type::Buffer(Box::new(BufferPtr::new(self.offset()))))
        } else if t == 1 {
            Ok(Type::String(Box::new(StringPtr::new(self.offset()))))
        } else {
            Ok(Type::Any(Box::new(self)))
        }
    }
    /// Get ptr stored offset
    pub fn offset(&self) -> u32 {
        self.0.offset()
    }
    pub fn export(&self, memory: &Memory, store: &Store) -> anyhow::Result<AnyPtrExported> {
        let content = self.read(memory, store)?;
        let id = ptr_id(&self, memory, store)?;
        Ok(AnyPtrExported { content, id })
    }
    /// Create a new pointer with an allocation and write the pointer that
    /// has been writen. Return a pointer type.
    pub fn import(
        ptr_exported: &AnyPtrExported,
        env: &Env,
        memory: &Memory,
        store: &mut Store,
    ) -> anyhow::Result<Type> {
        if ptr_exported.id == 0 {
            Ok(Type::Buffer(BufferPtr::alloc(
                &ptr_exported.content,
                env,
                memory,
                store,
            )?))
        } else if ptr_exported.id == 1 {
            let utf16_vec = unsafe {
                let len = ptr_exported.content.len();
                if len % 2 != 0 {
                    anyhow::bail!("Cannot cast u8 slice into u16")
                }
                let c = ptr_exported.content.as_ptr().cast::<u16>();
                let a = std::slice::from_raw_parts(c, len / 2);
                a.to_vec()
            };
            Ok(Type::String(StringPtr::alloc(
                &String::from_utf16_lossy(&utf16_vec),
                env,
                memory,
                store,
            )?))
        } else {
            // todo write type anyway
            let ptr = AnyPtr::alloc(&ptr_exported.content, env, memory, store)?;
            set_id(ptr.offset(), ptr_exported.id, env, memory, store)?;
            Ok(Type::Any(ptr))
        }
    }
}

unsafe impl FromToNativeWasmType for AnyPtr {
    type Native = i32;
    fn to_native(self) -> Self::Native {
        self.offset() as i32
    }
    fn from_native(n: Self::Native) -> Self {
        Self::new(n as u32)
    }
}

impl Read<Vec<u8>> for AnyPtr {
    fn read(&self, memory: &Memory, store: &impl AsStoreRef) -> anyhow::Result<Vec<u8>> {
        let size = self.size(memory, store)?;

        let memory_view = memory.view(store);
        let wasm_slice_ = self.0.slice(&memory_view, size);

        if let Ok(wasm_slice) = wasm_slice_ {
            let mut res = Vec::with_capacity(size as usize * 2);
            res.resize(size as usize, 0);
            wasm_slice.read_slice(&mut res)?;
            Ok(res)
        } else {
            anyhow::bail!("Wrong offset: can't read buf")
        }
    }

    fn size(&self, memory: &Memory, store: &impl AsStoreRef) -> anyhow::Result<u32> {
        let memory_view = memory.view(&store);
        let ptr = self.0.sub_offset(4)?;
        let slice_len_buf = ptr.slice(&memory_view, 4)?.read_to_vec()?;
        Ok(u32::from_ne_bytes(slice_len_buf.try_into().map_err(
            |v| anyhow::Error::msg(format!("Unable to convert vec: {:?} to &[u8; 4]", v)),
        )?))
    }
}

impl Write<Vec<u8>> for AnyPtr {
    fn alloc(
        value: &Vec<u8>,
        env: &Env,
        memory: &Memory,
        store: &mut impl AsStoreMut,
    ) -> anyhow::Result<Box<AnyPtr>> {
        let new = export_asr!(fn_new, env);
        let size = i32::try_from(value.len())?;
        let offset = u32::try_from(new.call(store, size, 0)?)?;
        write_buffer(offset, value, env, memory, store)?;
        Ok(Box::new(AnyPtr::new(offset)))
    }

    fn write(
        &mut self,
        _value: &Vec<u8>,
        _env: &Env,
        _memory: &Memory,
        _store: &mut impl AsStoreMut,
    ) -> anyhow::Result<Box<Self>> {
        todo!()
        /*
        let memory = match env.memory.get_ref() {
            Some(mem) => mem,
            _ => anyhow::bail!("Cannot get memory"),
        };
        let prev_size = size(self.offset(), memory)?;
        let new_size = u32::try_from(value.len())?;
        if prev_size == new_size {
            write_buffer(self.offset(), value, env)?;
            Ok(Box::new(*self))
        } else {
            // unpin old ptr
            let unpin = export_asr!(fn_pin, env);
            unpin.call(&[Value::I32(self.offset().try_into()?)])?;

            // collect
            let collect = export_asr!(fn_collect, env);
            collect.call(&[])?;

            // alloc with new size
            AnyPtr::alloc(value, env)
        }
        */
    }

    fn free(self, _env: &Env, _store: &mut Store) -> anyhow::Result<()> {
        todo!("Release the memory from this any")
    }
}

fn write_buffer(
    offset: u32,
    value: &[u8],
    _env: &Env,
    memory: &Memory,
    store: &mut impl AsStoreMut,
) -> anyhow::Result<()> {
    let mem_view = memory.view(store);
    let from = u64::from(offset);
    mem_view.write(from, value)?;
    Ok(())
}

fn size(offset: u32, _memory: &Memory) -> anyhow::Result<u32> {
    if offset < 8 {
        anyhow::bail!("Wrong offset: less than 8")
    }
    todo!()

    /*
    // read -4 offset
    // https://www.assemblyscript.org/memory.html#internals
    if let Some(cell) = memory.view::<u32>().get(offset as usize / (32 / 8) - 1) {
        Ok(cell.get() / 2)
    } else {
        anyhow::bail!("Wrong offset: can't read size")
    }
    */
}

fn ptr_id(ptr: &AnyPtr, memory: &Memory, store: &Store) -> anyhow::Result<u32> {
    // TODO: check offset again
    /*
    if offset < 8 {
        anyhow::bail!("Wrong offset: less than 8")
    }
    */

    // From https://www.assemblyscript.org/runtime.html#memory-layout
    // @ -8 (type: u32): unique id of the concrete class
    let memory_view = memory.view(&store);
    let ptr = ptr.0.sub_offset(8)?;
    let slice_len_buf = ptr
        .slice(&memory_view, mem::size_of::<u32>() as u32)?
        .read_to_vec()?;
    Ok(u32::from_ne_bytes(slice_len_buf.try_into().map_err(
        |v| anyhow::Error::msg(format!("Unable to convert vec: {:?} to &[u8; 4]", v)),
    )?))
}

fn set_id(offset: u32, id: u32, _env: &Env, memory: &Memory, store: &Store) -> anyhow::Result<()> {
    if offset < 8 {
        anyhow::bail!("Wrong offset: less than 8")
    }

    let mem_view = memory.view(store);
    let from = u64::from(offset - 8);
    let to_write = id.to_ne_bytes();
    mem_view.write(from, &to_write[..])?;
    Ok(())
}
