
# AssemblyScript bindings

Currently this binding library is compatible with wasmer and we are studying on a compatibility with wasmtime. With this helper are able to read, write and use the allocation in your webassembly modules compiled from AssemblyScript.


### Read

We provide two helpers, StringPtr and BufferPtr. The string correspond to a string in AssemblyScript and the BufferPtr to ArrayBuffer. If you want to read the return of a native function. It's really easy.

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

Note: If you choose to use `as_ffi_bindings::Env` you also have access to the memory when you declare your native function with env (using wasmer) so the memory of the instance is allays disponible.

```rust
let str_ptr: BufferPtr = get_buffer.call()?;
// Get directly a buffer pointer

let buffer: Vec<u8> = buffer_ptr.read(memory)?;
```

### Allocation/Writing

```rust
use as_ffi_bindings::{Write};
```

The `Write` traits allow you to free (indevelopment), write and allocate StringPtr and BufferPtr. This will allow you to send a value to your AssemblyScript module from rust!!

```rust
let input: Vec<u8> = vec![0x03, 0x02, 0x00, 0x01];
// instanciate a vector
let buffer_ptr = BufferPtr::alloc(&input, &env)?;
// Allocate a new buffer on the defined environment
sort_buffer.call(buffer_ptr.offset() as i32)?;
// Sort your buffer in webassembly
```

Everything remains accessible in the rust side. You can modfy your variable in the rust side with the `.write()` method and obviously in the webassembly module. So it's better to consider this as an unsafe action, pay attention 🥲.

### Env instanciation

You need to init your environment to allocate and write, it's because you need to use exported function as __new, __pin, accordingly to the beautifull AssemblyScript memory documentation 📚. This is automatically initialised when wasmer call a function in the `ImportObject` with an environment (examples comming soon).

But when wasmer isn't behind you, you haveto use your own hands! Look:

```rust
let mut env = Env::default();
env.init(&instance)?;
```

Not hard, right?

## More usage example

There is more subtil things to initialise, as the `abort` function in the ImportObject. Full examples for using features are in the test_features.rs file and we tried to use simple examples.

---

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

You can have a look to the 
