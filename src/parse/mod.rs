pub mod parse {
    //! A module which handles the parsing for java files
    extern crate colored;
    extern crate regex;

    use model::model::*;

    use colored::*;
    use regex::Regex;
    use std::fs::File;
    use std::io::Read;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::path::Path;

    fn regex_match(text: &str, regex_str: &str) -> bool {
        let reg = Regex::new(regex_str).unwrap();

        reg.is_match(text)
    }

    fn start_doc_match(text: &String) -> bool {
        if text.contains(r"/**") {
            return true;
        } else {
            return false;
        }
    }

    fn start_comment_match(text: &String) -> bool {
        if text.contains(r"/*") && !text.contains("/**") {
            return true;
        } else {
            return false;
        }
    }

    fn end_doc_match(text: &String) -> bool {
        if text.contains(r"**/") {
            return true;
        } else if text.contains(r"*/") {
            return true;
        } else {
            return false;
        }
    }

    pub fn trim_paren(part: String) -> String {
        let no_paren: Vec<&str> = part.split(&[')', '{'][..]).collect();
        no_paren.join("")
    }

    /// Determines the line type of a line of java code
    ///
    /// # Arguments
    ///
    /// * `line` - the line to determine the type from
    /// * `state` - the state of parsing the java file
    pub fn determine_line_type(line: &String, state: &ParseState) -> LineType {
        let method_match = r"(public|protected|private|static|\s) +[\w\[\]]+\s+(\w+) *\([^\)]*\)";
        let split = line.split(" ");
        let parts: Vec<&str> = split.collect();

        if line.contains("package ") && !state.class {
            LineType::IsPackage
        } else if line.contains("class ") && !state.comment && !state.doc {
            LineType::IsClass
        } else if line.contains("interface ") && !state.comment && !state.doc {
            LineType::IsInterface
        } else if line.contains("import ") && !state.class {
            LineType::IsImport
        } else if line.contains(";")
            && parts.len() > 1
            && !regex_match(&line, method_match)
            && !line.contains("//")
            && !line.contains("* ")
            && !line.contains("/*")
            && state.class
            && !state.method
        {
            // Case were the member variable assignment calls a contructor
            if line.contains("(") {
                if line.contains("=") {
                    LineType::IsVariable
                } else {
                    LineType::IsOther
                }
            } else {
                LineType::IsVariable
            }
        } else if regex_match(&line, method_match) && state.class && !state.method {
            if line.contains(" if(") {
                LineType::IsOther
            } else if line.contains(" for(") {
                LineType::IsOther
            } else if line.contains(" while(") {
                LineType::IsOther
            } else if line.contains(" catch(") {
                LineType::IsOther
            } else {
                LineType::IsMethod
            }
        } else if start_doc_match(&line) {
            LineType::IsStartdoc
        } else if start_comment_match(&line) {
            LineType::IsStartdoc
        } else if end_doc_match(&line) {
            LineType::IsEnddoc
        } else if line.contains("//") {
            LineType::IsComment
        } else {
            LineType::IsOther
        }
    }

    pub fn handle_class(mut class: Class, line: &String) -> Class {
        let access_match = r"(public|protected|private)";
        let split = line.split(" ");
        let parts: Vec<&str> = split.collect();

        for (num, class_part) in parts.iter().enumerate() {
            if regex_match(&class_part, access_match) {
                class.ch_access(class_part.clone().to_string());
            } else if class_part.contains("class") {
                if parts.len() > num + 1 {
                    class.ch_class_name(parts[num + 1].to_string());
                }
            } else if class_part.contains("extends") {
                if parts.len() > num + 1 {
                    class.ch_parent(parts[num + 1].to_string());
                }
            } else if class_part.contains("implements") {
                if parts.len() > num + 1 {
                    class.add_interface(parts[num + 1].to_string());
                }
            } else if class_part.contains("exception") {
                if parts.len() > num + 1 {
                    class.ch_class_name(parts[num + 1].to_string());
                }
            }
        }

        return class;
    }

    pub fn handle_interface(mut inter: Class, line: &String) -> Class {
        let access_match = r"(public|protected|private)";
        let split = line.split(" ");
        let parts: Vec<&str> = split.collect();

        for (num, inter_part) in parts.iter().enumerate() {
            if regex_match(&inter_part, access_match) {
                inter.ch_access(inter_part.clone().to_string());
            } else if inter_part.contains("interface") {
                if parts.len() > num + 1 {
                    inter.ch_class_name(parts[num + 1].to_string());
                }
            }
        }

        return inter;
    }

    pub fn handle_method(line: &String, num: usize) -> Result<Method, &str> {
        let parts = trim_whitespace(line);
        let mut method = Method::new();
        method.ch_line_num(num.to_string());

        for (i, method_part) in parts.iter().enumerate() {
            if method_part == "public"
                || method_part == "private"
                || method_part == "protected"
            {
                method.ch_privacy(method_part.clone().to_string());
            } else if method_part == "void" {
                method.ch_return_type("void".to_string());
            } else if method_part == "static" {
                method.ch_is_static(true);
            } else if method_part == "exception" {
                if parts.len() > i + 1 {
                    let ex = Exception {
                        exception_type: String::new(),
                        desc: parts[i + 1].to_string(),
                    };
                    method.ch_exception(ex);
                }
            } else if method_part.contains("(") {
                let name_parts: Vec<&str> = method_part.split("(").collect();
                let mut param_def = false;
                let mut param_type = String::new();

                if name_parts[1] != "" {
                    param_type = name_parts[1].to_string();
                    param_def = true;
                }
                for j in (i + 1)..parts.len() {
                    let mut meth_part = parts[j].clone();
                    if parts[j].contains(")") {
                        meth_part = trim_paren(meth_part);
                    }

                    if param_def {
                        if parts[j].contains(">") || parts[j].contains("]") {
                            param_type.push_str(format!("{}", meth_part).as_str());
                        } else {
                            method.add_param(Param {
                                desc: String::new(),
                                var_type: param_type.clone(),
                                name: meth_part,
                            });

                            param_def = false;
                        }
                    } else {
                        param_type = meth_part;
                        param_def = true;
                    }
                }

                if name_parts[0] == "" {
                    method.ch_method_name(parts[i - 1].clone());
                } else {
                    method.ch_method_name(name_parts[0].to_string());
                }
            }
        }

        Ok(method)
    }

    fn doc_desc(parts: &Vec<String>) -> String {
        let mut description = String::new();

        for i in 0..parts.len() {
            description.push_str(format!("{} ", parts[i].as_str()).as_str());
        }

        description
    }

    /// Removes all whitespcace from a line
    ///
    /// # Arguments
    ///
    /// * `line` - The string to remove all the whitespace from
    pub fn trim_whitespace(line: &String) -> Vec<String> {
        let sub_strs: Vec<&str> = line.split(" ").collect();
        let mut trimmed: Vec<String> = Vec::new();

        for sub in sub_strs {
            let bytes: Vec<u8> = sub.to_string().into_bytes();
            let mut new_str: Vec<u8> = Vec::new();

            for b in bytes {
                if b > 32 && b != 44 {
                    new_str.push(b);
                }
            }

            if new_str.len() == 1 {
                // If the line has an individual asterisk it ignores it
                if new_str[0] == 42 {
                    continue;
                }
            } else if new_str.len() > 0 {
                trimmed.push(unsafe { String::from_utf8_unchecked(new_str) });
            }
        }

        trimmed
    }

    /// Handles parsing javadoc comments
    pub fn handle_doc(buffer: &Vec<String>) -> Doc {
        let mut return_str = String::from("");
        let mut desc = String::from("");
        let mut parameters: Vec<Param> = Vec::new();
        let mut author = String::new();
        let mut version = String::new();
        let mut deprecated = String::new();
        let mut except = Exception::new();

        for line in buffer {
            let mut line_vec: Vec<String> = trim_whitespace(&line.to_string());

            if line_vec.len() > 1 {
                let len = line_vec.len();

                if len > 1 {
                    if line.contains("@param") {
                        if len == 2 {
                            parameters.push(Param {
                                name: line_vec[1].clone(),
                                desc: String::from(""),
                                var_type: String::new(),
                            })
                        } else if len > 2 {
                            let description = doc_desc(&line_vec[2..].to_vec());

                            parameters.push(Param {
                                name: line_vec[1].clone(),
                                desc: description,
                                var_type: String::new(),
                            })
                        }
                    } else if line.contains("@return") {
                        return_str = doc_desc(&line_vec[1..].to_vec());
                    } else if line.contains("@author") {
                        author = doc_desc(&line_vec[1..].to_vec());
                    } else if line.contains("@expeption") {
                        if len > 2 {
                            let mut ex = Exception {
                                exception_type: line_vec[1].clone(),
                                desc: doc_desc(&line_vec[2..].to_vec()),
                            };
                            except = ex;
                        }
                    } else if line.contains("@throws") {
                        if len > 2 {
                            let mut ex = Exception {
                                exception_type: line_vec[1].clone(),
                                desc: doc_desc(&line_vec[2..].to_vec()),
                            };
                            except = ex;
                        }
                    } else if line.contains("@version") {
                        version = doc_desc(&line_vec[1..].to_vec());
                    } else if line.contains("@deprecated") {
                        deprecated = doc_desc(&line_vec[1..].to_vec());
                    }
                }

                if !line.contains("@") {
                    desc.push_str(format!(" {} ", doc_desc(&line_vec)).as_str());
                }
            }
        }

        Doc {
            params: parameters,
            description: desc,
            return_desc: return_str,
            author: author,
            version: version,
            exception: except,
            deprecated: deprecated,
        }
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

    pub fn parse_contents(content: &str) -> Vec<String> {
        let mut tokens: Vec<String> = Vec::new();
        let mut curr_token = String::new();
        let mut block_depth = 0;
        let mut blob = content.chars();

        loop {
            let ch_res = blob.next();

            match ch_res {
                Some(ch) => {

                    match ch {
                        ' ' | '\n' | '\t' | ';' => {
                            if block_depth <= 1 && curr_token.len() > 0 {
                                tokens.push(curr_token.to_string());
                                curr_token = String::new();
                            }
                        }
                        '{' => block_depth += 1,
                        '}' => block_depth -= 1,
                        _ => {
                            if block_depth <= 1 {
                                curr_token.push_str(ch.to_string().as_str());
                            }
                        }
                    }
                },
                None => break,
            }
        }

        tokens
    }

    pub fn contrust_ast(tokens: Vec<String>) {
        let mut class = Class::new();
        let mut parse_state = ParseState::new();
        let mut jdoc = Doc::new();
        let mut jdoc_errs = String::new();

        for (i, elem) in tokens.iter().enumerate() {
            match elem.as_ref() {
                "/**" => parse_state.ch_doc(true),
                "class" => {
                    class.ch_class_name(tokens[i + 1].clone());
                }
                _ => println!("something else!"),
            }
        }
    }

    pub fn parse_file(path: &Path, lint: bool) -> Class {
        let mut doc_buffer: Vec<String> = Vec::new();
        let mut class = Class::new();
        let mut parse_state = ParseState::new();
        let mut jdoc = Doc::new();
        let mut jdoc_errs = String::new();
        let mut bracket_depth = 0;

        let file = File::open(path).expect("Could not open file");
        let mut contents = String::new();
        let mut buf = BufReader::new(file);
        buf.read_to_string(&mut contents);
        println!("{}", contents);

        let tokens = parse_contents(&contents);
        println!("{:?}", tokens);

        // Print all the errors found when linting the javadoc
        println!("{}\n", jdoc_errs);

        class
    }
}

#[cfg(test)]
mod test;
