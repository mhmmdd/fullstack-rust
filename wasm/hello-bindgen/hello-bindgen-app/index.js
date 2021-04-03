// import * as wasm from "hello-wasm-pack";
import * as wasm from "hello-bindgen";

let result = wasm.greet("Rust");
console.log(result);