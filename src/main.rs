extern crate regex;
#[macro_use]
extern crate clap;

mod model;
mod parse;

use clap::App;
use clap::Arg;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub use self::model::Class;
pub use self::model::Doc;
pub use self::model::LineType;
pub use self::model::Method;
pub use self::model::Param;
pub use self::model::ParseState;
pub use self::parse::parse_file;

/// Traverses the file structure to find all java files for parsing.
///
/// # Arguments
///
/// * `start_dir` - The directory to start looking for java files in.
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

    classes
}

fn generate_markdown(classes: Vec<Class>) {
    for class in classes {
        let name = format!("{}.{}", class.class_name, "md");
        let mut file = File::create(name).unwrap();

        let mut doc = format!("# {}\n\n", class.class_name);
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

            for param in member.parameters {
                doc.push_str(format!("- parameter: {} {}\n", param.name, param.desc).as_str());
            }

            doc.push_str("\n");
        }

        file.write(doc.as_str().as_bytes())
            .expect("Not able to write to file");
        println!("{}.{} was created", class.class_name, "md");
    }
}

fn main() {
    let matches = App::new("Javadoc-To-Markdown")
                          .version("1.0")
                          .author("Josh Brudnak <jobrud314@gmail.com>")
                          .about("A tool for generating markdown documentation from javadocs")
                          .arg(Arg::with_name("INPUT")
                               .value_name("FILE")
                               .help("Sets the input directory to use")
                               .index(1))
                          .arg(Arg::with_name("context")
                               .help("Sets the context path of the project")
                               .short("c"))
                          .arg(Arg::with_name("verbose")
                               .short("v")
                               .help("Generate verbose documentation for a project"))
                          .arg(Arg::with_name("destination")
                               .short("d")
                               .help("Sets the destination directory of the created markdown files"))
                          .get_matches();

    let dir = matches.value_of("INPUT").expect("Documentation directory not chosen");

    println!("Generating documentation from {}", dir);
    let docs = get_jdocs(&Path::new(&dir));
    generate_markdown(docs);
}
