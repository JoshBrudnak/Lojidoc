extern crate regex;

use regex::Regex;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
struct Param {
    desc: String,
    name: String,
}

struct Doc {
    params: Vec<Param>,
    description: String,
    return_desc: String,
}

#[derive(Debug)]
struct Method {
    parameters: Vec<Param>,
    name: String,
    privacy: String,
    description: String,
    return_type: String,
}

#[derive(Debug)]
struct Class {
    package_name: String,
    access: String,
    class_name: String,
    description: String,
    dependencies: Vec<String>,
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
    fn add_dependency(&mut self, value: String) {
        self.dependencies.push(value);
    }
}

impl Method {
    fn ch_privacy(&mut self, value: String) {
        self.privacy = value;
    }
    fn ch_method_name(&mut self, value: String) {
        self.name = value;
    }
    fn ch_description(&mut self, value: String) {
        self.description = value;
    }
    fn add_param(&mut self, value: Param) {
        self.parameters.push(value);
    }
    fn ch_return_type(&mut self, value: String) {
        self.return_type = value;
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
    IsPackage,
    IsImport,
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

    for (num, method_part) in parts.iter().enumerate() {
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
            } else if !line.contains("@") {
                desc = line.to_string();
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
    use LineType::{IsImport, IsPackage, IsClass, IsComment, IsEnddoc, IsMethod, IsOther, IsStartdoc};

    let path_str = path.as_path();
    let file = File::open(path_str).expect("File not found");
    let buf = BufReader::new(&file);
    let mut doc_buffer: Vec<String> = Vec::new();

    let mut class = Class {
        package_name: String::from(""),
        dependencies: Vec::new(),
        access: String::from(""),
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
                    parse_state.ch_class(true);
                }
            }
            IsMethod => {
                let mut j_method = Method {
                    parameters: Vec::new(),
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
                }

                j_method = handle_method(j_method, &l);

                class.add_method(j_method);
                parse_state.ch_doc_ready(false);
            }
            IsComment => println!("Comment"),
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

fn get_jdocs(start_dir: &Path) -> Vec<Class> {
    let mut classes: Vec<Class> = Vec::new();

    for f in fs::read_dir(start_dir).unwrap() {
        let p = f.unwrap().path();

        if p.extension().unwrap_or("".as_ref()) == "java" {
            let new_class = parse_file(p);
            classes.push(new_class);
        } else {
            let path = p.as_path();
            get_jdocs(path);
        }
    }

    return classes;
}

fn generate_markdown(classes: Vec<Class>) {
    for class in classes {
        let name = format!("{}.{}", class.class_name, "md");
        let mut file = File::create(name).unwrap();

        let class_name = format!("# {}\n\n", class.class_name);
        let class_access = format!("privacy: {}\n", class.access.trim());
        let class_package = format!("package: {}\n\n", class.package_name.trim());
        file.write(class_name.as_bytes()).unwrap();
        file.write(class_access.as_bytes()).unwrap();
        file.write(class_package.as_bytes()).unwrap();
        file.write(b"## Dependencies\n\n").unwrap();

        for dep in class.dependencies {
            let class_dep = format!("- {}\n", dep);
            file.write(class_dep.as_bytes()).unwrap();
        }

        file.write(b"\n## Methods\n\n").unwrap();

        for member in class.methods {
            let method_name = format!("#### {}\n\n", member.name);
            let method_privacy = format!("privacy: {}\n", member.privacy.trim());
            let method_desc = format!("description: {}\n", member.description);
            let method_return = format!("return: {}\n\n", member.return_type);

            file.write(method_name.as_bytes()).unwrap();
            file.write(method_privacy.as_bytes()).unwrap();
            file.write(method_desc.as_bytes()).unwrap();
            file.write_all(method_return.as_bytes()).unwrap();

            for param in member.parameters {
                let method_name = format!("- parameter: {} {}\n", param.name, param.desc);
                file.write_all(method_name.as_bytes()).unwrap();
                file.write_all(b"\n").unwrap();
            }
        }

        let class_name = format!("{}.{}", class.class_name, "md");
        println!("{} was created", class_name);
    }
}

fn main() {
    let dir = env::args().nth(1).expect("Missing argument");
    println!("Generating documentation from {}", dir);
    let docs = get_jdocs(&Path::new(&dir));
    generate_markdown(docs);
}
