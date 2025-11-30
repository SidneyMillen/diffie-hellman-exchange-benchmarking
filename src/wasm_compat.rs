use std::fmt;

use rand::{CryptoRng, RngCore};
use sapp_jsutils::JsObject;

unsafe extern "C" {
    fn macroquad_js_get_random_buffer(length: usize) -> JsObject;
}

/// Required by `getrandom` crate.
fn getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    let obj = unsafe { macroquad_js_get_random_buffer(buf.len()) };
    let mut bytes = Vec::with_capacity(buf.len());
    obj.to_byte_buffer(&mut bytes);

    for (target, data) in buf.iter_mut().zip(bytes) {
        *target = data;
    }
    Ok(())
}
getrandom::register_custom_getrandom!(getrandom);

pub struct MyRNG {}
//maybe a bad idea idk
impl RngCore for MyRNG {
    fn next_u32(&mut self) -> u32 {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        getrandom(&mut buffer).unwrap();
        u32::from_be_bytes(buffer)
    }
    fn next_u64(&mut self) -> u64 {
        let mut buffer: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        getrandom(&mut buffer).unwrap();
        u64::from_be_bytes(buffer)
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        getrandom(dest).unwrap();
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        getrandom(dest).unwrap();
        Ok(())
    }
}
impl CryptoRng for MyRNG {}
