use std::mem;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use objects::character::Character;
use targets::sqlite;

pub fn decode(source: &String, dest:&String) {
    let destination = Path::new(dest);

    let mut f = File::open(source).unwrap();
    let mut buffer: [u8; 75264] = [0; 75264]; 
    
    // TODO: Use read_exact, coming in Rust 1.6
    match f.read(&mut buffer) {
        Ok(75264) => {
            let result: Character = unsafe {
                mem::transmute(buffer)
            };
            sqlite::export(&result, &destination);
        }
        _ => panic!("read failed")
    }
}

