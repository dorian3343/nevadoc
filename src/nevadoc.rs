mod doc_type;

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use doc_type::{create_description, format_type_sig, get_name, Doc};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let path = Path::new(&args[1]);
        let file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };
        let reader = BufReader::new(file);
        let mut description = Vec::new();
        let mut doc = Doc::new(String::new(), String::new(), None); // Initialize with default values

        for line in reader.lines() {
            let line_contents = line.unwrap();

            if line_contents.trim().is_empty() {
                // Skip blank lines
                continue;
            }

            if line_contents.starts_with("///") {
                // Collect documentation lines
                description.push(line_contents);
            } else if line_contents.contains("component") {
                // Before processing the new component, finalize and write the previous doc if necessary
                if !doc.name.is_empty() && !doc.type_sig.is_empty() {
                    if !description.is_empty() {
                        doc.set_description(create_description(description.clone()));
                        description.clear();
                    }

                    // Write the finalized doc to the file
                    let mut file = match OpenOptions::new().append(true).create(true).open("output.md") {
                        Err(why) => panic!("couldn't open file: {}", why),
                        Ok(file) => file,
                    };

                    match writeln!(file, "{}", doc.generate_md()) {
                        Err(why) => panic!("couldn't write to file: {}", why),
                        Ok(_) => println!("successfully wrote to file"),
                    }
                }

                // Create a new Doc instance for the new component
                let name = get_name(line_contents.clone());
                let type_sig = format_type_sig(line_contents.clone());
                doc = Doc::new(name, type_sig, None);
            }
        }

        // Ensure the last Doc is written to the file
        if !doc.name.is_empty() && !doc.type_sig.is_empty() {
            if !description.is_empty() {
                doc.set_description(create_description(description.clone()));
            }

            let mut file = match OpenOptions::new().append(true).create(true).open("output.md") {
                Err(why) => panic!("couldn't open file: {}", why),
                Ok(file) => file,
            };

            match writeln!(file, "{}", doc.generate_md()) {
                Err(why) => panic!("couldn't write to file: {}", why),
                Ok(_) => println!("successfully wrote to file"),
            }
        }

    } else {
        println!("Usage: nevadoc [TARGET FILE]");
    }
}
