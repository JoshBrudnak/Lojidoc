pub mod class;
pub mod doc;
pub mod exception;
pub mod interface;
pub mod member;
pub mod method;
pub mod param;
pub mod project;
pub mod enumeration;
pub mod object;

pub mod model {
    //! Module that contains all necessary data stuctures for parsing javadocs and generating docs

    pub use model::class::Class;
    pub use model::enumeration::Enumeration;
    pub use model::doc::Doc;
    pub use model::exception::Exception;
    pub use model::interface::Interface;
    pub use model::member::Member;
    pub use model::method::Method;
    pub use model::param::Param;
    pub use model::project::Project;
    pub use model::object::Object;
    pub use model::object::ObjectState;

    pub enum ObjectType {
        Class(Class),
        Interface(Interface),
        Enumeration(Enumeration),
    }
}

#[cfg(test)]
mod test;
