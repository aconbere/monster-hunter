extern crate rusqlite;
extern crate rawsql;
extern crate docopt;
extern crate rustc_serialize;
extern crate flate2;
extern crate encoding;
extern crate regex;

use docopt::Docopt;

mod objects;
mod targets;
mod save;
mod archive;

const USAGE: &'static str = "
Usage: mh save decode <source> <destination>
       mh archive decode <source>
       mh archive decompress <source> <destination>
	   mh -h | --help
	   mh --version

Options:
    -h, --help  Show this message.
    --version  Show the version of mh.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_source:      String,
    arg_destination: String,
    cmd_save:        bool,
    cmd_archive:     bool,
    cmd_decode:      bool,
    cmd_decompress:  bool,
    cmd_decode_all:  bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());
	
	if args.cmd_save {
	    if args.cmd_decode {
            let character = save::decode(&args.arg_source);
            targets::sqlite::export_save(&args.arg_destination, &character);
        }
    }

    if args.cmd_archive {
        if args.cmd_decompress {
            archive::decompress(&args.arg_source, &args.arg_destination);
        } else if args.cmd_decode {
            let archives = archive::decode_text_files(&args.arg_source);
            targets::sqlite::export_archive(&args.arg_destination, archives);
        }
    }
}
