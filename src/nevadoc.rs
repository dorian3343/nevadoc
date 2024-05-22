mod doc_type;

use std::env;
use std::fs::{File};
use std::io::BufReader;
use std::path::Path;
use doc_type::{generate_docs,generate_docs_dir};
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Check for target file  / dir
    if args.len() > 1 {
        let path = Path::new(&args[1]);
        // Check if a target is a file or dir
        if let Ok(metadata) = fs::metadata(path) {
            if metadata.is_file() {
                //Check if the file is a Neva file
                if !is_neva_file(path){
                    println!("Documentation generation failed: {} is not a Neva file.",path.display());
                    return;
                }
                // Handle the file
                let file = match File::open(&path) {
                    Err(why) => panic!("Documentation generation failed: Couldn't open {}: {}", path.display(), why),
                    Ok(file) => file,
                };
                let reader = BufReader::new(file);
                generate_docs(reader,&path.display().to_string());
            } else if metadata.is_dir() {
                // get paths in dir
                generate_docs_dir(&path);


            }
        } else {
            println!("Documentation generation failed: The path does not exist or cannot be accessed.");
        }
    } else {
        println!("Usage: nevadoc [TARGET FILE / DIR]");
    }
}

fn is_neva_file(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.is_file() {
            if let Some(extension) = path.extension() {
                return extension == "neva";
            }
        }
    }
    false
}
