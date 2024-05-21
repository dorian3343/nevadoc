mod doc_type;

use std::env;
use std::fs::{File};
use std::io::BufReader;
use std::path::Path;
use doc_type::{generate_docs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let path = Path::new(&args[1]);
        let file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };
        let reader = BufReader::new(file);
        generate_docs(reader)

    } else {
        println!("Usage: nevadoc [TARGET FILE]");
    }
}
