# Web Assembly
### Memory
Wasm has a linear memory model which is just a contiguous vector of raw bytes.
Your code can grow this memory but not shrink it.

### Execution
The WebAssembly computational model is based on a stack machine. This means
that every operation can be modeled by maybe popping some values off a virtual
stack, possibly doing something with these values, and then maybe pushing some
values onto this stack.

## Örnek Wasm Projesi Oluşturma
1. Wasm derleyicisi yüklemek için:
   `rustup target add wasm32-unknown-unknown`
1. Örnek bir proje oluşturulur
   `cargo new --lib do-nothing`
2. Derleme işlemi yapılır
    `cargo build --target wasm32-unknown-unknown --release`

## Toplama işlemi yapan wasm projesi
1. Proje oluşturulur
   `cargo new --lib do-addition`
2. Optimize bir wasm kodu elde etmek için sh dosyası çalıştırılır
   `chmod +x build.sh`
3. Build alınıp **do_addition.wasm** kodu www klasörü altına taşınır.
   `cargo build --target wasm32-unknown-unknown --release`\
   `chmod +x serve.py`
   
## Working with complex types
1. Proje oluşturulur
   `cargo new --lib hello-raw`