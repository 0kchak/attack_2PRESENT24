use dm_crypto_rust::attaque::{attaque};
use std::time::{Instant};


fn main() {
    let start_time = Instant::now();
    attaque(); 
    let elapsed_time = start_time.elapsed();
    println!("Temps écoulé: {:?}", elapsed_time);
}
