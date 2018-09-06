#[macro_use]
pub mod grammar {
    //! Module that contains grammar used in the lexing and parsing of java code
    //! Also defines other sets of keywords like javadoc keywords or framework annotations

    #[derive(Clone, Debug, PartialEq)]
    pub enum Token {
        Symbol(String),
        Keyword(String),
        Join,
        ParamStart,
        ParamEnd,
        LineNumber(String),
        ExpressionEnd(String),
    }

    /// Gets a full list of all the keywords for the lexer
    pub fn get_keywords<'a>() -> Vec<&'a str> {
        vec![
            "abstract",
            "class",
            "const",
            "default",
            "enum",
            "extends",
            "final",
            "implements",
            "import",
            "instanseof",
            "interface",
            "native",
            "package",
            "public",
            "private",
            "protected",
            "return",
            "static",
            "strictfp",
            "super",
            "switch",
            "synchronized",
            "this",
            "throw",
            "throws",
            "transient",
            "volatile",
        ]
    }

    /// Gets a full list of all the javadoc keywords for the lexer
    pub fn get_jdoc_keywords<'a>() -> Vec<&'a str> {
        vec![
            "@return",
            "@param",
            "@author",
            "@code",
            "@deprecated",
            "@docRoot",
            "@exception",
            "@inheritDoc",
            "@link",
            "@linkplain",
            "@literal",
            "@see",
            "@throws",
            "@since",
            "@serialData",
            "@serialField",
            "@value",
            "@version",
        ]
    }

    #[derive(Debug, Clone)]
    pub enum Stream {
        Import,
        Package,
        Exception,
        Implement,
        Parent,
        Object(String),
        Access(String),
        Modifier(String),
        Type(String),
        Variable(String),
    }

    #[derive(Clone, Debug)]
    pub enum JdocToken {
        Keyword(String),
        Symbol(String),
    }

    /// Stores the state of javadoc parsing. Each enum field represents a javadoc
    /// keyword defined in the `get_jdoc_keywords()` function.
    #[derive(Clone, Debug)]
    pub enum JdocState {
        Desc,
        JdocReturn,
        Param,
        Author,
        Code,
        Deprecated,
        DocRoot,
        Exception,
        InheritDoc,
        Link,
        Linkplain,
        Literal,
        See,
        Since,
        SerialData,
        SerialField,
        Value,
        Version,
    }

    /// Struct that represents the parsing state of the high level java declarations
    pub struct ParseState {
        pub class: bool,
        pub interface: bool,
        pub enum_ob: bool,
        pub doc: bool,
        pub comment: bool,
        pub doc_ready: bool,
    }

    impl ParseState {
        pub fn new() -> ParseState {
            ParseState {
                class: false,
                interface: false,
                enum_ob: false,
                doc: false,
                comment: false,
                doc_ready: false,
            }
        }
        pub fn ch_class(&mut self, value: bool) {
            self.class = value;
        }
        pub fn ch_interface(&mut self, value: bool) {
            self.interface = value;
        }
    }
}
