extern crate rusqlite;
extern crate rawsql;
extern crate docopt;

use std::mem;
use std::fs::File;
use std::io::Read;

use docopt::Docopt;

mod objects;
use objects::character::Character;

mod targets;
use targets::sqlite;


fn decode(source: &str) {
	// aborts on failure
    let mut f = File::open(source).unwrap();
    let character: Character = unsafe { mem::uninitialized() };
    // let mut buffer: [u8; 75266] = unsafe { mem::transmute(character) };
    let mut buffer: [u8; 75264] = unsafe { mem::transmute(character) };
    
    // TODO: Use read_exact, coming in Rust 1.6
    match f.read(&mut buffer) {
        Ok(75264) => {
            let result: Character = unsafe {
                mem::transmute(buffer)
            };
            println!("result: {:?}", result);
        }
        _ => panic!("read failed")
    }
}

const USAGE: &'static str = "
Usage: mh decode save <file>
	   mh -h | --help
	   mh --version

Options:
    -h, --help  Show this message.
    --version  Show the version of mh.
";

fn main() {
	let args = Docopt::new(USAGE)
					  .and_then(|d| d.parse())
					  .unwrap_or_else(|e| e.exit());
	
	let source = args.get_str("<file>");
	decode(source);
}
