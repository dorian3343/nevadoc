use std::io::{BufRead, BufReader};

use std::fs::OpenOptions;
use std::io::Write;
use Path;
use fs;
use File;
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
    let noutput = rename_to_doc_md(output);
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
                println!("{}",output);
                let mut file = match OpenOptions::new().append(true).create(true).open(noutput.clone()) {
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

        let mut file = match OpenOptions::new().append(true).create(true).open(noutput) {
            Err(why) => panic!("couldn't open file: {}", why),
            Ok(file) => file,
        };

        match writeln!(file, "{}", doc.generate_md()) {
            Err(why) => panic!("couldn't write to file: {}", why),
            Ok(_) => println!("successfully wrote to file"),
        }
    }
}
pub fn generate_docs_dir(path: &Path) {
    let paths = match fs::read_dir(path) {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("Failed to read directory {}: {}", path.display(), e);
            return;
        },
    };

    let new_name = path.with_file_name(format!("{}_docs", path.file_name().unwrap().to_string_lossy()));
    if let Err(e) = fs::create_dir(&new_name) {
        eprintln!("Failed to create directory {}: {}", new_name.display(), e);
        return;
    }

    for entry in paths {
        match entry {
            Ok(entry) => {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    generate_docs_dir(&entry_path);
                } else {
                    let file = match File::open(&entry_path) {
                        Err(why) => panic!("Documentation generation failed: Couldn't open {}: {}", path.display(), why),
                        Ok(file) => file,
                    };
                    let reader = BufReader::new(file);
                    generate_docs(reader, &(new_name.display().to_string() + "/" + &path.display().to_string()));
                }
            },
            Err(e) => eprintln!("Failed to read an entry: {}", e),
        }
    }
    println!("New directory created: {}", new_name.display());
}

fn rename_to_doc_md(file_name: &str) -> String {
    let mut parts: Vec<&str> = file_name.splitn(2, '.').collect();
    if parts.len() == 1 {
        parts.push("");  // Add an empty string for the extension part
    }
    let base_name = parts.remove(0);
    println!("{}",base_name);
    format!("{}_doc.md", base_name)
}
