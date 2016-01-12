use std::mem;
use std::fs::File;
use std::io::Read;

use objects::character::Character;

pub fn decode(source: &str) -> Character {
    let mut f = File::open(source).unwrap();
    let mut buffer: [u8; 75264] = [0; 75264]; 
    
    // TODO: Use read_exact, coming in Rust 1.6
    match f.read(&mut buffer) {
        Ok(75264) => {
            let character: Character = unsafe {
                mem::transmute(buffer)
            };
            character
        }
        _ => panic!("read failed")
    }
}

