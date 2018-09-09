use model::class::Class;
use model::enumeration::Enumeration;
use model::interface::Interface;

/// Struct representing all the project data
pub struct Project {
    pub classes: Vec<Class>,
    pub interfaces: Vec<Interface>,
    pub enumerations: Vec<Enumeration>,
}

impl Project {
    pub fn new() -> Project {
        Project {
            classes: Vec::new(),
            interfaces: Vec::new(),
            enumerations: Vec::new(),
        }
    }
    pub fn add_class(&mut self, value: Class) {
        self.classes.push(value);
    }
    pub fn add_interface(&mut self, value: Interface) {
        self.interfaces.push(value);
    }
    pub fn add_enumeration(&mut self, value: Enumeration) {
        self.enumerations.push(value);
    }
}
