#[derive(Debug)]
/// Struct representing method parameter data contained in javadoc and method declaration
pub struct Exception {
    pub exception_type: String,
    pub desc: String,
}

impl Exception {
    pub fn clone(&self) -> Exception {
        Exception {
            exception_type: self.exception_type.clone(),
            desc: self.desc.clone(),
        }
    }
}
