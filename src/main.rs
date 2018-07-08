extern crate regex;

use regex::Regex;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

struct Param {
    desc: String,
    name: String,
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

fn handle_class(mut class: Class, mut state: ParseState, line: &String) -> Class {
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

    state.ch_class(true);

    return class;
}

fn parse_file(path: PathBuf) {
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
    };

    for line in buf.lines() {
        let l = line.unwrap();
        println!("{}", l);
        let line_type = determine_line_type(&l);

        match line_type {
            IsClass => class = handle_class(class, parse_state, &l),
            IsMethod => println!("Method"),
            IsComment => println!("Comment"),
            IsStartdoc => println!("Startdoc"),
            IsEnddoc => println!("Enddoc"),
            IsOther => println!("something else"),
        }
    }

    // println!("{}", &contents);
}

fn traverse_project(start_dir: &Path) {
    for f in fs::read_dir(start_dir).unwrap() {
        let p = f.unwrap().path();
        if p.extension().unwrap_or("".as_ref()) == "java" {
            parse_file(p);
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
