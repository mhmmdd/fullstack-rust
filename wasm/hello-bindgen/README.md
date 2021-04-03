## Wasm-bindgen ile wasm projesi geliştir
1. Proje oluşturulur\
`cargo new --lib hello-bindgen`
2. Kütüphaneler eklenir
```   
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen="^0.2"
```

3. Hello world kodu _src/lib.rs_'e eklenir
```   
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

4. **wasm-pack** [indirilir](https://rustwasm.github.io/wasm-pack/installer/)
5. `wasm-pack build` komutu çalıştırılır\
   _pkg_ klasörü altında js dosyalarının oluştuğu görülür
6. **wasm-app** template ile npm uygulaması oluşturulur\
`npm init wasm-app hello-bindgen-app`
7. package.json dosyası düzenlenir, dependencies satırı eklenir ve "hello-wasm-pack" satırı silinir.
```   
"dependencies": {
    "hello-bindgen": "file:../pkg"
},
```

8. wasm/hello-bindgen/hello-bindgen-app/package.json dosyası düzenlenir, 
   1. dependencies satırı eklenir
   2. "hello-wasm-pack" satırı silinir.
   ```   
   "dependencies": {
     "hello-bindgen": "file:../pkg"
   },
   ```
9. wasm/hello-bindgen/hello-bindgen-app/index.js dosyası düzenlenir
```   
import * as wasm from "hello-bindgen";

let result = wasm.greet("Rust");
console.log(result);
```
10. `npm install` ve `npm start` komutları çalıştırılarak proje başlatılır
11. http://localhost:8080/ adresinde console'da **Hello, Rust!** yazdığı görülür