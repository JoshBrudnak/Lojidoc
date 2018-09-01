use model::exception::Exception;
use model::param::Param;

/// Struct representing data contained in javadoc comments
#[derive(Debug)]
pub struct Doc {
    pub params: Vec<Param>,
    pub description: String,
    pub author: String,
    pub version: String,
    pub exceptions: Vec<Exception>,
    pub deprecated: String,
    pub return_desc: String,
}

impl Doc {
    pub fn new() -> Doc {
        Doc {
            params: Vec::new(),
            description: String::new(),
            return_desc: String::new(),
            author: String::new(),
            version: String::new(),
            exceptions: Vec::new(),
            deprecated: String::new(),
        }
    }
    pub fn clear(&mut self) {
        self.params = Vec::new();
        self.description = String::new();
        self.return_desc = String::new();
        self.author = String::new();
        self.version = String::new();
        self.exceptions = Vec::new();
        self.deprecated = String::new();
    }
}
