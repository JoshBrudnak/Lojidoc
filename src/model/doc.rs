use model::exception::Exception;
use model::method::Param;

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
    pub see: String,
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
            see: String::new(),
        }
    }
}
