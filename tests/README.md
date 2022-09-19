# Wasm tests

The .wasm files are builds from externals AS source codes and export a default runtime.

## Sort code

```ts

export function sortBuffer(a: StaticArray<u8>): void {
  a = a.sort();
}

```

