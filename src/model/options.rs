use clap::{ App, Arg};

#[derive(Debug, Clone)]
/// Struct for flags and options passed to Lojidoc
pub struct Options {
    pub clean: bool,
    pub lint: bool,
    pub include_def: bool,
    pub multi_thread: bool,
    pub verbose: bool,
    pub book: bool,
    pub dest: String,
    pub dir: String,
    pub ignore: String,
}

impl Options {
    pub fn get_options() -> Options {
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
           Arg::with_name("include-def")
               .short("s")
               .required(false)
               .help("Include the object, method, and variable signatures in the documentation"),
        ).arg(
           Arg::with_name("book")
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

        Options {
            clean: matches.is_present("clean"),
            lint: matches.is_present("lint"),
            include_def: matches.is_present("include_def"),
            verbose: matches.is_present("verbose"),
            book: matches.is_present("book"),
            ignore: matches.value_of("ignore").unwrap_or("").to_string(),
            multi_thread: matches.is_present("multi_thread"),
            dest: matches
                .value_of("destination")
                .unwrap_or("./generated/")
                .to_string(),
            dir: matches
                .value_of("INPUT")
                .expect("Documentation directory not chosen")
                .to_string(),
        }
    }
}
