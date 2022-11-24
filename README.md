Macro to derive wasm-bindgen Typescript definitons from structs.

⚠️ Early WIP!

## Install

```
export_ts_macro={ git = "https://github.com/ivanschuetz/export_ts_macro", branch = "main" }
```

## Usage

```rust
use export_ts_macro::export_ts;
use wasm_bindgen::prelude::*;

#[export_ts(FooTs)]
struct Foo {
    a: i32,
    b: String,
}

#[wasm_bindgen]
pub fn hello(pars: FooTs) {
    let js: JsValue = pars.into();
    let _actual: Foo = js.into_serde().unwrap();
}
```

This generates the following Typescript:

```ts
interface FooTs {
  a: i32;
  b: string;
}

/**
 * @param {FooTs} pars
 */
export function foo(pars: FooTs): void;
```

## Internals

The macro generates code according to the [typescript_type](https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/typescript_type.html#typescript_type) section in the wasm-bindgen guide.

```rust
#[wasm_bindgen(typescript_custom_section)]
const FooTs: &'static str = r#"
interface FooTs {
    a: number
    b: string
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "FooTs")]
    pub type FooTs;
}
```
