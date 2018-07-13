extern crate clap;
extern crate regex;
extern crate threadpool;

mod model;
mod parse;

use clap::App;
use clap::Arg;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use threadpool::ThreadPool;

pub use self::model::Class;
pub use self::model::Doc;
pub use self::model::LineType;
pub use self::model::Method;
pub use self::model::Param;
pub use self::model::ParseState;
pub use self::parse::parse_file;

fn is_java_file(file: &str) -> bool {
    let line_vec: Vec<&str> = file.split(".").collect::<Vec<&str>>();
    let l_index = line_vec.len() - 1;

    if line_vec[l_index].contains("java") {
        true
    } else {
        false
    }
}

/// Traverses the file structure to find all java files for parsing.
///
/// # Arguments
///
/// * `start_dir` - The directory to start looking for java files in.
fn find_java_files(start_dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();

    for f in fs::read_dir(start_dir).unwrap() {
        let p = f.unwrap().path();

        if p.is_dir() {
            let path = p.as_path();
            let new_files = find_java_files(path);

            for n_file in new_files {
                files.push(n_file.clone());
            }
        } else if p.is_file() {
            if is_java_file(p.as_path().file_name().unwrap().to_str().unwrap()) {
                files.push(p.clone());
            }
        }
    }

    files.clone()
}

fn generate_markdown(class: Class, dest: &str) {
    let name = format!("{}/{}.{}", dest, class.class_name, "md");
    let mut file = File::create(name).unwrap();

    let mut doc = format!("# {}\n\n", class.class_name);

    if class.description.as_str() != "" {
        doc.push_str(format!("description: {}\n", class.description.trim()).as_str());
    }
    doc.push_str(format!("privacy: {}\n", class.access.trim()).as_str());
    doc.push_str(format!("package: {}\n\n", class.package_name.trim()).as_str());
    doc.push_str("## Dependencies\n\n");

    for dep in class.dependencies {
        doc.push_str(format!("- {}\n", dep).as_str());
    }
    doc.push_str("\n## Methods\n\n");

    for member in class.methods {
        doc.push_str(format!("#### {}\n", member.name).as_str());
        doc.push_str(format!("privacy: {}\n", member.privacy.trim()).as_str());
        doc.push_str(format!("description: {}\n", member.description).as_str());
        doc.push_str(format!("return: {}\n", member.return_type).as_str());

        if member.parameters.len() > 0 {
            doc.push_str("| Name | Description |\n|_____|_____|\n");
        } else {
            doc.push_str("This method has no parameters.");
        }

        for param in member.parameters {
            doc.push_str(format!("| {} | {} |\n", param.name, param.desc).as_str());
        }

        doc.push_str("\n");
    }

    file.write(doc.as_str().as_bytes())
        .expect("Not able to write to file");
    println!("{}.{} was created", class.class_name, "md");
}

fn document(file_paths: Vec<PathBuf>, dest: String) {
    let files = Arc::new(file_paths);
    let pool = ThreadPool::new(files.len());
    let safe_dest = Arc::new(dest);

    for i in 0..files.len() {
        let file_cp = files[i].clone();
        let new_dest = safe_dest.clone();

        pool.execute(move || {
            let class = parse_file(&file_cp);
            generate_markdown(class, new_dest.as_str());
        });
    }

    pool.join();
}

fn main() {
    let matches = App::new("Javadoc-To-Markdown")
        .version("1.0")
        .author("Josh Brudnak <jobrud314@gmail.com>")
        .about("A tool for generating markdown documentation from javadocs")
        .arg(
            Arg::with_name("INPUT")
                .value_name("FILE")
                .required(true)
                .help("Sets the input directory to use")
                .index(1),
        )
        .arg(
            Arg::with_name("context")
                .help("Sets the context path of the project")
                .short("c"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .help("Generate verbose documentation for a project"),
        )
        .arg(
            Arg::with_name("destination")
                .required(false)
                .value_name("FILE")
                .short("d")
                .help("Sets the destination directory of the created markdown files"),
        )
        .get_matches();

    let dir = matches
        .value_of("INPUT")
        .expect("Documentation directory not chosen")
        .to_string();
    let dest = matches
        .value_of("destination")
        .expect("Destination directory not chosen")
        .to_string();

    fs::create_dir_all(dest.as_str()).expect("File path not able to be created");
    println!("Generating documentation from {}", dir);

    let file_paths = find_java_files(Path::new(dir.clone().as_str()));

    if file_paths.len() > 0 {
        document(file_paths, dest);
    } else {
        println!("No java files found");
    }
}
