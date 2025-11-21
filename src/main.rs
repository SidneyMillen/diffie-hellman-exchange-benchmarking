
mod clients;
mod key_exchange;
use macroquad::prelude::*;


#[macroquad::main("DiffieHellman")]
async fn main() {
    loop {
        clear_background(BEIGE);
        next_frame().await
    }
}
