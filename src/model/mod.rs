pub mod class;
pub mod interface;
pub mod method;
pub mod member;
pub mod param;
pub mod project;
pub mod exception;
pub mod doc;

pub mod model {
    //! Module that contains all necessary data stuctures for parsing javadocs and generating docs

    pub use model::class::Class;
    pub use model::interface::Interface;
    pub use model::exception::Exception;
    pub use model::method::Method;
    pub use model::member::Member;
    pub use model::param::Param;
    pub use model::project::Project;
    pub use model::doc::Doc;
}
