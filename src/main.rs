use std::env;
use std::fs::File;
use std::io::Read;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

struct Param {
  desc: String,
  name: String
}

struct Method {
    parameters: Param
}

struct Class {
    package_name: String,
    class_name: String,

}

fn parse_file(path: PathBuf) {
    let path_str = path.as_path();
    let mut file = File::open(path_str).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Cannot read file");

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
