use model::exception::Exception;
use model::member::Member;
use model::method::Method;

#[derive(Debug)]
pub struct EnumField {
    pub name: String,
    pub value: String,
}

impl EnumField {
    pub fn clone(&self) -> EnumField {
        EnumField {
            name: self.name.clone(),
            value: self.value.clone(),
        }
    }
}

#[derive(Debug)]
/// Struct containing enumeration documentation information
/// Includes package name, imports, methods, and other data
pub struct Enumeration {
    pub file_path: String,
    pub package_name: String,
    pub deprecation: String,
    pub license: String,
    pub access: String,
    pub version: String,
    pub author: String,
    pub name: String,
    pub description: String,
    pub exceptions: Vec<Exception>,
    pub interfaces: Vec<String>,
    pub dependencies: Vec<String>,
    pub modifiers: Vec<String>,
    pub fields: Vec<EnumField>,
    pub methods: Vec<Method>,
    pub variables: Vec<Member>,
}

impl Enumeration {
    pub fn clone(&mut self) -> Enumeration {
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
            modifiers: new_mods,
            fields: new_fields,
            variables: new_vars,
            methods: new_methods,
        }
    }
    pub fn ch_file_path(&mut self, value: String) {
        self.file_path = value;
    }
}
