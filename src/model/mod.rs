pub mod class;
pub mod doc;
pub mod exception;
pub mod interface;
pub mod member;
pub mod method;
pub mod param;
pub mod project;

pub mod model {
    //! Module that contains all necessary data stuctures for parsing javadocs and generating docs

    pub use model::class::Class;
    pub use model::doc::Doc;
    pub use model::exception::Exception;
    pub use model::interface::Interface;
    pub use model::member::Member;
    pub use model::method::Method;
    pub use model::param::Param;
    pub use model::project::Project;
}
