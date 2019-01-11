#[derive(Debug)]
/// Struct representing member variable data contained in javadoc and declaration
pub struct Member {
    pub line_num: String,
    pub signature: String,
    pub desc: String,
    pub modifiers: Vec<String>,
    pub access: String,
    pub name: String,
    pub var_type: String,
}

impl Member {
    pub fn clone(&self) -> Member {
        Member {
            line_num: self.line_num.clone(),
            signature: self.signature.clone(),
            desc: self.desc.clone(),
            access: self.access.clone(),
            name: self.name.clone(),
            modifiers: self.modifiers.clone(),
            var_type: self.var_type.clone(),
        }
    }
    pub fn new() -> Member {
        Member {
            line_num: String::new(),
            signature: String::new(),
            desc: String::new(),
            access: String::new(),
            name: String::new(),
            modifiers: Vec::new(),
            var_type: String::new(),
        }
    }
    pub fn ch_name(&mut self, value: String) {
        self.name = value;
    }
    pub fn ch_signature(&mut self, value: String) {
        self.signature = value;
    }
    pub fn ch_access(&mut self, value: String) {
        self.access = value;
    }
    pub fn ch_type(&mut self, value: String) {
        self.var_type = value;
    }
    pub fn add_modifier(&mut self, value: String) {
        self.modifiers.push(value);
    }
    pub fn ch_line_number(&mut self, value: String) {
        self.line_num = value;
    }
}
