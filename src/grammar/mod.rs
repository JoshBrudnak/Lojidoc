#[macro_use]
pub mod grammar {
    //! Module that contains grammar

    #[derive(Debug)]
    pub enum Token {
        symbol(String),
        keyword(String),
        doc_keyword(String),
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

    pub enum method_grammer {
        Access(String),
        Modifier(String),
        Parameter(String),
        Return_type(String),
    }

    macro_rules! access_mod_match {
        ($e:expr) => {
            match $e {
                Token::keyword(value) => match value.as_ref() {
                    "public" | "protected" | "private" => true,
                    _ => false,
                },
                _ => false,
            }
        };
    }

    macro_rules! modifier_match {
        ($e:expr) => {
            match $e {
                Token::keyword(value) => match value.as_ref() {
                    "static" | "final" | "abstract" | "synchronized" | "volatile" => true,
                    _ => false,
                },
                _ => false,
            }
        };
    }

    pub fn match_method(token_stream: Vec<Token>) -> bool {
        let method_prod: Vec<method_grammer> = Vec::new();

        for token in token_stream {
            if access_mod_match!(token) {
                method_prod.push(method_grammer::Access(token));
            } else if modifier_match!(token) {
                method_prod.push(method_grammer::Modifier(token));
            }
        }

        true
    }

    pub fn temp_fn() {
        access_mod_match!(Token::keyword("public".to_string()));
        modifier_match!(Token::keyword("public".to_string()));
    }
}
