pub mod parse {
    //! A module which handles the parsing for java files
    extern crate colored;

    use grammar::grammar::*;
    use model::model::Class;
    use model::model::Doc;
    use model::model::Exception;
    use model::model::Method;
    use model::model::Param;
    use model::model::Member;

    use colored::*;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;
    use std::path::Path;

    fn doc_desc(parts: &Vec<String>) -> String {
        let mut description = String::new();

        for i in 0..parts.len() {
            description.push_str(format!("{} ", parts[i].as_str()).as_str());
        }

        description
    }

    fn get_doc(tokens: &Vec<Jdoc_token>) -> Doc {
        let mut return_str = String::from("");
        let mut desc = String::from("");
        let mut parameters: Vec<Param> = Vec::new();
        let mut author = String::new();
        let mut version = String::new();
        let mut deprecated = String::new();
        let mut exceptions: Vec<Exception> = Vec::new();
        let mut state = Jdoc_state::desc;

        let mut word_buf = String::new();

        for i in 0..tokens.len() {
            match tokens[i].clone() {
                Jdoc_token::keyword(key) => {
                    let new_desc = word_buf.clone();
                    if i != 0 {
                        match state {
                            Jdoc_state::jdoc_return => return_str = new_desc,
                            Jdoc_state::param => {
                                let word_parts: Vec<&str> = new_desc.split(" ").collect();

                                parameters.push(Param {
                                    var_type: String::new(),
                                    name: word_parts[0].to_string(),
                                    desc: word_parts[1..].join(""),
                                });
                            }
                            Jdoc_state::author => author = new_desc,
                            Jdoc_state::deprecated => deprecated = new_desc,
                            Jdoc_state::exception => {
                                let word_parts: Vec<&str> = new_desc.split(" ").collect();

                                exceptions.push(Exception {
                                    exception_type: word_parts[0].to_string(),
                                    desc: word_parts[1..].join(""),
                                });
                            },
                            Jdoc_state::version => version = new_desc,
                            Jdoc_state::desc => desc = new_desc,
                            _ => println!("Code javadoc field not supported"),
                        }

                        word_buf.clear();
                    }

                    match key.as_ref() {
                        "@return" => state = Jdoc_state::jdoc_return,
                        "@param" => state = Jdoc_state::param,
                        "@author" => state = Jdoc_state::author,
                        "@code" => state = Jdoc_state::code,
                        "@deprecated" => state = Jdoc_state::deprecated,
                        "@docRoot" => state = Jdoc_state::docRoot,
                        "@exception" => state = Jdoc_state::exception,
                        "@inheritDoc" => state = Jdoc_state::inheritDoc,
                        "@link" => state = Jdoc_state::link,
                        "@linkplain" => state = Jdoc_state::linkplain,
                        "@literal" => state = Jdoc_state::literal,
                        "@see" => state = Jdoc_state::see,
                        "@throws" => state = Jdoc_state::exception,
                        "@since" => state = Jdoc_state::since,
                        "@serialData" => state = Jdoc_state::serialData,
                        "@serialField" => state = Jdoc_state::serialField,
                        "@value" => state = Jdoc_state::value,
                        "@version" => state = Jdoc_state::version,
                        _ => println!("Unsupported javadoc keyword used"),
                    }
                },
                Jdoc_token::symbol(key) => word_buf.push_str(key.as_str()),
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

    fn get_object(gram_parts: Vec<Stream>, java_doc: &Doc) -> Class {
        let mut class = Class::new();
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
                           exception_type: var
                        });
                    } else if class_name {
                        class.ch_class_name(var);
                        class_name = false;
                    } else if parent {
                        class.ch_parent(var);
                        parent = false;
                    }
                },
                Stream::Object(var) => {
                    if var == "interface" {
                        class.ch_is_class(false);
                    }
                    class_name = true;
                },
                Stream::Access(key) => class.ch_access(key),
                Stream::Modifier(key) => class.add_modifier(key),
                Stream::Exception => {
                    exception = true;
                    implement = false;
                    parent = false;
                },
                Stream::Implement => {
                    exception = false;
                    implement = true;
                    parent = false;
                },
                Stream::Parent => {
                    exception = false;
                    implement = false;
                    parent = true;
                },
                _ => println!("Class pattern not supported {:?}", gram_parts[i]),
            }
        }

        class
    }

    fn get_method(gram_parts: Vec<Stream>, java_doc: &Doc) -> Method {
        let mut method = Method::new();
        let mut exception = false;
        let mut method_name = false;

        for i in 0..gram_parts.len() {
            match gram_parts[i].clone() {
                Stream::Variable(var) => {
                    if exception {
                        method.add_exception(Exception {
                           desc: java_doc.exceptions[0].clone().desc,
                           exception_type: var
                        });
                    } else if method_name {
                        method.ch_method_name(var);
                        method_name = false;
                    } else if method.name == "" {
                        method.ch_return_type(var);
                        method_name = true;
                    }
                },
                Stream::Access(key) => method.ch_privacy(key),
                Stream::Modifier(key) => method.add_modifier(key),
                Stream::Exception => exception = true,
                _ => println!("Method pattern not supported"),
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
                },
                Stream::Access(key) => member.ch_access(key),
                Stream::Modifier(key) => member.add_modifier(key),
                _ => println!("Member variable pattern not supported"),
            }
        }

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

    fn push_token(depth: i32, curr_token: &String, tokens: &mut Vec<Token>) {
        if depth <= 1 && curr_token.len() > 0 {
            let keywords = get_keywords();
            if is_keyword!(curr_token, keywords) {
                tokens.push(Token::keyword(curr_token.to_string()));
            } else {
                tokens.push(Token::symbol(curr_token.to_string()));
            }
        }
    }

    pub fn lex_contents(content: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut curr_token = String::new();
        let mut block_depth = 0;
        let mut blob = content.chars();

        loop {
            let ch_res = blob.next();

            match ch_res {
                Some(ch) => match ch {
                    ' ' | '\t' | '\n' => {
                        push_token(block_depth, &curr_token, &mut tokens);
                        curr_token = String::new();
                    }
                    ',' => tokens.push(Token::join),
                    ';' => {
                        if block_depth <= 1 {
                            tokens.push(Token::expression_end(";".to_string()));
                        }
                        curr_token = String::new();
                    }
                    '(' => {
                        push_token(block_depth, &curr_token, &mut tokens);
                        if block_depth <= 1 {
                            tokens.push(Token::param_start);
                        }
                        curr_token = String::new();
                    }
                    ')' => {
                        push_token(block_depth, &curr_token, &mut tokens);
                        if block_depth <= 1 {
                            tokens.push(Token::param_end);
                        }
                        curr_token = String::new();
                    }
                    '{' => {
                        if block_depth <= 1 {
                            tokens.push(Token::expression_end("{".to_string()));
                        }
                        curr_token = String::new();

                        block_depth += 1;
                    }
                    '}' => block_depth -= 1,
                    _ => {
                        if block_depth <= 1 {
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

    pub fn construct_ast(tokens: Vec<Token>) -> Class {
        let mut class = Class::new();
        let mut parse_state = ParseState::new();
        let mut doc = false;
        let mut comment = false;
        let mut jdoc = Doc::new();
        let mut jdoc_errs = String::new();
        let mut symbols: Vec<String> = Vec::new();
        let mut jdoc_keywords: Vec<String> = Vec::new();
        let mut doc_tokens: Vec<Jdoc_token> = Vec::new();
        let mut method: Method = Method::new();
        let mut gram_parts: Vec<Stream> = Vec::new();

        // Only allow parameters one layer deep in param definition
        let mut param_depth = 0;

        for token in tokens.clone() {
            match token.clone() {
                Token::keyword(key) => {
                    match key.as_ref() {
                        "class" => {
                            if !doc && !comment {
                                gram_parts.push(Stream::Object("class".to_string()));
                                parse_state.ch_class(true);
                            }
                        },
                        "interface" => {
                            if !doc && !comment {
                                gram_parts.push(Stream::Object("interface".to_string()));
                                parse_state.ch_interface(true);
                            }
                        },
                        "package" => gram_parts.push(Stream::Package),
                        "throws" => gram_parts.push(Stream::Exception),
                        "extends" => gram_parts.push(Stream::Parent),
                        "implements" => gram_parts.push(Stream::Implement),
                        "import" => gram_parts.push(Stream::Import),
                        "enum" => gram_parts.push(Stream::Object("enum".to_string())),
                        _ => {
                            if access_mod_match!(token.clone()) {
                                class.ch_access(key.to_string());
                            } else if modifier_match!(token.clone()) {
                                class.add_modifier(key.to_string());
                            }
                        },
                    }
                }
                Token::symbol(word) => {
                    match word.as_ref() {
                        "/**" => doc = true,
                        "*/" => {
                            if doc {
                                jdoc = get_doc(&doc_tokens);
                                doc = false;
                            }
                            comment = false;
                        },
                        "//" => comment = true,
                        "/*" => comment = true,
                        _ => {
                            if doc {
                                if is_keyword!(word, get_jdoc_keywords()) {
                                    doc_tokens.push(Jdoc_token::keyword(word.clone()));
                                } else {
                                    doc_tokens.push(Jdoc_token::symbol(word.clone()));
                                }
                            } else {
                                symbols.push(word.to_string())
                            }
                        }
                    }
                },
                Token::join => {
                    if symbols.len() > 1 && param_depth == 1 {
                        let temp_sym = symbols.clone();
                        gram_parts.push(Stream::Type(temp_sym[0].clone()));
                        gram_parts.push(Stream::Variable(temp_sym[1].clone()));
                    }

                    symbols.clear();
                }
                Token::param_start => {
                    if param_depth == 0 {
                        let temp_sym = symbols.clone();
                        if symbols.len() == 1 {
                            gram_parts.push(Stream::Type(temp_sym[0].clone()));
                        } else if symbols.len() > 1 {
                            gram_parts.push(Stream::Type(temp_sym[0].clone()));
                            gram_parts.push(Stream::Variable(temp_sym[1].clone()));
                        }
                    }

                    param_depth += 1;
                    symbols.clear();
                }
                Token::param_end => {
                    if param_depth == 0 {
                        let temp_sym = symbols.clone();
                        if symbols.len() == 1 {
                            method.ch_method_name(temp_sym[0].clone());
                        } else if symbols.len() > 1 {
                            method.ch_return_type(temp_sym[0].clone());
                            method.ch_method_name(temp_sym[1].clone());
                        }
                    }

                    param_depth -= 1;
                    symbols.clear();
                }
                Token::doc_keyword(word) => jdoc_keywords.push(word.to_string()),
                Token::expression_end(end) => {
                    println!("Expression end {}", end.as_str());
                    if parse_state.class {
                                let mut temp_gram = gram_parts.clone();
                        match end.as_ref() {
                            ";" => {
                                if !class.is_class {
                                } else if temp_gram.len() == 2 {
                                    match temp_gram[0].clone() {
                                        Stream::Import => {
                                            match temp_gram[1].clone() {
                                                Stream::Variable(key) => class.add_dependency(key),
                                                _ => println!("Pattern not supported"),
                                            }
                                        },
                                        Stream::Package => {
                                            match temp_gram[1].clone() {
                                                Stream::Variable(key) => class.ch_package_name(key),
                                                _ => println!("Pattern not supported"),
                                            }
                                        },
                                        _ => println!("Not import or package"),
                                    }
                                } else {
                                    class.add_variable(get_var(temp_gram));
                                }
                            },
                            "{" => {
                                if parse_state.interface || parse_state.class {
                                    class = get_object(temp_gram.clone(), &jdoc);
                                } else {
                                    class.add_method(get_method(temp_gram, &jdoc));
                                }

                            }
                            _ => println!("Other"),

                        }

                        parse_state = ParseState::new();
                        println!("{:?}", gram_parts);
                        gram_parts.clear();
                    }
                }
            }
        }

        class
    }

    pub fn parse_file(path: &Path, lint: bool) -> Class {
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

#[cfg(test)]
mod test;
