#[macro_use]
pub mod grammar {
    //! Module that contains grammar

    #[derive(Clone, Debug)]
    pub enum Token {
        symbol(String),
        keyword(String),
        doc_keyword(String),
        join,
        param_start,
        param_end,
        expression_end(String),
    }

    /// Gets a full list of all the keywords for the lexer
    pub fn get_keywords<'a>() -> Vec<&'a str> {
        vec![
            "abstract",
            "class",
            "const",
            "default",
            "else",
            "if",
            "enum",
            "extends",
            "final",
            "for",
            "implements",
            "import",
            "instanseof",
            "interface",
            "native",
            "new",
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
            "try",
            "void",
            "volatile",
            "while",
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

    #[derive(Clone)]
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
        Return_type(String),
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

    /*
    pub fn match_method(token_stream: Vec<Token>) -> bool {
        let mut method_prod: Vec<method_grammer> = Vec::new();

        for token in token_stream.clone() {
            if access_mod_match!(token.clone()) {
                match token {
                    Token::keyword(key) => method_prod.push(method_grammer::Access(key)),
                    _ => println!("Parse error with access modifiers"),
                }
            } else if modifier_match!(token.clone()) {
                match token {
                    Token::keyword(key) => method_prod.push(method_grammer::Modifier(key)),
                    _ => println!("Parse error with modifiers"),
                }
            }
        }

        true
    }

    pub fn temp_fn() {
        access_mod_match!(Token::keyword("public".to_string()));
        modifier_match!(Token::keyword("public".to_string()));
    }
    */
}
