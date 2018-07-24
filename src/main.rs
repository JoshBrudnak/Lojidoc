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

use model::model::Class;
use model::model::Method;
use model::model::Interface;
use model::model::Project;
use model::model::LineType;
use parse::parse::parse_file;

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
pub fn find_java_files(start_dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    let file_dir = fs::read_dir(start_dir);

    if !file_dir.is_ok() {
        println!("Incorrect file path");
        return files.clone();
    }

    for f in file_dir.unwrap() {
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

/// Generates the markdown documentation for a class
///
/// # Arguments
///
/// * `class` - The class struct containing the javadoc data
pub fn gen_class_docs(class: Class) -> String {
    let mut doc = String::new();

    if class.file_path != "" {
        doc.push_str(
            format!("# Class {} [[src]]({})  \n\n", class.class_name, class.file_path)
            .as_str());
    } else {
        doc.push_str(format!("# Class {}\n\n", class.class_name).as_str());
    }

    if class.description.as_str() != "" {
        doc.push_str(format!("description: {}  \n", class.description.trim()).as_str());
    }
    doc.push_str(format!("privacy: {}  \n", class.access.trim()).as_str());
    if class.parent != "" {
        doc.push_str(format!("parent class: {}  \n", class.parent).as_str());
    }

    if class.interfaces.len() > 0 {
        doc.push_str("Interfaces:  \n");

        for inter in class.interfaces {
            doc.push_str(format!("- {}  \n", inter).as_str());
        }
    }

    doc.push_str(format!("package: {}  \n\n", class.package_name.trim()).as_str());

    if !class.exception.is_empty() {
        doc.push_str(
            format!(
                "Throws {}: {}  \n\n",
                class.exception.exception_type, class.exception.desc
            ).as_str(),
        );
    }
    doc.push_str("## Dependencies\n\n");
    doc.push_str("<details>  \n");
    doc.push_str("  <summary>  \n");
    doc.push_str("    Show dependencies  \n");
    doc.push_str("  </summary>  \n");

    doc.push_str("  <ul>  \n");
    for dep in class.dependencies {
        doc.push_str(format!("<li>{}</li>\n", dep).as_str());
    }
    doc.push_str("  </ul>  \n");
    doc.push_str("</details>  \n\n");

    doc
}

/// Generates the markdown documentation for an interface
///
/// # Arguments
///
/// * `inter` - The interface struct containing the javadoc data
pub fn gen_interface_docs(inter: Interface) -> String {
    let mut doc = String::new();

    if inter.file_path != "" {
        doc.push_str(format!("# Interface {} [[src]]({})  \n\n", inter.name, inter.file_path).as_str());

    } else {
        doc.push_str(format!("# Interface {}\n\n", inter.name).as_str());
    }

    if inter.description.as_str() != "" {
        doc.push_str(format!("description: {}  \n", inter.description.trim()).as_str());
    }
    doc.push_str(format!("privacy: {}  \n", inter.access.trim()).as_str());
    doc.push_str(format!("package: {}  \n\n", inter.package_name.trim()).as_str());
    doc.push_str("## Dependencies\n\n");
    doc.push_str("<details>  \n");
    doc.push_str("  <summary>  \n");
    doc.push_str("    Show dependencies  \n");
    doc.push_str("  </summary>  \n");

    doc.push_str("  <ul>  \n");
    for dep in inter.dependencies {
        doc.push_str(format!("    <li>{}</li>\n", dep).as_str());
    }
    doc.push_str("  </ul>  \n");
    doc.push_str("</details>  \n\n");
    doc.push_str("## Methods\n\n");

    doc
}

/// Generates the markdown documentation for the methods of a class
///
/// # Arguments
///
/// * `methods` - The vector of class methods to be documented
pub fn gen_method_docs(methods: Vec<Method>) -> String {
    let mut doc = "## Methods\n\n".to_string();

    for member in methods {
        doc.push_str(format!("### {}\n\n", member.name).as_str());

        if member.is_static {
            doc.push_str("+ Static");
        }
        doc.push_str(format!("+ privacy: {}  \n", member.privacy.trim()).as_str());
        doc.push_str(format!("+ description: {}  \n", member.description).as_str());

        if !member.exception.is_empty() {
            doc.push_str(
                format!(
                    "+ Throws {}: {}  \n",
                    member.exception.exception_type, member.exception.desc
                ).as_str(),
            );
        }
        doc.push_str(format!("+ return: {}  \n\n", member.return_type).as_str());

        if member.parameters.len() > 0 {
            doc.push_str("| Name | Type | Description |  \n");
            doc.push_str("| ----- | ----- | ----- |  \n");
        } else {
            doc.push_str("This method has no parameters.  \n");
        }

        for param in member.parameters {
            doc.push_str(
                format!(
                    "| {} | {} | {} |  \n",
                    param.name, param.var_type, param.desc
                ).as_str(),
            );
        }

        doc.push_str("\n\n");
    }

    doc
}

/// Generates a markdown file for a java file
/// Uses a Class struct to write the markdown
///
/// # Arguments
///
/// * `class` - The class struct containing the java documentation data
/// * `dest` - The file path where the markdown file will be saved
/// * `context` - The project context e.g. `github.com/user/repo`
pub fn generate_markdown(proj: Project, dest: &str) {
    for mut class in proj.classes {
        let name = format!("{}/{}.{}", dest, class.class_name, "md");
        let mut file = File::create(name).unwrap();

        let mut doc = gen_class_docs(class.clone());
        doc.push_str(gen_method_docs(class.methods).as_str());
        file.write(doc.as_str().as_bytes()).expect("Not able to write to file");

        println!("{}.{} was created", class.class_name, "md");
    }

    for mut inter in proj.interfaces {
        let name = format!("{}/{}.{}", dest, inter.name, "md");
        let mut file = File::create(name).unwrap();

        let mut doc = gen_interface_docs(inter.clone());
        doc.push_str(gen_method_docs(inter.methods).as_str());
        file.write(doc.as_str().as_bytes()).expect("Not able to write to file");

        println!("{}.{} was created", inter.name, "md");
    }
}

/// Determines whether a file path contains a git or mercurial file
///
/// # Arguments
///
/// * `file` - The repo directory file path
fn is_repo_dir(file: &str) -> bool {
    let line_vec: Vec<&str> = file.split("/").collect::<Vec<&str>>();
    let l_part = line_vec[line_vec.len() - 1];

    if l_part.contains(".git") || l_part.contains(".hg") {
        true
    } else {
        false
    }
}

/// Finds the root directory of the cloned repository
///
/// # Arguments
///
/// * `orig_path` - The java file path
fn find_repo_home(orig_path: String) -> String {
    let line_vec: Vec<&str> = orig_path.split("/").collect::<Vec<&str>>();
    let mut res = String::new();

    for i in 0..line_vec.len() {
        let mut line_p = String::new();

        for j in 0..i {
            line_p.push_str(format!("{}/", line_vec[j]).as_str());
        }

        let file_dir = fs::read_dir(line_p);

        if file_dir.is_ok() {
            for f in file_dir.unwrap() {
                let p = f.unwrap().path();

                if p.is_dir() {
                    let p_str = p.as_path().to_str().unwrap();
                    if is_repo_dir(&p_str) {
                        let res_str = p.parent().unwrap().as_os_str().to_str().unwrap();
                        res = res_str.to_string().clone();
                        break;
                    }
                }
            }
        }
    }

    res
}

/// Combines the repo url with java file path to provide a link in the docs
///
/// # Arguments
///
/// * `paths` - The java file path
/// * `context` - Url of the git or mercurial repository
pub fn resolve_context(path: PathBuf, context: &String) -> String {
    let p = path.to_str().unwrap();
    let line_vec: Vec<&str> = p.split("/").collect::<Vec<&str>>();
    let mut part = line_vec[0].to_string();
    part.push_str("/");

    let repo_root = find_repo_home(p.to_string());
    let line_vec: Vec<&str> = p.split(repo_root.as_str()).collect::<Vec<&str>>();
    let mut new_context = context.clone();
    new_context.push_str(line_vec.join("").as_str());

    new_context
}

/// Handles linting javadocs without saving the documentation
///
/// # Arguments
///
/// * `file_paths` - A vector of the file paths of java files
/// * `dest` - The file path where the markdown will be saved
pub fn lint_javadoc(file_paths: Vec<PathBuf>, dest: String) {
    let mut project: Project = Project::new();

    for file in file_paths.clone() {
         let mut class = parse_file(&file, true);

         if !class.is_class {
             project.add_interface(class.to_interface());
         } else {
             project.add_class(class.clone());
         }
    }

    generate_markdown(project, dest.as_str());
    println!("\nDocumentation finished. Generated {} markdown files.", file_paths.len());
}

/// Handles the single threaded option for running the application
///
/// # Arguments
///
/// * `file_paths` - A vector of the file paths of java files
/// * `dest` - The file path where the markdown will be saved
/// * `context` - The project context e.g. `github.com/user/repo`
/// * `verbose` - Whether the program will output verbose logging
pub fn document_single(file_paths: Vec<PathBuf>, dest: String, context: String, verbose: bool) {
    let mut project: Project = Project::new();

    for file in file_paths.clone() {
         let mut class = parse_file(&file, verbose);

         let m_context = resolve_context(file, &context);

         if m_context != "" {
             class.ch_file_path(m_context);
         }
         if !class.is_class {
             project.add_interface(class.to_interface());
         } else {
             project.add_class(class.clone());
         }
    }

    generate_markdown(project, dest.as_str());
    println!("\nDocumentation finished. Generated {} markdown files.", file_paths.len());
}

/// Handles the thread pooling the application
///
/// # Arguments
///
/// * `file_paths` - A vector of the file paths of java files
/// * `dest` - The file path where the markdown will be saved
pub fn document(file_paths: Vec<PathBuf>, dest: String, context: String, verbose: bool) {
    let files = Arc::new(file_paths);
    let size = files.len();
    let mut pool_size = size / 4;
    if files.len() % 4 != 0 {
        pool_size += 1;
    }
    let pool = ThreadPool::new(pool_size);
    let safe_dest = Arc::new(dest);

    for i in 0..pool_size {
        let file_cp = files.clone();
        let new_dest = safe_dest.clone();
        let new_context = context.clone();

        pool.execute(move || {
            let mut project: Project = Project::new();

            for j in 0..4 {
                if (i * 4) + j < size {
                    let mut file = file_cp[(i * 4) + j].clone();
                    let mut class = parse_file(&file, verbose);
                    let m_context = resolve_context(file, &new_context);

                    if m_context != "" {
                        class.ch_file_path(m_context);
                    }
                    if !class.is_class {
                        project.add_interface(class.to_interface());
                    } else {
                        project.add_class(class.clone());
                    }
                }
            }

            generate_markdown(project, new_dest.as_str());
        });
    }

    pool.join();

    println!("\nDocumentation finished. Generated {} markdown files.", files.len());
}

fn main() {
    let matches = App::new("Javadoc-To-Markdown")
        .version("1.0.0")
        .author("Josh Brudnak <jobrud314@gmail.com>")
        .about("A tool for generating markdown documentation for java projects")
        .arg(
            Arg::with_name("INPUT")
                .value_name("FILE")
                .required(true)
                .help("Set the input directory to use")
                .index(1),
        )
        .arg(
            Arg::with_name("context")
                .help("Set the context path of the project")
                .value_name("FILE")
                .short("c"),
        )
        .arg(
            Arg::with_name("lint")
                .help("Check a java project for incorrent and missing javadocs")
                .short("l")
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .help("Generate documentation for a project and provide verbose output"),
        )
        .arg(
            Arg::with_name("single-thread")
                .short("s")
                .help("Use only on thread for execution of the program"),
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
        .unwrap_or("./generated/")
        .to_string();
    let context = matches
        .value_of("context")
        .unwrap_or("")
        .to_string();
    let file_paths = find_java_files(Path::new(dir.clone().as_str()));
    let single_thread = matches.is_present("single_thread");
    let lint = matches.is_present("lint");
    let verbose = matches.is_present("verbose");

    fs::create_dir_all(dest.as_str()).expect("File path not able to be created");
    println!("Generating documentation from {}", dir);

    if file_paths.len() > 0 {
        if single_thread {
            document_single(file_paths, dest, context, verbose);
        } else if lint {
            lint_javadoc(file_paths, dest);
        } else {
            document(file_paths, dest, context, verbose);
        }

    } else {
        println!("No java files found");
    }
}
