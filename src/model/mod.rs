pub mod model {
    //! Module that contains all necessary data stuctures for parsing javadocs and generating docs
    #[derive(Debug)]
    /// Struct representing method parameter data contained in javadoc and method declaration
    pub struct Param {
        pub desc: String,
        pub name: String,
        pub var_type: String,
    }

    /// Struct representing all the project data
    pub struct Project {
        pub classes: Vec<Class>,
        pub interfaces: Vec<Interface>,
    }

    #[derive(Debug)]
    /// Struct representing method parameter data contained in javadoc and method declaration
    pub struct Exception {
        pub exception_type: String,
        pub desc: String,
    }

    /// Struct representing data contained in javadoc comments
    pub struct Doc {
        pub params: Vec<Param>,
        pub description: String,
        pub author: String,
        pub version: String,
        pub exception: Exception,
        pub deprecated: String,
        pub return_desc: String,
    }

    #[derive(Debug)]
    /// Struct containing method data from the javadoc and method declaration
    pub struct Method {
        pub parameters: Vec<Param>,
        pub name: String,
        pub static_meth: bool,
        pub privacy: String,
        pub description: String,
        pub exception: Exception,
        pub return_type: String,
    }

    #[derive(Debug)]
    /// Struct containing class documentation information
    /// Includes package name, imports, methods, and other data
    pub struct Class {
        pub is_class: bool,
        pub package_name: String,
        pub deprecation: String,
        pub parent: String,
        pub access: String,
        pub version: String,
        pub author: String,
        pub class_name: String,
        pub description: String,
        pub exception: Exception,
        pub interfaces: Vec<String>,
        pub dependencies: Vec<String>,
        pub methods: Vec<Method>,
    }

    #[derive(Debug)]
    /// Struct containing interface documentation information
    /// Includes package name, imports, method templates, and other data
    pub struct Interface {
        pub package_name: String,
        pub deprecation: String,
        pub access: String,
        pub version: String,
        pub author: String,
        pub name: String,
        pub description: String,
        pub dependencies: Vec<String>,
        pub methods: Vec<Method>,
    }

    /// Struct that represents the parsing state
    pub struct ParseState {
        pub class: bool,
        pub interface: bool,
        pub method: bool,
        pub doc: bool,
        pub comment: bool,
        pub doc_ready: bool,
        pub block_depth: i32,
    }

    impl Exception {
        pub fn new() -> Exception {
            Exception {
                exception_type: String::new(),
                desc: String::new(),
            }
        }
        pub fn is_empty(&self) -> bool {
            if self.exception_type != "" && self.desc != "" {
                false
            } else {
                true
            }
        }
        pub fn clone(&self) -> Exception {
            Exception {
                exception_type: self.exception_type.clone(),
                desc: self.desc.clone(),
            }
        }
    }

    impl Class {
        pub fn new() -> Class {
            Class {
                is_class: true,
                package_name: String::new(),
                dependencies: Vec::new(),
                deprecation: String::new(),
                parent: String::new(),
                interfaces: Vec::new(),
                access: String::new(),
                version: String::new(),
                author: String::new(),
                class_name: String::new(),
                exception: Exception::new(),
                description: String::new(),
                methods: Vec::new(),
            }
        }
        pub fn clone(&mut self) -> Class {
            let mut new_methods = Vec::new();

            for i in 0..self.methods.len() {
                new_methods.push(self.methods[i].clone());
            }

            Class {
                is_class: self.is_class.clone(),
                parent: self.parent.clone(),
                package_name: self.package_name.clone(),
                dependencies: self.dependencies.clone(),
                deprecation: self.deprecation.clone(),
                access: self.access.clone(),
                version: self.version.clone(),
                author: self.author.clone(),
                class_name: self.class_name.clone(),
                description: self.description.clone(),
                exception: self.exception.clone(),
                interfaces: self.interfaces.clone(),
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
        pub fn add_interface(&mut self, value: String) {
            self.interfaces.push(value);
        }
        pub fn ch_exception(&mut self, value: Exception) {
            self.exception = value;
        }
    }

    impl Interface {
        pub fn new() -> Interface {
            Interface {
                package_name: String::new(),
                dependencies: Vec::new(),
                deprecation: String::new(),
                access: String::new(),
                version: String::new(),
                author: String::new(),
                name: String::new(),
                description: String::new(),
                methods: Vec::new(),
            }
        }
        pub fn ch_access(&mut self, value: String) {
            self.access = value;
        }
        pub fn ch_package_name(&mut self, value: String) {
            self.package_name = value;
        }
        pub fn ch_inter_name(&mut self, value: String) {
            self.name = value;
        }
        pub fn ch_description(&mut self, value: String) {
            self.description = value;
        }
        pub fn ch_deprecation(&mut self, value: String) {
            self.deprecation = value;
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
    }

    impl Method {
        pub fn new() -> Method {
            Method {
                parameters: Vec::new(),
                exception: Exception::new(),
                name: String::new(),
                static_meth: false,
                privacy: String::new(),
                description: String::new(),
                return_type: String::new(),
            }
        }
        pub fn clone(&mut self) -> Method {
            let mut new_params = Vec::new();

            for i in 0..self.parameters.len() {
                new_params.push(self.parameters[i].clone());
            }

            Method {
                parameters: new_params,
                exception: self.exception.clone(),
                name: self.name.clone(),
                static_meth: self.static_meth.clone(),
                privacy: self.privacy.clone(),
                description: self.description.clone(),
                return_type: self.return_type.clone(),
            }
        }
        pub fn ch_privacy(&mut self, value: String) {
            self.privacy = value;
        }
        pub fn ch_method_name(&mut self, value: String) {
            self.name = value;
        }
        pub fn ch_description(&mut self, value: String) {
            self.description = value;
        }
        pub fn ch_exception(&mut self, value: Exception) {
            self.exception = value;
        }
        pub fn add_param(&mut self, value: Param) {
            self.parameters.push(value);
        }
        pub fn ch_params(&mut self, value: Vec<Param>) {
            self.parameters = value;
        }
        pub fn ch_return_type(&mut self, value: String) {
            self.return_type = value;
        }
    }

    impl ParseState {
        pub fn new() -> ParseState {
            ParseState {
                class: false,
                interface: false,
                method: false,
                doc: false,
                comment: false,
                doc_ready: false,
                block_depth: 0,
            }
        }
        pub fn ch_class(&mut self, value: bool) {
            self.class = value;
        }
        pub fn ch_doc(&mut self, value: bool) {
            self.doc = value;
        }
        pub fn ch_doc_ready(&mut self, value: bool) {
            self.doc_ready = value;
        }
    }

    impl Param {
        pub fn clone(&mut self) -> Param {
            let new_desc = self.desc.clone();
            let new_name = self.name.clone();
            let new_type = self.var_type.clone();

            Param {
                desc: new_desc,
                name: new_name,
                var_type: new_type,
            }
        }
        pub fn ch_desc(&mut self, value: String) {
            self.desc = value;
        }
    }

    impl Doc {
        pub fn new() -> Doc {
            Doc {
                params: Vec::new(),
                description: String::from(""),
                return_desc: String::from(""),
                author: String::from(""),
                version: String::from(""),
                exception: Exception::new(),
                deprecated: String::from(""),
            }
        }
    }

    impl Project {
        pub fn new() -> Project {
            Project {
                classes: Vec::new(),
                interfaces: Vec::new(),
            }
        }
        pub fn add_class(&mut self, value: Class) {
            self.classes.push(value);
        }
        pub fn add_interface(&mut self, value: Interface) {
            self.interfaces.push(value);
        }
    }

    /// Enum that is used to determine the line type for each line
    pub enum LineType {
        IsPackage,
        IsImport,
        IsClass,
        IsInterface,
        IsMethod,
        IsComment,
        IsStartdoc,
        IsEnddoc,
        IsOther,
    }
}
