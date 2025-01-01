use std::time::{SystemTime, UNIX_EPOCH};

pub fn random_number(min :u32 , max: u32) -> u32{
    let duration =SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time went backwards");
    
    let seed = duration.as_secs() ^ duration.subsec_nanos() as u64;
    let mut number = seed as u32;

    number = number.wrapping_mul(1664525).wrapping_add(1013904223);
    

    min + (number % (max - min + 1))



}