pub mod model {
    //! Module that contains all necessary data stuctures for parsing javadocs and generating docs

    /// Gets a full list of all the keywords for the lexer
    pub fn get_keywords<'a>() -> Vec<&'a str> {
        vec![
            "abstract",
            "class",
            "const",
            "default",
            "else",
            "if",
            "enum",
            "extends",
            "final",
            "for",
            "implements",
            "import",
            "instanseof",
            "interface",
            "native",
            "new",
            "package",
            "public",
            "private",
            "protected",
            "return",
            "static",
            "strictfp",
            "super",
            "switch",
            "synchronized",
            "this",
            "throw",
            "throws",
            "transient",
            "try",
            "void",
            "volatile",
            "while",
        ]
    }

    /// Gets a full list of all the javadoc keywords for the lexer
    pub fn get_jdoc_keywords<'a>() -> Vec<&'a str> {
        vec![
            "@return",
            "@param",
            "@author",
            "@code",
            "@deprecated",
            "@docRoot",
            "@exception",
            "@inheritDoc",
            "@link",
            "@linkplain",
            "@literal",
            "@see",
            "@throws",
            "@since",
            "@serialData",
            "@serialField",
            "@value",
            "@version",
        ]
    }

    #[derive(Debug)]
    /// Struct representing method parameter data contained in javadoc and method declaration
    pub struct Param {
        pub desc: String,
        pub name: String,
        pub var_type: String,
    }

    #[derive(Debug)]
    pub enum Token {
        symbol(String),
        keyword(String),
        doc_keyword(String),
        expression_end(String),
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
        pub line_num: String,
        pub parameters: Vec<Param>,
        pub is_static: bool,
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
        pub exception: Exception,
        pub interfaces: Vec<String>,
        pub dependencies: Vec<String>,
        pub modifiers: Vec<String>,
        pub methods: Vec<Method>,
        pub variables: Vec<Param>,
    }

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
        pub methods: Vec<Method>,
    }

    /// Struct that represents the parsing state
    pub struct ParseState {
        pub class: bool,
        pub method: bool,
        pub doc: bool,
        pub comment: bool,
        pub doc_ready: bool,
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
                exception: Exception::new(),
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

            for i in 0..self.methods.len() {
                new_methods.push(self.methods[i].clone());
            }
            for i in 0..self.variables.len() {
                new_vars.push(self.variables[i].clone());
            }
            for i in 0..self.variables.len() {
                new_mods.push(self.modifiers[i].clone());
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
                exception: self.exception.clone(),
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
        pub fn add_interface(&mut self, value: String) {
            self.interfaces.push(value);
        }
        pub fn add_modifier(&mut self, value: String) {
            self.modifiers.push(value);
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
                file_path: String::new(),
                version: String::new(),
                author: String::new(),
                name: String::new(),
                description: String::new(),
                methods: Vec::new(),
            }
        }
        pub fn clone(&mut self) -> Interface {
            let mut new_methods = Vec::new();

            for i in 0..self.methods.len() {
                new_methods.push(self.methods[i].clone());
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
                methods: new_methods,
            }
        }
        pub fn ch_access(&mut self, value: String) {
            self.access = value;
        }
        pub fn ch_file_path(&mut self, value: String) {
            self.file_path = value;
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
                is_static: false,
                exception: Exception::new(),
                line_num: String::new(),
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
                line_num: self.line_num.clone(),
                parameters: new_params,
                is_static: self.is_static,
                exception: self.exception.clone(),
                name: self.name.clone(),
                static_meth: self.static_meth.clone(),
                privacy: self.privacy.clone(),
                description: self.description.clone(),
                return_type: self.return_type.clone(),
            }
        }
        pub fn clone_params(&mut self) -> Vec<Param> {
            let mut new_params = Vec::new();

            for i in 0..self.parameters.len() {
                new_params.push(self.parameters[i].clone());
            }

            new_params
        }
        pub fn ch_line_num(&mut self, value: String) {
            self.line_num = value;
        }
        pub fn ch_privacy(&mut self, value: String) {
            self.privacy = value;
        }
        pub fn ch_is_static(&mut self, value: bool) {
            self.is_static = value;
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
                method: false,
                doc: false,
                comment: false,
                doc_ready: false,
            }
        }
        pub fn ch_class(&mut self, value: bool) {
            self.class = value;
        }
        pub fn ch_method(&mut self, value: bool) {
            self.method = value;
        }
        pub fn ch_doc(&mut self, value: bool) {
            self.doc = value;
        }
        pub fn ch_comment(&mut self, value: bool) {
            self.comment = value;
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
                description: String::new(),
                return_desc: String::new(),
                author: String::new(),
                version: String::new(),
                exception: Exception::new(),
                deprecated: String::new(),
            }
        }
        pub fn clear(&mut self) {
            self.params = Vec::new();
            self.description = String::new();
            self.return_desc = String::new();
            self.author = String::new();
            self.version = String::new();
            self.exception = Exception::new();
            self.deprecated = String::new();
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
    #[derive(PartialEq, Debug)]
    pub enum LineType {
        IsPackage,
        IsImport,
        IsVariable,
        IsClass,
        IsInterface,
        IsMethod,
        IsComment,
        IsStartdoc,
        IsEnddoc,
        IsOther,
    }
}
