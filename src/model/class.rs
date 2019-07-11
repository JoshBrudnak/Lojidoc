use model::exception::Exception;
use model::member::Member;
use model::method::Method;
use model::spring_data::SpringClass;

#[derive(Debug)]
/// Struct containing class documentation information
/// Includes package name, imports, methods, and other data
pub struct Class {
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
    pub spring_data: SpringClass,
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
            spring_data: SpringClass::new(),
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
            spring_data: self.spring_data.clone(),
            description: self.description.clone(),
            exceptions: new_except,
            interfaces: self.interfaces.clone(),
            modifiers: new_mods,
            variables: new_vars,
            methods: new_methods,
        }
    }
    pub fn ch_file_path(&mut self, value: String) {
        self.file_path = value;
    }
}
