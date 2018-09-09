pub mod parse {
    //! A module which handles the parsing for java files
    extern crate colored;

    use grammar::grammar::*;
    use model::model::Class;
    use model::model::Doc;
    use model::model::Exception;
    use model::model::Member;
    use model::model::Method;
    use model::model::Param;
    use model::model::ObjectType;
    use model::model::Object;
    use model::model::ObjectState;

    use colored::*;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;
    use std::path::Path;

    /// Handles token streams for javadoc comments and returns a `Doc` struct
    /// containing the information parsed from the javadoc comment.
    ///
    /// # Arguments
    ///
    /// * `tokens` - A vector of tokens from the javadoc comment
    fn get_doc(tokens: &Vec<JdocToken>) -> Doc {
        let mut return_str = String::from("");
        let mut desc = String::from("");
        let mut parameters: Vec<Param> = Vec::new();
        let mut author = String::new();
        let mut version = String::new();
        let mut link = String::new();
        let mut deprecated = String::new();
        let mut exceptions: Vec<Exception> = Vec::new();
        let mut state = JdocState::Desc;
        let mut word_buf = String::new();

        for i in 0..tokens.len() {
            match tokens[i].clone() {
                JdocToken::Keyword(key) => {
                    let new_desc = word_buf.clone();
                    if i != 0 {
                        match state {
                            JdocState::JdocReturn => {
                                return_str = new_desc;
                            }
                            JdocState::Param => {
                                let word_parts: Vec<&str> = new_desc.split(" ").collect();

                                if word_parts.len() > 1 {
                                    parameters.push(Param {
                                        var_type: String::new(),
                                        name: word_parts[0].to_string(),
                                        desc: word_parts[1..].join(""),
                                    });
                                } else if word_parts.len() == 1 {
                                    parameters.push(Param {
                                        var_type: String::new(),
                                        name: word_parts[0].to_string(),
                                        desc: String::new(),
                                    });
                                }
                            }
                            JdocState::Author => author = new_desc,
                            JdocState::Deprecated => deprecated = new_desc,
                            JdocState::Since => version = new_desc,
                            JdocState::Link => link = new_desc,
                            JdocState::See => link = new_desc,
                            JdocState::Exception => {
                                let word_parts: Vec<&str> = new_desc.split(" ").collect();

                                if exceptions.len() > 0 {
                                    exceptions.push(Exception {
                                        exception_type: word_parts[0].to_string(),
                                        desc: word_parts[1..].join(""),
                                    });
                                }
                            }
                            JdocState::Version => version = new_desc,
                            JdocState::Desc => desc = new_desc,
                            _ => println!("Code javadoc field not supported"),
                        }

                        word_buf.clear();
                    }

                    match key.as_ref() {
                        "@return" => state = JdocState::JdocReturn,
                        "@param" => state = JdocState::Param,
                        "@author" => state = JdocState::Author,
                        "@code" => state = JdocState::Code,
                        "@deprecated" => state = JdocState::Deprecated,
                        "@docRoot" => state = JdocState::DocRoot,
                        "@exception" => state = JdocState::Exception,
                        "@inheritDoc" => state = JdocState::InheritDoc,
                        "@link" => state = JdocState::Link,
                        "@linkplain" => state = JdocState::Linkplain,
                        "@literal" => state = JdocState::Literal,
                        "@see" => state = JdocState::See,
                        "@throws" => state = JdocState::Exception,
                        "@since" => state = JdocState::Since,
                        "@serialData" => state = JdocState::SerialData,
                        "@serialField" => state = JdocState::SerialField,
                        "@value" => state = JdocState::Value,
                        "@version" => state = JdocState::Version,
                        _ => println!("Unsupported javadoc keyword used"),
                    }
                }
                JdocToken::Symbol(key) => {
                    if key != "*" {
                        word_buf.push_str(format!("{} ", key.as_str()).as_str());
                    }
                }
            }
        }

        Doc {
            params: parameters,
            description: desc,
            return_desc: return_str,
            author: author,
            version: version,
            exceptions: exceptions,
            deprecated: deprecated,
            see: link,
        }
    }

    /// Handles token streams for object declarations and modifies the `Class` struct
    /// which is passed to the function.
    ///
    /// This function is used for class, interface, and enum declarations.
    ///
    /// # Arguments
    ///
    /// * `gram_parts` - A vector of tokens from the object's declaration
    /// * `java_doc` - The java doc struct with the documentation for the class
    /// * `class` - The Class stuct to be modified with the new information
    fn get_object(gram_parts: Vec<Stream>, java_doc: &Doc, ob: &mut Object) {
        let mut implement = false;
        let mut exception = false;
        let mut parent = false;
        let mut class_name = false;

        for i in 0..gram_parts.len() {
            match gram_parts[i].clone() {
                Stream::Variable(var) => {
                    if implement {
                        ob.add_interface(var);
                    } else if exception {
                        ob.add_exception(Exception {
                            desc: String::new(),
                            exception_type: var,
                        });
                    } else if class_name {
                        ob.ch_name(var);
                        class_name = false;
                    } else if parent {
                        ob.ch_parent(var);
                        parent = false;
                    }
                }
                Stream::Object(_) => class_name = true,
                Stream::Access(key) => ob.ch_access(key),
                Stream::Modifier(key) => ob.add_modifier(key),
                Stream::Exception => {
                    exception = true;
                    implement = false;
                    parent = false;
                    class_name = false;
                }
                Stream::Implement => {
                    exception = false;
                    implement = true;
                    parent = false;
                    class_name = false;
                }
                Stream::Parent => {
                    exception = false;
                    implement = false;
                    parent = true;
                    class_name = false;
                }
                _ => println!("Class pattern not supported {:?}", gram_parts[i]),
            }
        }

        if java_doc.description != "" {
            ob.ch_description(java_doc.description.clone());
        }

        if java_doc.author != "" {
            ob.ch_author(java_doc.author.clone());
        }

        if java_doc.version != "" {
            ob.ch_version(java_doc.version.clone());
        }
    }

    /// Handles token streams for methods and returns a `Method` struct
    /// Containing the methods information from it's declaration
    ///
    /// # Arguments
    ///
    /// * `gram_parts` - A vector of tokens from the method's declaration
    /// * `_java_doc` - The java doc struct with the documentation for the method
    fn get_method(gram_parts: Vec<Stream>, java_doc: &Doc, line_num: String) -> Method {
        let mut method = Method::new();
        let mut exception = false;
        let mut method_name = false;
        let mut param_name = false;
        let mut param_type = String::new();

        for i in 0..gram_parts.len() {
            match gram_parts[i].clone() {
                Stream::Variable(var) => {
                    if exception {
                        if java_doc.exceptions.len() > 0 {
                            method.add_exception(Exception {
                                desc: java_doc.exceptions[0].clone().desc,
                                exception_type: var,
                            });
                        }
                    } else if method_name {
                        method.ch_method_name(var);
                        method_name = false;
                    } else if param_name {
                        method.add_param(Param {
                            var_type: param_type.clone(),
                            name: var,
                            desc: String::new(),
                        });

                        param_type = String::new();
                        param_name = false;
                    } else if method.name == "" {
                        method.ch_return_type(var);
                        method_name = true;
                    }
                }
                Stream::Type(key) => {
                    if method.return_type == "" {
                        method.ch_return_type(key);
                        method_name = true;
                    } else {
                        param_type = key;
                        param_name = true;
                    }
                }
                Stream::Access(key) => method.ch_privacy(key),
                Stream::Modifier(key) => method.add_modifier(key),
                Stream::Exception => exception = true,
                _ => println!("Method pattern not supported"),
            }
        }
        method.ch_line_num(line_num);

        if java_doc.return_desc != "" {
            method.ch_return_type(java_doc.return_desc.clone());
        }

        if java_doc.description != "" {
            method.ch_description(java_doc.description.clone());
        }

        let n_params: Vec<Param> =
            match_params(&mut method, &java_doc.params, &mut String::new(), false);
        method.ch_params(n_params);

        method
    }

    /// Handles token streams for member variables and returns a `Member` struct
    /// Containing the member variable's data
    ///
    /// # Arguments
    ///
    /// * `gram_parts` - A vector of tokens in the member variable expression
    fn get_var(gram_parts: Vec<Stream>, line_num: String) -> Member {
        let mut member = Member::new();
        let mut member_name = false;

        for i in 0..gram_parts.len() {
            match gram_parts[i].clone() {
                Stream::Variable(var) => {
                    if var == "=" {
                        return member;
                    } else if member_name {
                        member.ch_name(var);
                        return member;
                    } else {
                        member.ch_type(var);
                        member_name = true;
                    }
                }
                Stream::Type(key) => {
                    if key.contains("=") {
                        let parts: Vec<&str> = key.split("=").collect();
                        member.ch_name(parts[0].to_string());

                        return member;
                    } else {
                        member.ch_type(key);
                        member_name = true;
                    }
                }
                Stream::Access(key) => member.ch_access(key),
                Stream::Modifier(key) => member.add_modifier(key),
                _ => println!("Member variable pattern not supported"),
            }
        }
        member.ch_line_number(line_num);

        member
    }

    fn match_params(
        method: &mut Method,
        jparams: &Vec<Param>,
        jdoc_err: &mut String,
        lint: bool,
    ) -> Vec<Param> {
        let params = method.clone_params();
        let mut new_param: Vec<Param> = Vec::new();

        for mut param in params {
            let mut found = false;
            for i in 0..jparams.len() {
                if param.name == jparams[i].name {
                    new_param.push(Param {
                        name: param.name.clone(),
                        var_type: param.var_type.clone(),
                        desc: jparams[i].desc.clone(),
                    });
                    found = true;
                }
            }

            if !found {
                new_param.push(Param {
                    name: param.name.clone(),
                    var_type: param.var_type.clone(),
                    desc: "No description found".to_string(),
                });

                if lint {
                    jdoc_err.push_str(
                        "\tJavadoc parameter not found "
                            .yellow()
                            .to_string()
                            .as_str(),
                    );
                    jdoc_err.push_str(
                        format!(
                            "{} in method: {} (Line: {})\n",
                            param.name,
                            method.name.clone(),
                            method.line_num
                        ).as_str(),
                    );
                }
            }
        }

        new_param
    }

    macro_rules! is_keyword {
        ($w:expr, $k:expr) => {{
            let mut found = false;
            for key in $k {
                if key == $w {
                    found = true
                }
            }

            found
        }};
    }

    fn push_token(curr_token: &String, tokens: &mut Vec<Token>) {
        if curr_token != "" {
            let keywords = get_keywords();
            let jdoc_keywords = get_jdoc_keywords();
            if is_keyword!(curr_token, keywords) {
                tokens.push(Token::Keyword(curr_token.to_string()));
            } else if is_keyword!(curr_token, jdoc_keywords) {
                tokens.push(Token::Keyword(curr_token.to_string()));
            } else {
                tokens.push(Token::Symbol(curr_token.to_string()));
            }
        }
    }

    pub fn lex_contents(content: &String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut curr_token = String::new();
        let mut block_depth = 0;
        let mut line_number = 1;
        let mut blob = content.chars();

        tokens.push(Token::LineNumber(line_number.to_string()));
        loop {
            match blob.next() {
                Some(ch) => match ch {
                    ' ' | '\t' => {
                        if block_depth < 2 {
                            push_token(&curr_token, &mut tokens);
                        }
                        curr_token = String::new();
                    }
                    '\n' => {
                        if block_depth < 2 {
                            push_token(&curr_token, &mut tokens);
                        }

                        line_number = line_number + 1;
                        tokens.push(Token::LineNumber(line_number.to_string()));
                        curr_token = String::new();
                    }
                    ',' => {
                        if block_depth < 2 {
                            push_token(&curr_token, &mut tokens);
                            tokens.push(Token::Join)
                        }
                        curr_token = String::new();
                    }
                    ';' => {
                        if block_depth < 2 {
                            push_token(&curr_token, &mut tokens);
                            tokens.push(Token::ExpressionEnd(";".to_string()));
                        }
                        curr_token = String::new();
                    }
                    '(' => {
                        if block_depth < 2 {
                            push_token(&curr_token, &mut tokens);
                            tokens.push(Token::ParamStart);
                        }
                        curr_token = String::new();
                    }
                    ')' => {
                        if block_depth < 2 {
                            push_token(&curr_token, &mut tokens);
                            tokens.push(Token::ParamEnd);
                        }
                        curr_token = String::new();
                    }
                    '{' => {
                        if block_depth < 2 {
                            push_token(&curr_token, &mut tokens);
                            tokens.push(Token::ExpressionEnd("{".to_string()));
                        }
                        curr_token = String::new();
                        block_depth = block_depth + 1;
                    }
                    '}' => {
                        if block_depth < 2 {
                            push_token(&curr_token, &mut tokens);
                        }
                        curr_token = String::new();
                        block_depth = block_depth - 1;
                    }
                    _ => {
                        if block_depth < 2 {
                            curr_token.push_str(ch.to_string().as_str());
                        }
                    }
                },
                None => break,
            }
        }

        tokens
    }

    macro_rules! access_mod_match {
        ($e:expr) => {
            match $e {
                Token::Keyword(value) => match value.as_ref() {
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
                Token::Keyword(value) => match value.as_ref() {
                    "static" | "final" | "abstract" | "synchronized" | "volatile" => true,
                    _ => false,
                },
                _ => false,
            }
        };
    }

    /// Constucts a syntax tree based on the stream of token from the lexing
    /// Outputs a Class struct containing all the data for a java class
    ///
    /// # Arguments
    ///
    /// * `tokens` - The list of tokens from the lexer
    pub fn construct_ast(tokens: Vec<Token>) -> ObjectType {
        let mut annotation = false;
        let mut ignore = false;
        let mut object = Object::new();
        let mut in_object = false;
        let mut parse_state = ParseState::new();
        let mut doc = false;
        let mut comment = false;
        let mut jdoc = Doc::new();
        let mut _jdoc_errs = String::new();
        let mut symbols: Vec<String> = Vec::new();
        let mut doc_tokens: Vec<JdocToken> = Vec::new();
        let mut method: Method = Method::new();
        let mut gram_parts: Vec<Stream> = Vec::new();
        let mut comment_buf = String::new();
        let mut line_num = String::new();

        for token in tokens.clone() {
            if ignore {
                match token.clone() {
                    Token::ParamEnd => ignore = false,
                    _ => continue,
                }

                continue;
            }

            match token.clone() {
                Token::Keyword(key) => {
                    let temp_sym = symbols.clone();
                    if temp_sym.len() == 1 {
                        gram_parts.push(Stream::Variable(temp_sym[0].clone()));
                    } else if temp_sym.len() > 1 {
                        gram_parts.push(Stream::Type(temp_sym[..temp_sym.len() - 1].join(" ")));
                        gram_parts.push(Stream::Variable(temp_sym[temp_sym.len() - 1].clone()));
                    }

                    match key.as_ref() {
                        "class" => {
                            if !doc && !comment {
                                object.ch_state(ObjectState::Class);
                                gram_parts.push(Stream::Object(key.to_string()));
                                parse_state.ch_class(true);
                            }
                            in_object = true;
                        }
                        "interface" => {
                            if !doc && !comment {
                                object.ch_state(ObjectState::Interface);
                                gram_parts.push(Stream::Object(key.to_string()));
                                parse_state.ch_interface(true);
                            }
                            in_object = true;
                        }
                        "enum" => {
                            if !doc && !comment {
                                object.ch_state(ObjectState::Enumeration);
                                gram_parts.push(Stream::Object(key.to_string()));
                                parse_state.ch_enum(true);
                            }
                            in_object = true;
                        }
                        "package" => {
                            if comment_buf != "" {
                                object.ch_license(comment_buf.clone());
                            }
                            gram_parts.push(Stream::Package);
                        }
                        "throws" => gram_parts.push(Stream::Exception),
                        "extends" => gram_parts.push(Stream::Parent),
                        "implements" => gram_parts.push(Stream::Implement),
                        "import" => gram_parts.push(Stream::Import),
                        _ => {
                            if access_mod_match!(token.clone()) {
                                gram_parts.push(Stream::Access(key.to_string()));
                            } else if modifier_match!(token.clone()) {
                                gram_parts.push(Stream::Modifier(key.to_string()));
                            } else if is_keyword!(key, get_jdoc_keywords()) {
                                doc_tokens.push(JdocToken::Keyword(key.clone()));
                            } else if doc {
                                doc_tokens.push(JdocToken::Symbol(key.clone()));
                            } else if !comment && !doc {
                                println!("Keyword not supported: {}", key);
                            }
                        }
                    }

                    if comment {
                        comment_buf.push_str(format!("{} ", key).as_str());
                    }

                    annotation = false;
                }
                Token::Symbol(word) => {
                    match word.as_ref() {
                        "/**" => doc = true,
                        "*/" => {
                            if doc {
                                jdoc = get_doc(&doc_tokens);
                                parse_state = ParseState::new();
                                doc_tokens.clear();
                                gram_parts.clear();
                            }

                            doc = false;
                            comment = false;
                        }
                        "//" => comment = true,
                        "/*" => {
                            comment_buf = String::new();
                            comment = true;
                        }
                        _ => {
                            if doc {
                                if is_keyword!(word, get_jdoc_keywords()) {
                                    doc_tokens.push(JdocToken::Keyword(word.clone()));
                                } else {
                                    doc_tokens.push(JdocToken::Symbol(word.clone()));
                                }
                            } else if word.contains("@") && !doc {
                                annotation = true;
                                continue;
                            } else if !comment {
                                symbols.push(word.to_string());
                            }
                        }
                    }

                    if comment {
                        if word != "*" && word != "/*" {
                            comment_buf.push_str(format!("{} ", word).as_str());
                        }
                    }

                    annotation = false;
                }
                Token::Join => {
                    if symbols.len() > 1 {
                        let temp_sym = symbols.clone();
                        gram_parts.push(Stream::Type(temp_sym[..temp_sym.len() - 1].join(" ")));
                        gram_parts.push(Stream::Variable(temp_sym[temp_sym.len() - 1].clone()));
                    }

                    if comment {
                        comment_buf.push_str(",");
                    }

                    symbols.clear();
                }
                Token::ParamStart => {
                    if annotation {
                        ignore = true;
                        annotation = false;
                    } else {
                        let temp_sym = symbols.clone();
                        if temp_sym.len() == 1 {
                            gram_parts.push(Stream::Variable(temp_sym[0].clone()));
                        } else if temp_sym.len() > 1 {
                            gram_parts.push(Stream::Type(temp_sym[..temp_sym.len() - 1].join(" ")));
                            gram_parts.push(Stream::Variable(temp_sym[temp_sym.len() - 1].clone()));
                        }
                    }

                    if comment {
                        comment_buf.push_str("(");
                    }

                    symbols.clear();
                }
                Token::ParamEnd => {
                    let temp_sym = symbols.clone();
                    if symbols.len() == 1 {
                        method.ch_method_name(temp_sym[0].clone());
                    } else if symbols.len() > 1 {
                        gram_parts.push(Stream::Type(temp_sym[..temp_sym.len() - 1].join(" ")));
                        gram_parts.push(Stream::Variable(temp_sym[temp_sym.len() - 1].clone()));
                    }

                    if comment {
                        comment_buf.push_str(")");
                    }
                    symbols.clear();
                }
                Token::ExpressionEnd(end) => {
                    // For any symbols not included add them to the stream for parsing
                    if symbols.len() == 1 {
                        gram_parts.push(Stream::Variable(symbols[0].clone()));
                    } else if symbols.len() > 1 {
                        gram_parts.push(Stream::Type(symbols[..symbols.len() - 1].join(" ")));
                        gram_parts.push(Stream::Variable(symbols[symbols.len() - 1].clone()));
                    }

                    let mut temp_gram = gram_parts.clone();

                    match end.as_ref() {
                        ";" => {
                            if !in_object {
                                if temp_gram.len() > 1 {
                                    match temp_gram[0].clone() {
                                        Stream::Import => match temp_gram[1].clone() {
                                            Stream::Variable(key) => object.add_dependency(key),
                                            _ => println!("Pattern not supported"),
                                        },
                                        Stream::Package => match temp_gram[1].clone() {
                                            Stream::Variable(key) => object.ch_package_name(key),
                                            _ => println!("Pattern not supported"),
                                        },
                                        _ => {
                                            object.add_variable(get_var(temp_gram, line_num.clone()))
                                        }
                                    }
                                }
                            } else {
                                match object.state {
                                    ObjectState::Class => object.add_variable(get_var(temp_gram, line_num.clone())),
                                    _ => object.add_method(get_method(
                                        temp_gram,
                                        &jdoc,
                                        line_num.clone(),
                                    )),
                                }
                            }
                        }
                        "{" => {
                            if parse_state.interface || parse_state.class {
                                get_object(temp_gram.clone(), &jdoc, &mut object);
                            } else {
                                object.add_method(get_method(temp_gram, &jdoc, line_num.clone()));
                            }
                        }
                        _ => {
                            if comment {
                                comment = false;
                            } else if !doc {
                                panic!("Expression end not allowed");
                            }
                        }
                    }

                    parse_state = ParseState::new();
                    jdoc = Doc::new();
                    gram_parts.clear();
                    symbols.clear();
                }
                Token::LineNumber(num) => line_num = num,
            }
        }

        match object.state {
            ObjectState::Class => return ObjectType::Class(object.to_class()),
            ObjectState::Interface => return ObjectType::Interface(object.to_interface()),
            ObjectState::Enumeration => return ObjectType::Enumeration(object.to_enumeration()),
            ObjectState::Unset => {
                println!("Java file type not supported. Supported types: class, interface, enum");
                return ObjectType::Class(object.to_class());
            },
        }
    }

    /// Root function of the module. Calls the lex and parse functions and returns
    /// a `Class` struct.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the java file
    /// * `lint` - A bool representing whether the class's javadoc comments should be linted
    pub fn parse_file(path: &Path, _lint: bool) -> ObjectType {
        let file = File::open(path).expect("Could not open file");
        let mut contents = String::new();
        let mut buf = BufReader::new(file);
        let res = buf.read_to_string(&mut contents);
        if res.is_ok() {
            let tokens = lex_contents(&contents);
            construct_ast(tokens)
        } else {
            println!("Unable to read file");
            ObjectType::Class(Class::new())
        }
    }
}

#[cfg(test)]
mod test;
