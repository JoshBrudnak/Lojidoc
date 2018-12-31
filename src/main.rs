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
use model::model::Options;
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
pub fn document_single(file_paths: Vec<PathBuf>, options: Options) {
    if options.verbose {
        println!("{}", lint_project(get_project(&file_paths).unwrap()));
    }

    generate_markdown(get_project(&file_paths).unwrap(), options);

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
pub fn document(file_paths: Vec<PathBuf>, options: Options) {
    let files = Arc::new(file_paths);
    let opts = Arc::new(options);
    let size = files.len();
    let mut pool_size = size / 4;

    if files.len() % 4 != 0 {
        pool_size += 1;
    }
    let pool = ThreadPool::new(pool_size);

    for i in 0..pool_size {
        let file_cp = files.clone();
        let options_cp = opts.clone();

        pool.execute(move || {
            let mut project: Project = Project::new();

            for j in 0..4 {
                if (i * 4) + j < size {
                    let mut file = file_cp[(i * 4) + j].clone();
                    let m_context = resolve_context(&file);

                    match parse_file(&file, options_cp.verbose.clone()) {
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

            let opts_deref = match Arc::try_unwrap(options_cp) {
                Ok(res) => res,
                Err(err) => panic!(err),
            };

            generate_markdown(project, opts_deref);
        });
    }

    pool.join();

    println!(
        "\nDocumentation finished. Generated {} markdown files.",
        files.len()
    );
}

fn main() {
    let options = Options::get_options();
    let file_paths = find_file_type(Path::new(options.dir.clone().as_str()), vec!["java"]);


    fs::create_dir_all(options.dest.as_str()).expect("File path not able to be created");
    println!("\nGenerating documentation from {}\n", options.dir);

    if file_paths.len() > 0 {
        if options.book {
            let mut cfg = Config::default();
            cfg.book.title = Some(String::from("Application"));

            let init_res = MDBook::init("./markdown-book").with_config(cfg).build();

            if !init_res.is_ok() {
                println!("Error initializing markdown book");
            }
        }

        if options.multi_thread {
            document(file_paths, options.clone());
        } else if options.lint {
            println!("{}", lint_project(get_project(&file_paths).unwrap()));
        } else {
            document_single(file_paths, options.clone());
        }

        if options.book.clone() {
            gen_md_book(options.dest);
        }
    } else {
        println!("No java files found");
    }
}
