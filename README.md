# AssemblyScript bindings [![cargo version](https://img.shields.io/crates/v/as-ffi-bindings.svg)](https://crates.io/crates/as-ffi-bindings)

Currently, this binding library is compatible with Wasmer, and we are studying on a compatibility with WasmTime. With this helper are able to read, write and use the allocation in your WebAssembly modules compiled from AssemblyScript.

### Read

We provide two helpers, `StringPtr` and `BufferPtr`. The string correspond to a string in AssemblyScript and the `BufferPtr` to `ArrayBuffer`. If you want to read the return of a native function, it's really easy:

```rust
use as_ffi_bindings::{Read, StringPtr};
let instance = Instance::new(&module, &import_object)?;
let memory = instance.exports.get_memory("memory").expect("get memory");
// Instanciation of your wasmer things

let get_string = instance
    .exports
    .get_native_function::<(), StringPtr>("getString")?;
// Get native function in your module

let str_ptr: StringPtr = get_string.call()?;
// Get directly a string pointer

let string: String = str_ptr.read(memory)?;
```

Note: If you choose to use `as_ffi_bindings::Env` you also have access to the memory when you declare your native function with env (using Wasmer) so the memory of the instance is always available:

```rust
let str_ptr: BufferPtr = get_buffer.call()?;
// Get directly a buffer pointer

let buffer: Vec<u8> = buffer_ptr.read(memory)?;
```

### Allocation/Writing

```rust
use as_ffi_bindings::{Write};
```

The `Write` traits allow you to free (in development), write and allocate `StringPtr` and `BufferPtr`. This allows you to send a value to your AssemblyScript module from Rust:

```rust
let input: Vec<u8> = vec![0x03, 0x02, 0x00, 0x01];
// instanciate a vector
let buffer_ptr = BufferPtr::alloc(&input, &env)?;
// Allocate a new buffer on the defined environment
sort_buffer.call(buffer_ptr.offset() as i32)?;
// Sort your buffer in webassembly
```

Everything remains accessible in the rust side. You can modify your variable in the rust side with the `.write()` method and obviously in the WebAssembly module. So it's better to consider this as an unsafe action, pay attention 🥲.

#### no_thread feature

The feature 'no_thread' can be enabled to efficiently copy a buffer into a BufferPtr. To avoid data races,
you should only use 1 thread to deal with the memory of your Wasmer instance.

### Env instantiation

You need to `init` your environment to allocate and write, it's because you need to use exported function as `__new`, `__pin`, accordingly to the beautiful AssemblyScript memory documentation 📚. This is automatically initialized when Wasmer call a function in the `ImportObject` with an environment (examples coming soon).

But when Wasmer isn't behind you, you have to use your own hands! Look:

```rust
let mut env = Env::default();
env.init(&instance)?;
```

Not hard, right?

## More usage example

There are more subtle things to initialize, as the `abort` function in the `ImportObject`. Full examples for using features are in the test_features.rs file, and we tried to use simple examples.

---

#### License

<sup>Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.</sup>

<sub>Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.</sub>

#### Credits

<sub>You can have a look at the original @onsails [`wasmer-as`](https://github.com/onsails/wasmer-as) experiment, on which this repository is a fork.</sub>

> Minimal Rust version: 1.56.1