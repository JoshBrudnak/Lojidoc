pub mod class;
pub mod contents;
pub mod doc;
pub mod enumeration;
pub mod exception;
pub mod interface;
pub mod member;
pub mod method;
pub mod object;
pub mod project;

pub mod model {
    //! Module that contains all necessary data stuctures for parsing javadocs and generating docs

    pub use model::class::Class;
    pub use model::contents::ApplicationDoc;
    pub use model::doc::Doc;
    pub use model::enumeration::Enumeration;
    pub use model::enumeration::EnumField;
    pub use model::exception::Exception;
    pub use model::interface::Interface;
    pub use model::member::Member;
    pub use model::method::Param;
    pub use model::method::Method;
    pub use model::object::Object;
    pub use model::object::ObjectState;
    pub use model::project::Project;

    pub enum ObjectType {
        Class(Class),
        Interface(Interface),
        Enumeration(Enumeration),
    }
}

#[cfg(test)]
mod test;
