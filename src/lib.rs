//! The current module give a free access to an helper. With the goal of
//! providing an helper to read and write basic pointers in a WebAssembly
//! script builded from the [AssemblyScript compiler](https://www.assemblyscript.org).
//!
//! Thanks to wasmer and the [AssemblyScript Runtime](https://www.assemblyscript.org/garbage-collection.html#runtime-interface),
//! we can provide functions like `alloc`, `read` and `write` to interact with
//! a given webassembly instance.
//!
//! # Helpers
//!
//! For the moment this crate implement helpers for the ArrayBuffer and for strings.
//! Historically the ArrayBuffer is less tested than the string. But the both allow
//! you to interact with a wasmer instance.
//!
//! ```ignore
//! let wasm_bytes = include_bytes!(concat!(
//! env!("CARGO_MANIFEST_DIR"),
//! "/tests/runtime_exported.wat"
//! ));
//! let store = Store::default();
//! let module = Module::new(&store, wasm_bytes)?;
//!
//! let import_object = imports! {
//! "env" => {
//!     "abort" => Function::new_native_with_env(&store, Env::default(), abort),
//! },
//! };
//!
//! let instance = Instance::new(&module, &import_object)?;
//! let memory = instance.exports.get_memory("memory").expect("get memory");
//!
//! let mut env = Env::default();
//! env.init(&instance)?;
//!
//! let get_string = instance
//! .exports
//! .get_native_function::<(), StringPtr>("getString")?;
//!
//! let str_ptr = get_string.call()?;
//! let string = str_ptr.read(memory)?;
//!
//! assert_eq!(string, "hello test");
//!
//! let str_ptr_2 = StringPtr::alloc(&"hello return".to_string(), &env)?;
//! let string = str_ptr_2.read(memory)?;
//! assert_eq!(string, "hello return");
//! ```
//!
//!
//!
//! # Unsafe note
//! This crate has a low-level access to your memory, it's often dangerous to
//! share memory between programs and you should consider this in your
//! project.
// mod any_ptr;
mod buffer_ptr;
mod env;
mod string_ptr;
mod tools;

//pub use any_ptr::AnyPtr;
//pub use any_ptr::AnyPtrExported;
//pub use any_ptr::Type;
pub use buffer_ptr::BufferPtr;
pub use env::Env;
pub use string_ptr::StringPtr;
pub use tools::abort;

use std::fmt;
use wasmer::{AsStoreRef, Memory, Store};

pub trait Read<T> {
    /// Read the value contained in the given memory at the current pointer
    /// offset.
    ///
    /// # Return
    /// A result with an Error if for some reason the binding failed to read
    /// the offset returned. It can be a cast error for example.
    ///
    /// Otherwise, a success with the value red at the point.
    ///
    /// # Example
    /// ```ignore
    /// let get_string = instance
    ///     .exports
    ///     .get_native_function::<(), StringPtr>("getString")?;
    /// let str_ptr = get_string.call()?;
    /// let string = str_ptr.read(memory)?;
    /// ```
    fn read(&self, memory: &Memory, store: &Store) -> anyhow::Result<T>;

    fn read2(&self, memory: &Memory, store: &impl AsStoreRef) -> anyhow::Result<T>;

    /// Read the size as indicated in the [AssemblyScript object header](https://www.assemblyscript.org/memory.html#internals)
    ///
    /// # Return
    /// A result with an Error if for some reason the binding failed to read
    /// the offset returned. It can be a cast error for example.
    ///
    /// Otherwise, a success with the size.
    ///
    /// # Example
    /// ```ignore
    /// let get_string = instance
    ///     .exports
    ///     .get_native_function::<(), StringPtr>("getString")?;
    /// let str_ptr = get_string.call()?;
    /// let size: u32 = str_ptr.size(memory)?;
    /// ```
    fn size(&self, memory: &Memory, store: &Store) -> anyhow::Result<u32>;

    fn size2(&self, memory: &Memory, store: &impl AsStoreRef) -> anyhow::Result<u32>;
}

pub trait Write<T> {
    /// Try to write in the given environment a new value thanks to the
    /// AssemblyScript runtime.
    ///
    /// # Return
    /// A result with an Error if the given environment don't export the
    /// [AssemblyScript Runtime](https://www.assemblyscript.org/garbage-collection.html#runtime-interface)
    /// or if for some reason the binding failed to read the offset returned.
    ///
    /// Otherwise, the result return a success containing the new pointer.
    ///
    /// # Example
    /// ```ignore
    /// let mut env = Env::default();
    /// env.init(&instance)?;
    /// let str_ptr = StringPtr::alloc(&"hello return".to_string(), &env)?;
    /// ```
    fn alloc(value: &T, env: &Env, memory: &Memory, store: &mut Store)
        -> anyhow::Result<Box<Self>>;
    /// Try to write in the given environment a value. If the size is
    /// different, we procede to free the previous string and realloc a new
    /// pointer.
    ///
    /// # Return
    /// A result with an Error if the given environment don't export the
    /// [AssemblyScript Runtime](https://www.assemblyscript.org/garbage-collection.html#runtime-interface)
    /// or if for some reason the binding failed to read the offset returned.
    ///
    /// Otherwise, the result return a success containing pointer. This pointer
    /// can be another if a reallocation occured.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut env = Env::default();
    /// env.init(&instance)?;
    /// let string = str_ptr.write(&"hello return".to_string(), &env)?;
    /// ```
    fn write(
        &mut self,
        value: &T,
        env: &Env,
        memory: &Memory,
        store: &mut Store,
    ) -> anyhow::Result<Box<Self>>;
    /// Unpin the pointer
    fn free(self, env: &Env, store: &mut Store) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub enum Error {
    Mem(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Mem(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}
