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

    use colored::*;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;
    use std::path::Path;

    fn get_doc(tokens: &Vec<JdocToken>) -> Doc {
        let mut return_str = String::from("");
        let mut desc = String::from("");
        let mut parameters: Vec<Param> = Vec::new();
        let mut author = String::new();
        let mut version = String::new();
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
                            JdocState::Jdoc_return => {
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
                            _ => { /* println!("Code javadoc field not supported") */ }
                        }

                        word_buf.clear();
                    }

                    match key.as_ref() {
                        "@return" => state = JdocState::Jdoc_return,
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
        }
    }

    fn get_object(gram_parts: Vec<Stream>, _java_doc: &Doc, class: &mut Class) {
        let mut implement = false;
        let mut exception = false;
        let mut parent = false;
        let mut class_name = false;

        for i in 0..gram_parts.len() {
            match gram_parts[i].clone() {
                Stream::Variable(var) => {
                    if implement {
                        class.add_interface(var);
                    } else if exception {
                        class.add_exception(Exception {
                            desc: String::new(),
                            exception_type: var,
                        });
                    } else if class_name {
                        class.ch_class_name(var);
                        class_name = false;
                    } else if parent {
                        class.ch_parent(var);
                        parent = false;
                    }
                }
                Stream::Object(var) => {
                    if var == "interface" {
                        class.ch_is_class(false);
                    }
                    class_name = true;
                }
                Stream::Access(key) => class.ch_access(key),
                Stream::Modifier(key) => class.add_modifier(key),
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
    }

    fn get_method(gram_parts: Vec<Stream>, _java_doc: &Doc) -> Method {
        let mut method = Method::new();
        let mut exception = false;
        let mut method_name = false;

        for i in 0..gram_parts.len() {
            match gram_parts[i].clone() {
                Stream::Variable(var) => {
                    if exception {
                        if _java_doc.exceptions.len() > 0 {
                            method.add_exception(Exception {
                                desc: _java_doc.exceptions[0].clone().desc,
                                exception_type: var,
                            });
                        }
                    } else if method_name {
                        method.ch_method_name(var);
                        method_name = false;
                    } else if method.name == "" {
                        method.ch_return_type(var);
                        method_name = true;
                    }
                }
                Stream::Access(key) => method.ch_privacy(key),
                Stream::Modifier(key) => method.add_modifier(key),
                Stream::Exception => exception = true,
                _ => {
                    /*
                    println!("Method pattern not supported");
                    println!("{:?}", gram_parts);
                    */
                }
            }
        }

        method
    }

    fn get_var(gram_parts: Vec<Stream>) -> Member {
        let mut member = Member::new();
        let mut member_name = false;

        for i in 0..gram_parts.len() {
            match gram_parts[i].clone() {
                Stream::Variable(var) => {
                    if var == "=" {
                        return member;
                    } else if member_name {
                        member.ch_name(var);
                        member_name = false;
                    } else if member.name == "" {
                        member.ch_type(var);
                        member_name = true;
                    }
                }
                Stream::Access(key) => member.ch_access(key),
                Stream::Modifier(key) => member.add_modifier(key),
                _ => {
                    /*
                    println!("Member variable pattern not supported");
                    println!("{:?}", gram_parts[i]);
                    println!("{:?}", gram_parts);
                    */
                }
            }
        }

        member
    }

    /*

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
    */

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
            } else if !curr_token.contains("@") {
                tokens.push(Token::Symbol(curr_token.to_string()));
            }
        }
    }

    pub fn lex_contents(content: &String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut curr_token = String::new();
        let mut block_depth = 0;
        let mut blob = content.chars();

        loop {
            let ch_res = blob.next();

            match ch_res {
                Some(ch) => match ch {
                    ' ' | '\t' | '\n' => {
                        if block_depth < 2 {
                            push_token(&curr_token, &mut tokens);
                        }
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
                            tokens.push(Token::Expression_end(";".to_string()));
                        }
                        curr_token = String::new();
                    }
                    '(' => {
                        if block_depth < 2 {
                            push_token(&curr_token, &mut tokens);
                            tokens.push(Token::Param_start);
                        }
                        curr_token = String::new();
                    }
                    ')' => {
                        if block_depth < 2 {
                            push_token(&curr_token, &mut tokens);
                            tokens.push(Token::Param_end);
                        }
                        curr_token = String::new();
                    }
                    '{' => {
                        if block_depth < 2 {
                            push_token(&curr_token, &mut tokens);
                            tokens.push(Token::Expression_end("{".to_string()));
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

    pub fn construct_ast(tokens: Vec<Token>) -> Class {
        let mut class = Class::new();
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

        for token in tokens.clone() {
            match token.clone() {
                Token::Keyword(key) => match key.as_ref() {
                    "class" => {
                        if !doc && !comment {
                            gram_parts.push(Stream::Object("class".to_string()));
                            parse_state.ch_class(true);
                        }
                        in_object = true;
                    }
                    "interface" => {
                        if !doc && !comment {
                            gram_parts.push(Stream::Object("interface".to_string()));
                            parse_state.ch_interface(true);
                        }
                        in_object = true;
                    }
                    "package" => gram_parts.push(Stream::Package),
                    "throws" => gram_parts.push(Stream::Exception),
                    "extends" => gram_parts.push(Stream::Parent),
                    "implements" => gram_parts.push(Stream::Implement),
                    "import" => gram_parts.push(Stream::Import),
                    "enum" => gram_parts.push(Stream::Object("enum".to_string())),
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
                },
                Token::Symbol(word) => match word.as_ref() {
                    "/**" => doc = true,
                    "*/" => {
                        if doc {
                            jdoc = get_doc(&doc_tokens);
                            println!("{:?}", jdoc);
                            parse_state = ParseState::new();
                            doc_tokens.clear();
                            gram_parts.clear();
                        }
                        doc = false;
                        comment = false;
                    }
                    "//" => comment = true,
                    "/*" => comment = true,
                    _ => {
                        if doc {
                            if is_keyword!(word, get_jdoc_keywords()) {
                                doc_tokens.push(JdocToken::Keyword(word.clone()));
                            } else {
                                doc_tokens.push(JdocToken::Symbol(word.clone()));
                            }
                        } else if !comment {
                            symbols.push(word.to_string());
                            gram_parts.push(Stream::Variable(word));
                        }
                    }
                },
                Token::Join => {
                    if symbols.len() > 1 {
                        let temp_sym = symbols.clone();
                        gram_parts.push(Stream::Type(temp_sym[0].clone()));
                        gram_parts.push(Stream::Variable(temp_sym[1].clone()));
                    }

                    symbols.clear();
                }
                Token::Param_start => {
                    let temp_sym = symbols.clone();
                    if temp_sym.len() == 1 {
                        gram_parts.push(Stream::Type(temp_sym[0].clone()));
                    } else if temp_sym.len() > 1 {
                        gram_parts.push(Stream::Type(temp_sym[0].clone()));
                        gram_parts.push(Stream::Variable(temp_sym[1].clone()));
                    }

                    symbols.clear();
                }
                Token::Param_end => {
                    let temp_sym = symbols.clone();
                    if symbols.len() == 1 {
                        method.ch_method_name(temp_sym[0].clone());
                    } else if symbols.len() > 1 {
                        method.ch_return_type(temp_sym[0].clone());
                        method.ch_method_name(temp_sym[1].clone());
                    }
                    symbols.clear();
                }
                Token::Expression_end(end) => {
                    let mut temp_gram = gram_parts.clone();
                    match end.as_ref() {
                        ";" => {
                            if !in_object {
                                if temp_gram.len() > 1 {
                                    match temp_gram[0].clone() {
                                        Stream::Import => match temp_gram[1].clone() {
                                            Stream::Variable(key) => class.add_dependency(key),
                                            _ => println!("Pattern not supported"),
                                        },
                                        Stream::Package => match temp_gram[1].clone() {
                                            Stream::Variable(key) => class.ch_package_name(key),
                                            _ => println!("Pattern not supported"),
                                        },
                                        _ => class.add_variable(get_var(temp_gram)),
                                    }
                                }
                            } else {
                                if class.is_class {
                                    class.add_variable(get_var(temp_gram));
                                }
                            }
                        }
                        "{" => {
                            if parse_state.interface || parse_state.class {
                                get_object(temp_gram.clone(), &jdoc, &mut class);
                            } else {
                                class.add_method(get_method(temp_gram, &jdoc));
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
                    gram_parts.clear();
                }
            }
        }

        class
    }

    pub fn parse_file(path: &Path, _lint: bool) -> Class {
        let file = File::open(path).expect("Could not open file");
        let mut contents = String::new();
        let mut buf = BufReader::new(file);
        let res = buf.read_to_string(&mut contents);
        if res.is_ok() {
            let tokens = lex_contents(&contents);
            construct_ast(tokens)
        } else {
            println!("Unable to read file");
            Class::new()
        }
    }
}
