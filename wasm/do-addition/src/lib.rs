#[no_mangle] // method adı sadece add olması için
pub extern "C" fn add(a: u32, b: u32) -> u32 {
    a + b
}