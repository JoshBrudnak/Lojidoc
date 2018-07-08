extern crate regex;

use regex::Regex;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

struct Param {
    desc: String,
    name: String,
}

struct Doc {
    params: Vec<Param>,
    description: String,
    return_desc: String,
}

struct Method {
    parameters: Vec<Param>,
    name: String,
    privacy: String,
    description: String,
    return_type: String,
}

struct Class {
    package_name: String,
    access: String,
    class_name: String,
    description: String,
    methods: Vec<Method>,
}

struct ParseState {
    class: bool,
    method: bool,
    doc: bool,
    doc_ready: bool,
    block_depth: i32,
}

impl Class {
    fn ch_access(&mut self, value: String) {
        self.access = value;
    }
    fn ch_package_name(&mut self, value: String) {
        self.package_name = value;
    }
    fn ch_class_name(&mut self, value: String) {
        self.class_name = value;
    }
    fn ch_description(&mut self, value: String) {
        self.description = value;
    }
    fn add_method(&mut self, value: Method) {
        self.methods.push(value);
    }
}

impl ParseState {
    fn ch_class(&mut self, value: bool) {
        self.class = value;
    }
    fn ch_method(&mut self, value: bool) {
        self.method = value;
    }
    fn ch_doc(&mut self, value: bool) {
        self.doc = value;
    }
    fn ch_doc_ready(&mut self, value: bool) {
        self.doc_ready = value;
    }
    fn increase_depth(&mut self) {
        self.block_depth = self.block_depth + 1;
    }
    fn decrease_depth(&mut self) {
        if self.block_depth > 0 {
            self.block_depth = self.block_depth - 1;
        } else {
            println!("syntax error extra bracket");
        }
    }
}

impl Param {
    fn clone(&mut self) -> Param {
        let new_desc = self.desc.clone();
        let new_name = self.name.clone();

        Param {
            desc: new_desc,
            name: new_name,
        }
    }
}

enum LineType {
    IsClass,
    IsMethod,
    IsComment,
    IsStartdoc,
    IsEnddoc,
    IsOther,
}

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

    if line.contains("class ") {
        LineType::IsClass
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

fn handle_method(mut class: Class, line: &String) -> Class {
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

fn doc_desc(parts: &Vec<&str>) -> String {
    let mut description = String::from("");

    for i in 0..parts.len() {
        description.push_str(" ");
        description.push_str(parts[i]);
    }

    return description;
}

fn handle_doc(buffer: Vec<String>) -> Doc {
    let mut return_str = String::from("");
    let mut desc = String::from("");
    let mut parameters: Vec<Param> = Vec::new();

    for line in buffer {
        let line = line.split("* ").collect::<Vec<&str>>()[0];
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
        } else if !line.contains("@") {
            let split = line.split(" ");
            let parts: Vec<&str> = split.collect();

            if parts.len() > 1 {
                desc = doc_desc(&parts);
            }
        }
    }

    Doc {
        params: parameters,
        description: desc,
        return_desc: return_str,
    }
}

fn parse_file(path: PathBuf) -> Class {
    use LineType::{IsClass, IsComment, IsEnddoc, IsMethod, IsOther, IsStartdoc};

    let path_str = path.as_path();
    let file = File::open(path_str).expect("File not found");
    let buf = BufReader::new(&file);
    let mut class = Class {
        package_name: String::from(""),
        access: String::from("public"),
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
    };

    for line in buf.lines() {
        let l = line.unwrap();
        println!("{}", l);
        let line_type = determine_line_type(&l);
        let mut doc_buffer: Vec<String> = Vec::new();

        match line_type {
            IsClass => {
                class = handle_class(class, &l);
                parse_state.ch_class(true);
            }
            IsMethod => {
                if parse_state.doc_ready {
                    let mut new_params: Vec<Param> = Vec::new();

                    for i in 0..jdoc.params.len() {
                        new_params.push(jdoc.params[i].clone());
                    }

                    let j_method = Method {
                        parameters: new_params,
                        name: String::from(""),
                        privacy: String::from(""),
                        description: jdoc.description.clone(),
                        return_type: jdoc.return_desc.clone(),
                    };

                    class.add_method(j_method);
                    parse_state.ch_doc_ready(false);
                }
            }
            IsComment => println!("Comment"),
            IsStartdoc => {
                doc_buffer = Vec::new();
                parse_state.ch_doc(true);
            }
            IsEnddoc => {
                if parse_state.doc {
                    jdoc = handle_doc(doc_buffer);
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

    return class;
}

fn traverse_project(start_dir: &Path) {
    let mut classes: Vec<Class> = Vec::new();

    for f in fs::read_dir(start_dir).unwrap() {
        let p = f.unwrap().path();

        if p.extension().unwrap_or("".as_ref()) == "java" {
            let new_class = parse_file(p);
            classes.push(new_class);
        } else {
            let path = p.as_path();
            traverse_project(path);
        }
    }
}

fn main() {
    let dir = env::args().nth(1).expect("Missing argument");
    println!("Generating documentation from {}", dir);
    traverse_project(&Path::new(&dir));
}
