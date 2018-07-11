#[derive(Debug)]
pub struct Param {
    pub desc: String,
    pub name: String,
}

pub struct Doc {
    pub params: Vec<Param>,
    pub description: String,
    pub return_desc: String,
}

#[derive(Debug)]
pub struct Method {
    pub parameters: Vec<Param>,
    pub name: String,
    pub privacy: String,
    pub description: String,
    pub return_type: String,
}

#[derive(Debug)]
pub struct Class {
    pub package_name: String,
    pub access: String,
    pub class_name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub methods: Vec<Method>,
}

pub struct ParseState {
    pub class: bool,
    pub method: bool,
    pub doc: bool,
    pub doc_ready: bool,
    pub block_depth: i32,
}

impl Class {
    pub fn ch_access(&mut self, value: String) {
        self.access = value;
    }
    pub fn ch_package_name(&mut self, value: String) {
        self.package_name = value;
    }
    pub fn ch_class_name(&mut self, value: String) {
        self.class_name = value;
    }
    pub fn ch_description(&mut self, value: String) {
        self.description = value;
    }
    pub fn add_method(&mut self, value: Method) {
        self.methods.push(value);
    }
    pub fn add_dependency(&mut self, value: String) {
        self.dependencies.push(value);
    }
}

impl Method {
    pub fn ch_privacy(&mut self, value: String) {
        self.privacy = value;
    }
    pub fn ch_method_name(&mut self, value: String) {
        self.name = value;
    }
    pub fn ch_description(&mut self, value: String) {
        self.description = value;
    }
    pub fn add_param(&mut self, value: Param) {
        self.parameters.push(value);
    }
    pub fn ch_return_type(&mut self, value: String) {
        self.return_type = value;
    }
}

impl ParseState {
    pub fn ch_class(&mut self, value: bool) {
        self.class = value;
    }
    pub fn ch_method(&mut self, value: bool) {
        self.method = value;
    }
    pub fn ch_doc(&mut self, value: bool) {
        self.doc = value;
    }
    pub fn ch_doc_ready(&mut self, value: bool) {
        self.doc_ready = value;
    }
    pub fn increase_depth(&mut self) {
        self.block_depth = self.block_depth + 1;
    }
    pub fn decrease_depth(&mut self) {
        if self.block_depth > 0 {
            self.block_depth = self.block_depth - 1;
        } else {
            println!("syntax error extra bracket");
        }
    }
}

impl Param {
    pub fn clone(&mut self) -> Param {
        let new_desc = self.desc.clone();
        let new_name = self.name.clone();

        Param {
            desc: new_desc,
            name: new_name,
        }
    }
}

pub enum LineType {
    IsPackage,
    IsImport,
    IsClass,
    IsMethod,
    IsComment,
    IsStartdoc,
    IsEnddoc,
    IsOther,
}
