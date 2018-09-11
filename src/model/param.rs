#[derive(Debug, Clone)]
/// Struct representing method parameter data contained in javadoc and method declaration
pub struct Param {
    pub desc: String,
    pub name: String,
    pub var_type: String,
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
}
