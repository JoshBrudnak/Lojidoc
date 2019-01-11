use model::class::Class;
use model::enumeration::EnumField;
use model::enumeration::Enumeration;
use model::exception::Exception;
use model::interface::Interface;
use model::member::Member;
use model::method::Method;

#[derive(Debug, Clone)]
pub enum ObjectState {
    Class,
    Interface,
    Enumeration,
    Unset,
}

#[derive(Debug)]
/// Struct containing class documentation information
/// Includes package name, imports, methods, and other data
pub struct Object {
    pub state: ObjectState,
    pub file_path: String,
    pub signature: String,
    pub package_name: String,
    pub deprecation: String,
    pub license: String,
    pub parent: String,
    pub access: String,
    pub version: String,
    pub author: String,
    pub name: String,
    pub description: String,
    pub exceptions: Vec<Exception>,
    pub interfaces: Vec<String>,
    pub dependencies: Vec<String>,
    pub fields: Vec<EnumField>,
    pub modifiers: Vec<String>,
    pub methods: Vec<Method>,
    pub variables: Vec<Member>,
}

impl Object {
    pub fn new() -> Object {
        Object {
            state: ObjectState::Unset,
            package_name: String::new(),
            file_path: String::new(),
            signature: String::new(),
            dependencies: Vec::new(),
            deprecation: String::new(),
            license: String::new(),
            parent: String::new(),
            interfaces: Vec::new(),
            access: String::new(),
            version: String::new(),
            author: String::new(),
            name: String::new(),
            exceptions: Vec::new(),
            description: String::new(),
            fields: Vec::new(),
            modifiers: Vec::new(),
            variables: Vec::new(),
            methods: Vec::new(),
        }
    }
    pub fn to_class(&mut self) -> Class {
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
        for i in 0..self.modifiers.len() {
            new_mods.push(self.modifiers[i].clone());
        }
        for i in 0..self.exceptions.len() {
            new_except.push(self.exceptions[i].clone());
        }

        Class {
            parent: self.parent.clone(),
            file_path: self.file_path.clone(),
            signature: self.signature.clone(),
            package_name: self.package_name.clone(),
            license: self.license.clone(),
            dependencies: self.dependencies.clone(),
            deprecation: self.deprecation.clone(),
            access: self.access.clone(),
            version: self.version.clone(),
            author: self.author.clone(),
            name: self.name.clone(),
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
        let mut new_variables = Vec::new();

        for i in 0..self.methods.len() {
            new_methods.push(self.methods[i].clone());
        }
        for i in 0..self.variables.len() {
            new_variables.push(self.variables[i].clone());
        }

        Interface {
            package_name: self.package_name.clone(),
            signature: self.signature.clone(),
            dependencies: self.dependencies.clone(),
            deprecation: self.deprecation.clone(),
            access: self.access.clone(),
            file_path: self.file_path.clone(),
            version: self.version.clone(),
            author: self.author.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            variables: new_variables,
            methods: new_methods,
        }
    }
    pub fn to_enumeration(&mut self) -> Enumeration {
        let mut new_methods = Vec::new();
        let mut new_vars = Vec::new();
        let mut new_mods = Vec::new();
        let mut new_except = Vec::new();
        let mut new_fields = Vec::new();

        for i in 0..self.methods.len() {
            new_methods.push(self.methods[i].clone());
        }
        for i in 0..self.variables.len() {
            new_vars.push(self.variables[i].clone());
        }
        for i in 0..self.modifiers.len() {
            new_mods.push(self.modifiers[i].clone());
        }
        for i in 0..self.exceptions.len() {
            new_except.push(self.exceptions[i].clone());
        }
        for i in 0..self.fields.len() {
            new_fields.push(self.fields[i].clone());
        }

        Enumeration {
            file_path: self.file_path.clone(),
            package_name: self.package_name.clone(),
            license: self.license.clone(),
            dependencies: self.dependencies.clone(),
            deprecation: self.deprecation.clone(),
            access: self.access.clone(),
            version: self.version.clone(),
            author: self.author.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            exceptions: new_except,
            interfaces: self.interfaces.clone(),
            fields: new_fields,
            modifiers: new_mods,
            variables: new_vars,
            methods: new_methods,
        }
    }
    pub fn ch_access(&mut self, value: String) {
        self.access = value;
    }
    pub fn ch_signature(&mut self, value: String) {
        self.signature = value;
    }
    pub fn ch_state(&mut self, value: ObjectState) {
        self.state = value;
    }
    pub fn ch_license(&mut self, value: String) {
        self.license = value;
    }
    pub fn ch_package_name(&mut self, value: String) {
        self.package_name = value;
    }
    pub fn ch_name(&mut self, value: String) {
        self.name = value;
    }
    pub fn ch_description(&mut self, value: String) {
        self.description = value;
    }
    pub fn ch_fields(&mut self, value: Vec<EnumField>) {
        self.fields = value;
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
