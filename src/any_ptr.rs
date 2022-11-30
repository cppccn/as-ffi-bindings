use super::{Env, Memory, Read, Write};
use crate::{BufferPtr, StringPtr};
use std::convert::{TryFrom, TryInto};
use wasmer::{FromToNativeWasmType, Value, WasmPtr};

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
    pub fn to_type(self, memory: &Memory) -> anyhow::Result<Type> {
        let t = ptr_id(self.offset(), memory)?;
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
    pub fn export(&self, memory: &Memory) -> anyhow::Result<AnyPtrExported> {
        let content = self.read(memory)?;
        let id = ptr_id(self.offset(), memory)?;
        Ok(AnyPtrExported { content, id })
    }
    /// Create a new pointer with an allocation and write the pointer that
    /// has been writen. Return a pointer type.
    pub fn import(ptr_exported: &AnyPtrExported, env: &Env) -> anyhow::Result<Type> {
        if ptr_exported.id == 0 {
            Ok(Type::Buffer(BufferPtr::alloc(&ptr_exported.content, env)?))
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
            )?))
        } else {
            // todo write type anyway
            let ptr = AnyPtr::alloc(&ptr_exported.content, env)?;
            set_id(ptr.offset(), ptr_exported.id, env)?;
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
    fn read(&self, memory: &Memory) -> anyhow::Result<Vec<u8>> {
        let size = self.size(memory)?;
        if let Some(buf) = self.0.deref(memory, 0, size * 2) {
            Ok(buf.iter().map(|b| b.get()).collect())
        } else {
            anyhow::bail!("Wrong offset: can't read any object")
        }
    }

    fn size(&self, memory: &Memory) -> anyhow::Result<u32> {
        size(self.0.offset(), memory)
    }
}

impl Write<Vec<u8>> for AnyPtr {
    fn alloc(value: &Vec<u8>, env: &Env) -> anyhow::Result<Box<AnyPtr>> {
        let new = export_asr!(fn_new, env);
        let size = i32::try_from(value.len())?;
        let offset = u32::try_from(
            if let Some(value) = new.call(&[Value::I32(size), Value::I32(0)])?.get(0) {
                match value.i32() {
                    Some(offset) => offset,
                    _ => anyhow::bail!("Unable to allocate value"),
                }
            } else {
                anyhow::bail!("Unable to allocate value")
            },
        )?;
        write_buffer(offset, value, env)?;
        Ok(Box::new(AnyPtr::new(offset)))
    }

    fn write(&mut self, value: &Vec<u8>, env: &Env) -> anyhow::Result<Box<Self>> {
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
    }

    fn free(self, _env: &Env) -> anyhow::Result<()> {
        todo!("Release the memory from this string")
    }
}

fn write_buffer(offset: u32, value: &[u8], env: &Env) -> anyhow::Result<()> {
    let view = match env.memory.get_ref() {
        Some(mem) => mem.view::<u8>(),
        _ => anyhow::bail!("Uninitialized memory"),
    };
    // We count in 32 so we have to devide by 2
    let from = usize::try_from(offset)?;
    for (bytes, cell) in value.iter().zip(view[from..from + value.len()].iter()) {
        cell.set(*bytes);
    }
    Ok(())
}

fn size(offset: u32, memory: &Memory) -> anyhow::Result<u32> {
    if offset < 8 {
        anyhow::bail!("Wrong offset: less than 8")
    }
    // read -4 offset
    // https://www.assemblyscript.org/memory.html#internals
    if let Some(cell) = memory.view::<u32>().get(offset as usize / (32 / 8) - 1) {
        Ok(cell.get() / 2)
    } else {
        anyhow::bail!("Wrong offset: can't read size")
    }
}

fn ptr_id(offset: u32, memory: &Memory) -> anyhow::Result<u32> {
    if offset < 8 {
        anyhow::bail!("Wrong offset: less than 8")
    }
    // read -8 offset
    // https://www.assemblyscript.org/memory.html#internals
    if let Some(cell) = memory.view::<u32>().get(offset as usize / (32 / 8) - 2) {
        Ok(cell.get())
    } else {
        anyhow::bail!("Wrong offset: can't read type")
    }
}

fn set_id(offset: u32, id: u32, env: &Env) -> anyhow::Result<()> {
    if offset < 8 {
        anyhow::bail!("Wrong offset: less than 8")
    }
    let memory = match env.memory.get_ref() {
        Some(mem) => mem,
        _ => anyhow::bail!("Uninitialized memory"),
    };
    // read -8 offset
    // https://www.assemblyscript.org/memory.html#internals
    if let Some(cell) = memory.view::<u32>().get((offset as usize / (32 / 8)) - 2) {
        cell.set(id);
    } else {
        anyhow::bail!("Wrong offset: can't read type")
    }

    Ok(())
}
