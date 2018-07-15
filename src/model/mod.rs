pub mod model {
    //! Module that contains all necessary data stuctures for parsing javadocs and generating docs
    #[derive(Debug)]
    /// Struct representing method parameter data contained in javadoc and method declaration
    pub struct Param {
        pub desc: String,
        pub name: String,
        pub var_type: String,
    }

    /// Struct representing data contained in javadoc comments
    pub struct Doc {
        pub params: Vec<Param>,
        pub description: String,
        pub author: String,
        pub version: String,
        pub exceptions: Vec<String>,
        pub deprecated: String,
        pub return_desc: String,
    }

    #[derive(Debug)]
    /// Struct containing method data from the javadoc and method declaration
    pub struct Method {
        pub parameters: Vec<Param>,
        pub name: String,
        pub privacy: String,
        pub exceptions: Vec<String>,
        pub description: String,
        pub return_type: String,
    }

    #[derive(Debug)]
    /// Struct containing class documentation information
    /// Includes package name, imports, methods, and other data
    pub struct Class {
        pub package_name: String,
        pub deprecation: String,
        pub access: String,
        pub version: String,
        pub author: String,
        pub class_name: String,
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
        pub fn ch_privacy(&mut self, value: String) {
            self.privacy = value;
        }
        pub fn ch_method_name(&mut self, value: String) {
            self.name = value;
        }
        pub fn ch_description(&mut self, value: String) {
            self.description = value;
        }
        pub fn add_exception(&mut self, value: String) {
            self.exceptions.push(value);
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

    /// Enum that is used to determine the line type for each line
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

    pub enum ParseError {
        NoError,
        NotMethod,
        IncorrectSyntax,
    }

    /// Used for handling method parsing results
    pub type MethodResult = Result<Method, ParseError>;
}
