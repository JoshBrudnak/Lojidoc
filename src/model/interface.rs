use model::member::Member;
use model::method::Method;

#[derive(Debug)]
/// Struct containing interface documentation information
/// Includes package name, imports, method templates, and other data
pub struct Interface {
    pub package_name: String,
    pub deprecation: String,
    pub access: String,
    pub file_path: String,
    pub version: String,
    pub author: String,
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub variables: Vec<Member>,
    pub methods: Vec<Method>,
}

impl Interface {
    pub fn clone(&mut self) -> Interface {
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
            dependencies: self.dependencies.clone(),
            deprecation: self.deprecation.clone(),
            access: self.access.clone(),
            file_path: self.access.clone(),
            version: self.version.clone(),
            author: self.author.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            variables: new_variables,
            methods: new_methods,
        }
    }
    pub fn ch_file_path(&mut self, value: String) {
        self.file_path = value;
    }
}
