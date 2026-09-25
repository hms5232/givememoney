# givememoney wasm

A WebAssembly wrapper for the [`givememoney`](../core) crate.

## Build

Add the wasm32 target via rustup and install wasm-pack first:

```shell
rustup target add wasm32-unknown-unknown
cargo install wasm-pack # or `cargo binstall wasm-pack`
```

Then run the following command to build in this directory:

```shell
wasm-pack build --target web
```

The output will be generated in `./pkg/`.

## Usage

```html
<script type="module">
  import init, { allocate } from "./pkg/givememoney_wasm.js";

  await init();

  try {
    // throws an `Error` if the input is invalid
    const result = allocate({
      total: 100,
      players: [
        { name: "Alice", amount: 70 },
        { amount: 40 }, // name is optional
      ],
    });
    console.log(result);
    // {
    //   total: 100,
    //   players: [
    //     { name: "Alice", number: 1, original: 70, allocated: 64, displayName: "Alice" },
    //     { name: undefined, number: 2, original: 40, allocated: 36, displayName: "2" }
    //   ]
    // }
  } catch (e) {
    console.error(e.message);
  }
</script>
```

## Developing

You need a local web server because browsers can't load wasm modules through `file://`.

For example, use Python in this directory to host a local server on port 8000:

```shell
python -m http.server 8000
```

Then open <http://localhost:8000> and check the result in the DevTools console.
