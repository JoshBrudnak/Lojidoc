extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub use model::Class;
pub use model::Doc;
pub use model::LineType;
pub use model::Method;
pub use model::Param;
pub use model::ParseState;

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

fn end_doc_match(text: &String) -> bool {
    if text.contains(r"**/") {
        return true;
    } else if text.contains(r"*/") {
        return true;
    } else {
        return false;
    }
}

fn determine_line_type(line: &String) -> LineType {
    let method_match = r"(public|protected|private|static|\s) +[\w\[\]]+\s+(\w+) *\([^\)]*\)";

    if line.contains("package ") {
        LineType::IsPackage
    } else if line.contains("class ") {
        LineType::IsClass
    } else if line.contains("import ") {
        LineType::IsImport
    } else if regex_match(&line, method_match) {
        LineType::IsMethod
    } else if start_doc_match(&line) {
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

fn handle_method(mut method: Method, line: &String) -> Method {
    let access_match = r"(public|protected|private)";
    let split = line.split(" ");
    let parts: Vec<&str> = split.collect();

    for method_part in parts {
        if regex_match(&method_part, access_match) {
            method.ch_privacy(method_part.clone().to_string());
        } else if method_part.contains("(") {
            let name_split = method_part.split("(");
            let name_parts: Vec<&str> = name_split.collect();

            method.ch_method_name(name_parts[0].to_string());
        }
    }

    return method;
}

fn doc_desc(parts: &Vec<&str>) -> String {
    let mut description = String::from("");

    for i in 0..parts.len() {
        description.push_str(" ");
        description.push_str(parts[i]);
    }

    return description;
}

fn handle_doc(buffer: &Vec<String>) -> Doc {
    let mut return_str = String::from("");
    let mut desc = String::from("");
    let mut parameters: Vec<Param> = Vec::new();
    let mut author = String::new();
    let mut version = String::new();
    let mut deprecated = String::new();
    let mut except: Vec<String> = Vec::new();

    for line in buffer {
        let line_vec: Vec<&str> = line.split("* ").collect::<Vec<&str>>();

        if line_vec.len() > 1 {
            let line = line_vec[1];

            if line.contains("@param") {
                let split = line.split(" ");
                let parts: Vec<&str> = split.collect();

                if parts.len() == 2 {
                    parameters.push(Param {
                        name: parts[1].to_string(),
                        desc: String::from(""),
                    })
                } else if parts.len() > 2 {
                    let description = doc_desc(&parts[2..].to_vec());

                    parameters.push(Param {
                        name: parts[1].to_string(),
                        desc: description,
                    })
                }
            } else if line.contains("@return") {
                let split = line.split(" ");
                let parts: Vec<&str> = split.collect();

                if parts.len() > 1 {
                    return_str = doc_desc(&parts[1..].to_vec());
                }
            } else if line.contains("@author") {
                let split = line.split(" ");
                let parts: Vec<&str> = split.collect();

                if parts.len() > 1 {
                    author = doc_desc(&parts[1..].to_vec());
                }
            } else if line.contains("@expeption") {
                let split = line.split(" ");
                let parts: Vec<&str> = split.collect();

                if parts.len() > 1 {
                    except.push(doc_desc(&parts[1..].to_vec()));
                }
            } else if line.contains("@version") {
                let split = line.split(" ");
                let parts: Vec<&str> = split.collect();

                if parts.len() > 1 {
                    version = doc_desc(&parts[1..].to_vec());
                }
            } else if line.contains("@deprecated") {
                let split = line.split(" ");
                let parts: Vec<&str> = split.collect();

                if parts.len() > 1 {
                    deprecated = doc_desc(&parts[1..].to_vec());
                }
            } else if !line.contains("@") {
                desc.push_str(format!("  {}\n", line.trim()).as_str());
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
        let line_type = determine_line_type(&l);

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
                let mut j_method = Method {
                    parameters: Vec::new(),
                    exceptions: Vec::new(),
                    name: String::from(""),
                    privacy: String::from(""),
                    description: String::from(""),
                    return_type: String::from(""),
                };

                if parse_state.doc_ready {
                    j_method.ch_description(jdoc.description.clone());
                    j_method.ch_return_type(jdoc.return_desc.clone());

                    for i in 0..jdoc.params.len() {
                        j_method.add_param(jdoc.params[i].clone());
                    }

                    for i in 0..jdoc.exceptions.len() {
                        j_method.add_exception(jdoc.exceptions[i].clone());
                    }
                }

                j_method = handle_method(j_method, &l);

                class.add_method(j_method);
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
