#[macro_use]
pub mod grammar {
    //! Module that contains grammar

    #[derive(Clone, Debug)]
    pub struct Ignore {
        pub is_ignore: bool,
        pub ignore_token: String,
    }

    impl Ignore {
        pub fn new() -> Ignore {
            Ignore {
                is_ignore: false,
                ignore_token: String::new(),
            }
        }
        pub fn set_ignore(&mut self, value: String) {
            self.is_ignore = true;
            self.ignore_token = value;
        }
        pub fn clear(&mut self) {
            self.is_ignore = false;
            self.ignore_token = String::new();
        }
    }

    #[derive(Clone, Debug)]
    pub enum Token {
        Symbol(String),
        Keyword(String),
        Join,
        ParamStart,
        ParamEnd,
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
        Word(String),
        Doc(String),
        Return_type(String),
    }

    #[derive(Clone, Debug)]
    pub enum JdocToken {
        Keyword(String),
        Symbol(String),
    }

    #[derive(Clone, Debug)]
    pub enum JdocState {
        Desc,
        Jdoc_return,
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
        Throws,
        Since,
        SerialData,
        SerialField,
        Value,
        Version,
    }

    /// Struct that represents the parsing state
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
        pub fn ch_enum(&mut self, value: bool) {
            self.enum_ob = value;
        }
        pub fn ch_doc(&mut self, value: bool) {
            self.doc = value;
        }
        pub fn ch_comment(&mut self, value: bool) {
            self.comment = value;
        }
        pub fn ch_doc_ready(&mut self, value: bool) {
            self.doc_ready = value;
        }
    }
}
