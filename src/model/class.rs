use model::exception::Exception;
use model::interface::Interface;
use model::member::Member;
use model::method::Method;

#[derive(Debug)]
/// Struct containing class documentation information
/// Includes package name, imports, methods, and other data
pub struct Class {
    pub is_class: bool,
    pub file_path: String,
    pub package_name: String,
    pub deprecation: String,
    pub license: String,
    pub parent: String,
    pub access: String,
    pub version: String,
    pub author: String,
    pub class_name: String,
    pub description: String,
    pub exceptions: Vec<Exception>,
    pub interfaces: Vec<String>,
    pub dependencies: Vec<String>,
    pub modifiers: Vec<String>,
    pub methods: Vec<Method>,
    pub variables: Vec<Member>,
}

impl Class {
    pub fn new() -> Class {
        Class {
            is_class: true,
            package_name: String::new(),
            file_path: String::new(),
            dependencies: Vec::new(),
            deprecation: String::new(),
            license: String::new(),
            parent: String::new(),
            interfaces: Vec::new(),
            access: String::new(),
            version: String::new(),
            author: String::new(),
            class_name: String::new(),
            exceptions: Vec::new(),
            description: String::new(),
            modifiers: Vec::new(),
            variables: Vec::new(),
            methods: Vec::new(),
        }
    }
    pub fn clone(&mut self) -> Class {
        let mut new_methods = Vec::new();
        let mut new_vars = Vec::new();
        let mut new_mods = Vec::new();
        let mut new_except = Vec::new();

        for i in 0..self.methods.len() {
            new_methods.push(self.methods[i].clone());
        }
        for i in 0..self.variables.len() {
            new_vars.push(self.variables[i].clone());
        }
        for i in 0..self.variables.len() {
            new_mods.push(self.modifiers[i].clone());
        }
        for i in 0..self.exceptions.len() {
            new_except.push(self.exceptions[i].clone());
        }

        Class {
            is_class: self.is_class.clone(),
            parent: self.parent.clone(),
            file_path: self.file_path.clone(),
            package_name: self.package_name.clone(),
            license: self.license.clone(),
            dependencies: self.dependencies.clone(),
            deprecation: self.deprecation.clone(),
            access: self.access.clone(),
            version: self.version.clone(),
            author: self.author.clone(),
            class_name: self.class_name.clone(),
            description: self.description.clone(),
            exceptions: new_except,
            interfaces: self.interfaces.clone(),
            modifiers: new_mods,
            variables: new_vars,
            methods: new_methods,
        }
    }
    pub fn to_interface(&mut self) -> Interface {
        let mut new_methods = Vec::new();

        for i in 0..self.methods.len() {
            new_methods.push(self.methods[i].clone());
        }

        Interface {
            package_name: self.package_name.clone(),
            dependencies: self.dependencies.clone(),
            deprecation: self.deprecation.clone(),
            access: self.access.clone(),
            file_path: self.file_path.clone(),
            version: self.version.clone(),
            author: self.author.clone(),
            name: self.class_name.clone(),
            description: self.description.clone(),
            methods: new_methods,
        }
    }
    pub fn ch_access(&mut self, value: String) {
        self.access = value;
    }
    pub fn ch_license(&mut self, value: String) {
        self.file_path = value;
    }
    pub fn ch_file_path(&mut self, value: String) {
        self.file_path = value;
    }
    pub fn ch_is_class(&mut self, value: bool) {
        self.is_class = value;
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
    pub fn ch_deprecation(&mut self, value: String) {
        self.deprecation = value;
    }
    pub fn ch_parent(&mut self, value: String) {
        self.parent = value;
    }
    pub fn ch_version(&mut self, value: String) {
        self.deprecation = value;
    }
    pub fn ch_author(&mut self, value: String) {
        self.author = value;
    }
    pub fn add_method(&mut self, value: Method) {
        self.methods.push(value);
    }
    pub fn add_dependency(&mut self, value: String) {
        self.dependencies.push(value);
    }
    pub fn add_variable(&mut self, value: Member) {
        self.variables.push(value);
    }
    pub fn add_interface(&mut self, value: String) {
        self.interfaces.push(value);
    }
    pub fn add_modifier(&mut self, value: String) {
        self.modifiers.push(value);
    }
    pub fn add_exception(&mut self, value: Exception) {
        self.exceptions.push(value);
    }
}
