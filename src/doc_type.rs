use std::io::{BufRead, BufReader};

use std::fs::OpenOptions;
use File;
use std::io::Write;
use env;
use Path;
use std::path::PathBuf;
use fs;
use std::io::Read;

#[derive(Debug)]
pub struct Doc {
    pub name: String,
    pub type_sig: String,
    pub description: Option<String>,
}

impl Doc {
    pub fn new(name: String, type_sig: String, description: Option<String>) -> Self {
        Doc { name, type_sig, description }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_type_sig(&mut self, type_sig: String) {
        self.type_sig = type_sig;
    }
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn generate_md(&self) -> String {
        let mut text = String::new();
        text += &format!("# {}\n", self.name);
        text += &format!("## Type signature\n```\n{}\n```\n", self.type_sig);
        if let Some(description) = &self.description {
            text += description;
        }
        text
    }
}


pub fn format_type_sig(text: String) -> String {
    if let Some(pos) = text.rfind(')') {
        text[..=pos].to_string()
    } else {
        text
    }
}
pub fn get_name(text: String) -> String {
    if let Some(start_pos) = text.find("component") {
        if let Some(end_pos) = text[start_pos..].find('(') {
            return text[start_pos + "component".len()..start_pos + end_pos].trim().to_string();
        }
    }
    String::new()
}

pub fn create_description(vec: Vec<String>) -> String {
    vec.iter()
        .map(|s| s.trim_start_matches("///").trim())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn generate_docs<R>(reader: BufReader<R>, output: &String) where R: std::io::Read{
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

                let mut file = match OpenOptions::new().append(true).create(true).open(output.clone()) {
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

        let mut file = match OpenOptions::new().append(true).create(true).open(output) {
            Err(why) => panic!("couldn't open file: {}", why),
            Ok(file) => file,
        };

        match writeln!(file, "{}", doc.generate_md()) {
            Err(why) => panic!("couldn't write to file: {}", why),
            Ok(_) => println!("successfully wrote to file"),
        }
    }
}

pub fn generate_docs_dir(path: &Path) -> Result<(), std::io::Error> {
    // Create a new folder
    let docs_path = format!("{}_docs", path.display());
    fs::create_dir(&docs_path)?;

    // Iterate over directory entries
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the extension is "neva"
        if let Some(ext) = path.extension() {
            if ext == "neva" {
                let file_stem = path.file_stem().and_then(|stem| stem.to_str()).unwrap_or("");
                let file = File::open(&path)?;
                let reader = BufReader::new(file);
                println!("{:?}",file_stem);
                generate_docs(reader, &(docs_path.clone() + "/README.md"));
            }
        }
    }
    Ok(())

}