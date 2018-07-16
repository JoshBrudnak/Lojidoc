pub mod parse {
    //! A module which handles the parsing for java files
    extern crate regex;

    use model::model::*;

    use regex::Regex;
    use std::fs::File;
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

    fn trim_paren(part: String) -> String {
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

        if line.contains("package ") {
            LineType::IsPackage
        } else if line.contains("class ") && !state.comment && !state.doc {
            LineType::IsClass
        } else if line.contains("import ") {
            LineType::IsImport
        } else if regex_match(&line, method_match) {
            LineType::IsMethod
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

    fn handle_class(mut class: Class, line: &String) -> Class {
        let access_match = r"(public|protected|private)";
        let split = line.split(" ");
        let parts: Vec<&str> = split.collect();

        for (num, class_part) in parts.iter().enumerate() {
            if regex_match(&class_part, access_match) {
                class.ch_access(class_part.clone().to_string());
            } else if class_part.contains("class") {
                class.ch_class_name(parts[num + 1].to_string());
            }
        }

        return class;
    }

    fn handle_method(line: &String) -> Result<Method, &str> {
        let access_match = r"(public|protected|private)";
        let parts = trim_whitespace(line);

        let mut method = Method {
            parameters: Vec::new(),
            exceptions: Vec::new(),
            name: String::from(""),
            privacy: String::from(""),
            description: String::from(""),
            return_type: String::from(""),
        };

        for (i, method_part) in parts.iter().enumerate() {
            if regex_match(&method_part, access_match) {
                method.ch_privacy(method_part.clone().to_string());
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
                        method.add_param(Param {
                            desc: String::new(),
                            var_type: param_type.clone(),
                            name: meth_part,
                        });
                        param_def = false;
                    } else {
                        param_type = meth_part;
                        param_def = true;
                    }
                }

                method.ch_method_name(name_parts[0].to_string());
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
                if b > 32 {
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
        let mut except: Vec<String> = Vec::new();

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
                            let description = doc_desc(&line_vec[1..].to_vec());

                            parameters.push(Param {
                                name: line_vec[2].clone(),
                                desc: description,
                                var_type: String::new(),
                            })
                        }
                    } else if line.contains("@return") {
                        return_str = doc_desc(&line_vec[1..].to_vec());
                    } else if line.contains("@author") {
                        author = doc_desc(&line_vec[1..].to_vec());
                    } else if line.contains("@expeption") {
                        except.push(doc_desc(&line_vec[1..].to_vec()));
                    } else if line.contains("@version") {
                        version = doc_desc(&line_vec[1..].to_vec());
                    } else if line.contains("@deprecated") {
                        deprecated = doc_desc(&line_vec[1..].to_vec());
                    }
                }

                if !line.contains("@") {
                    desc.push_str(format!(" {} ", doc_desc(&line_vec[1..].to_vec())).as_str());
                }
            }
        }

        Doc {
            params: parameters,
            description: desc,
            return_desc: return_str,
            author: author,
            version: version,
            exceptions: except,
            deprecated: deprecated,
        }
    }

    fn match_params(params: &Vec<Param>, jparams: &Vec<Param>) -> Vec<Param> {
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
                    println!("Same {:?}", param);
                    found = true;
                }
            }

            if !found {
                    new_param.push(Param {
                        name: param.name.clone(),
                        var_type: param.var_type.clone(),
                        desc: "No description found".to_string(),
                    });
                    println!("missing javadoc parameter: {}", param.name);
            }
        }

        new_param
    }

    pub fn parse_file(path: &Path) -> Class {
        use LineType::{
            IsClass, IsComment, IsEnddoc, IsImport, IsMethod, IsOther, IsPackage, IsStartdoc,
        };

        let file = File::open(path).expect("File not found");
        let buf = BufReader::new(&file);
        let mut doc_buffer: Vec<String> = Vec::new();

        let mut class = Class {
            package_name: String::from(""),
            dependencies: Vec::new(),
            deprecation: String::from(""),
            access: String::from(""),
            version: String::from(""),
            author: String::from(""),
            class_name: String::from(""),
            description: String::from(""),
            methods: Vec::new(),
        };
        let mut parse_state = ParseState {
            class: false,
            method: false,
            doc: false,
            comment: false,
            doc_ready: false,
            block_depth: 0,
        };
        let mut jdoc = Doc {
            params: Vec::new(),
            description: String::from(""),
            return_desc: String::from(""),
            author: String::from(""),
            version: String::from(""),
            exceptions: Vec::new(),
            deprecated: String::from(""),
        };

        for line in buf.lines() {
            let l = line.unwrap();
            let line_type = determine_line_type(&l, &parse_state);

            match line_type {
                IsPackage => {
                    let split = l.split(" ");
                    let parts: Vec<&str> = split.collect();

                    for (num, w) in parts.iter().enumerate() {
                        if w.contains("package") {
                            let mut pack_name = &parts[num + 1].trim();
                            class.ch_package_name(pack_name.to_string().replace(";", ""));
                        }
                    }
                }
                IsImport => {
                    let split = l.split(" ");
                    let mut parts: Vec<&str> = split.collect();

                    for (num, w) in parts.iter().enumerate() {
                        if w.contains("import") {
                            let mut im_name = &parts[num + 1].trim();
                            class.add_dependency(im_name.to_string().replace(";", ""));
                        }
                    }
                }
                IsClass => {
                    if !parse_state.class {
                        class = handle_class(class, &l);

                        if parse_state.doc_ready {
                            class.ch_description(jdoc.description.clone());
                            class.ch_author(jdoc.author.clone());
                            class.ch_version(jdoc.version.clone());
                            class.ch_deprecation(jdoc.deprecated.clone());
                        }
                        parse_state.ch_class(true);
                        parse_state.ch_doc_ready(false);
                    }
                }
                IsMethod => {
                    let method_res = handle_method(&l);

                    if method_res.is_ok() {
                        let mut j_method = method_res.unwrap();

                        if parse_state.doc_ready {
                            j_method.ch_description(jdoc.description.clone());
                            j_method.ch_return_type(jdoc.return_desc.clone());

                            let n_params: Vec<Param> = match_params(&j_method.parameters, &jdoc.params);
                            j_method.ch_params(n_params);

                            for i in 0..jdoc.exceptions.len() {
                                j_method.add_exception(jdoc.exceptions[i].clone());
                            }
                        }

                        class.add_method(j_method);
                    }
                    parse_state.ch_doc_ready(false);
                }
                IsComment => {}
                IsStartdoc => {
                    doc_buffer.clear();
                    parse_state.ch_doc(true);
                }
                IsEnddoc => {
                    if parse_state.doc {
                        jdoc = handle_doc(&doc_buffer);
                        parse_state.ch_doc(false);
                        parse_state.ch_doc_ready(true);
                    }
                }
                IsOther => {
                    if parse_state.doc {
                        doc_buffer.push(l);
                    }
                }
            }
        }

        class
    }
}
