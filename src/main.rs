extern crate clap;
extern crate colored;
extern crate mdbook;
extern crate threadpool;
extern crate git2;

mod document;
mod grammar;
mod model;
mod parse;

use mdbook::config::Config;
use mdbook::MDBook;

use clap::App;
use clap::Arg;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use threadpool::ThreadPool;

use document::document::find_file_type;
use document::document::gen_md_book;
use document::document::generate_markdown;
use document::document::lint_project;
use document::document::resolve_context;
use model::model::ObjectType;
use model::model::Project;
use parse::parse::parse_file;

fn get_project<'a>(files: &Vec<PathBuf>) -> Result<Project, &'a str> {
    let mut project: Project = Project::new();

    for file in files {
        match parse_file(&file, true) {
            ObjectType::Class(mut class) => {
                class.ch_file_path(file.to_str().unwrap().to_string());
                project.add_class(class);
            }
            ObjectType::Interface(mut inter) => {
                inter.ch_file_path(file.to_str().unwrap().to_string());
                project.add_interface(inter)
            }
            ObjectType::Enumeration(mut enumeration) => {
                enumeration.ch_file_path(file.to_str().unwrap().to_string());
                project.add_enumeration(enumeration);
            }
        }
    }

    Ok(project)
}

/// Handles the single threaded option for running the application
///
/// # Arguments
///
/// * `file_paths` - A vector of the file paths of java files
/// * `dest` - The file path where the markdown will be saved
/// * `context` - The project context e.g. `github.com/user/repo`
/// * `ignore` - Permission to ignore when parsing member variables and methods
/// * `verbose` - Whether the program will output verbose logging
pub fn document_single(
    file_paths: Vec<PathBuf>,
    dest: String,
    ignore: String,
    verbose: bool,
    book: bool,
    clean: bool,
) {
    if verbose {
        println!("{}", lint_project(get_project(&file_paths).unwrap()));
    }

    generate_markdown(get_project(&file_paths).unwrap(), dest.as_str(), ignore, book, clean);

    println!(
        "\nDocumentation finished. Generated {} markdown files.",
        file_paths.len()
    );
}

/// Handles thread pooling the application
///
/// # Arguments
///
/// * `file_paths` - A vector of the file paths of java files
/// * `dest` - The file path where the markdown will be saved
/// * `ignore` - Permission to ignore when parsing member variables and methods
pub fn document(
    file_paths: Vec<PathBuf>,
    dest: String,
    ignore: String,
    verbose: bool,
    book: bool,
    clean: bool,
) {
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
        let ignore_cp = ignore.clone();

        pool.execute(move || {
            let mut project: Project = Project::new();

            for j in 0..4 {
                if (i * 4) + j < size {
                    let mut file = file_cp[(i * 4) + j].clone();
                    let m_context = resolve_context(&file);

                    match parse_file(&file, verbose) {
                        ObjectType::Class(mut class) => {
                            class.ch_file_path(m_context);
                            project.add_class(class.clone());
                        }
                        ObjectType::Interface(mut inter) => {
                            inter.ch_file_path(m_context);
                            project.add_interface(inter.clone());
                        }
                        ObjectType::Enumeration(mut enumeration) => {
                            enumeration.ch_file_path(m_context);
                            project.add_enumeration(enumeration.clone());
                        }
                    }
                }
            }

            generate_markdown(project, new_dest.as_str(), ignore_cp, book, clean);
        });
    }

    pool.join();

    println!(
        "\nDocumentation finished. Generated {} markdown files.",
        files.len()
    );
}

fn main() {
    let matches = App::new("Lojidoc")
        .version("0.3.1")
        .author("Josh Brudnak <jobrud314@gmail.com>")
        .about("A tool for generating markdown documentation for java projects")
        .arg(
            Arg::with_name("INPUT")
                .value_name("FILE")
                .required(true)
                .help("Set the input directory to use")
                .index(1),
        ).arg(
            Arg::with_name("ignore")
                .value_name("STRING")
                .required(false)
                .short("i")
                .help("Ignore fields with a certain permission"),
        ).arg(
            Arg::with_name("book")
                .value_name("FILE")
                .required(false)
                .short("b")
                .help("Use mdbook to create a book for your generated documentation"),
        ).arg(
            Arg::with_name("lint")
                .help("Check a java project for incorrect and missing javadocs")
                .short("l"),
        ).arg(
            Arg::with_name("clean")
                .help("Delete the destination directory before generating documentation")
                .short("c"),
        ).arg(
            Arg::with_name("verbose")
                .short("v")
                .help("Generate documentation for a project and provide verbose output"),
        ).arg(
            Arg::with_name("multi-thread")
                .short("m")
                .help("Use multiple threads to execute the program"),
        ).arg(
            Arg::with_name("destination")
                .required(false)
                .value_name("FILE")
                .short("d")
                .help("Sets the destination directory of the created markdown files"),
        ).get_matches();

    let dir = matches
        .value_of("INPUT")
        .expect("Documentation directory not chosen")
        .to_string();
    let dest = matches
        .value_of("destination")
        .unwrap_or("./generated/")
        .to_string();

    let book = matches.value_of("book").unwrap_or("").to_string();
    let file_paths = find_file_type(Path::new(dir.clone().as_str()), vec!["java"]);
    let multi_thread = matches.is_present("multi_thread");
    let ignore = matches.value_of("ignore").unwrap_or("").to_string();
    let lint = matches.is_present("lint");
    let clean = matches.is_present("clean");
    let verbose = matches.is_present("verbose");

    let gen_book = if book != "" { true } else { false };

    fs::create_dir_all(dest.as_str()).expect("File path not able to be created");
    println!("\nGenerating documentation from {}\n", dir);

    if file_paths.len() > 0 {
        if book != "" {
            let mut cfg = Config::default();
            cfg.book.title = Some(book.clone());

            let init_res = MDBook::init("./markdown-book").with_config(cfg).build();

            if !init_res.is_ok() {
                println!("Error initializing markdown book");
            }
        }

        if multi_thread {
            document(file_paths, dest.clone(), ignore, verbose, gen_book, clean);
        } else if lint {
            println!("{}", lint_project(get_project(&file_paths).unwrap()));
        } else {
            document_single(file_paths, dest.clone(), ignore, verbose, gen_book, clean);
        }

        if book != "" {
            gen_md_book(dest);
        }
    } else {
        println!("No java files found");
    }
}
