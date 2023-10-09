use std::sync::Once;

pub const BUFFER_SIZE: usize = 1_073_741_824; // 1GB
static mut BUFFER: Option<Vec<u8>> = None;
static INIT_BUFFER: Once = Once::new();

pub fn load_buffer() {
    let data = std::fs::read("benches/files/benchmark_1G_random").expect("Failed to read the test file");
    unsafe {
        BUFFER = Some(data);
    }
}

pub fn ensure_buffer_loaded() {
    INIT_BUFFER.call_once(|| {
        load_buffer();
    });
}

pub fn get_buffer() -> &'static mut Vec<u8> {
    unsafe { BUFFER.as_mut().unwrap() }
}
