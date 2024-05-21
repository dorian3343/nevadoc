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

