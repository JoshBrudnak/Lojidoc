#[macro_use]
pub mod grammar {
    //! Module that contains grammar used in the lexing and parsing of java code
    //! Also defines other sets of keywords like javadoc keywords or framework annotations
    //!
    //! This module uses the following grammar:
    //!
    //! dec := access {mod}* ident sym
    //! ident := sym | sym term {dec term}*
    //! term := ( | ) | , | < | >
    //! expr_end := ; | { | =
    //! sym := {a-Z | 0-9}*
    //! except := throws sym | throws {sym}*
    //! impl := implements sym | implements {sym}*
    //! paren := extends sym
    //!
    //! package_dec := package sym expr_end
    //! import_dec := import sym expr_end | import static sym expr_end
    //! method_dec := dec term {dec term}* expr_end
    //! var_dec := dec term expr_end
    //! class_dec := access {mod}* class sym {paren} {impl}* {except}* expr_end
    //! inter_dec := access {mod}* interface sym {except}* expr_end

    #[derive(Clone, Debug, PartialEq)]
    pub enum Token {
        Symbol(String),
        Keyword(String),
        Sign(String),
        Join,
        ParamStart,
        ParamEnd,
        LineNumber(String),
        Spring(String),
        ExpressionEnd(String),
    }

    /// Gets a full list of all the keywords for the lexer
    pub fn get_keywords() -> Vec<&'static str> {
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
            "instanceof",
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

    pub fn get_spring_keywords<'a>() -> Vec<&'a str> {
        vec![
            "Autowired",
            "Data",
            "Controller",
            "Service",
            "Component",
            "RequestMapping",
            "GetMapping",
            "PostMapping",
            "PutMapping",
            "DeleteMapping",
            "PatchMapping",
            "Value",
            "Bean",
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
    pub enum ParseState {
        Class,
        Interface,
        Enum,
        Other,
    }
}
