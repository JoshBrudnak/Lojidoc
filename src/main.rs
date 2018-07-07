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
    class_name: String,
    description: String,
    methods: Vec<Method>,
}

enum LineType {
    Class,
    Method,
    Comment,
    Startdoc,
    Enddoc,
    Other,
}

fn regex_match(text: &str, regex_str: &str) -> bool {
    let reg = Regex::new(regex_str).unwrap();

    reg.is_match(text)
}

fn determine_line_type(line: String) -> LineType {
    let method_match = "(public|protected|private|static|\\s) +[\\w\\<\\>\\[\\]]+\\s+(\\w+) *\\([^\\)]*\\) *(\\{?|[^;])";
    let start_doc_match = "(\\/\\*\\*)";
    let end_doc_match = "(\\*\\*\\/)";
    let comment_match = "(\\/\\/)";

    if line.contains("class ") {
        LineType::Class
    } else if regex_match(&line, method_match) {
        LineType::Method
    } else if regex_match(&line, start_doc_match) {
        LineType::Startdoc
    } else if regex_match(&line, end_doc_match) {
        LineType::Enddoc
    } else if regex_match(&line, comment_match) {
        LineType::Comment
    } else {
        LineType::Other
    }
}

fn parse_file(path: PathBuf) {
    use LineType::{Class, Comment, Enddoc, Method, Other, Startdoc};

    let path_str = path.as_path();
    let mut file = File::open(path_str).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot read file");

    let mut buf = BufReader::new(file);
    let mut done = false;

    while !done {
        if buf.fill_buf().unwrap().len() > 0 {
            let mut line = String::new();
            let len = buf.read_line(&mut line);
            let line_type = determine_line_type(line);
        }
    }

    println!("{}", &contents);
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
