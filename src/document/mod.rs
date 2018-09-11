pub mod document {
    extern crate colored;

    use mdbook::MDBook;

    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    use std::path::PathBuf;

    use colored::*;
    use model::contents::ApplicationDoc;
    use model::model::Class;
    use model::model::Interface;
    use model::model::Member;
    use model::model::Method;
    use model::model::Project;

    /// Finds out whether a file is a java file
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

        // If the start directory is a single file return only that path
        if start_dir.is_file() {
            files.push(start_dir.to_path_buf());
            return files;
        }

        let file_dir = fs::read_dir(start_dir);

        if !file_dir.is_ok() {
            println!("Incorrect file path");
            return files;
        }

        // For every file or directory in the current directory find java files
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

    /// Traverses the file structure to find all java files for parsing.
    ///
    /// # Arguments
    ///
    /// * `start_dir` - The directory to start looking for java files in.
    pub fn find_gen_files(gen_dir: &Path) -> Vec<String> {
        let mut files: Vec<String> = Vec::new();
        let file_dir = fs::read_dir(gen_dir);

        if file_dir.is_ok() {
            for f in file_dir.unwrap() {
                if f.is_ok() {
                    let p = f.unwrap().path();

                    if p.is_file() {
                        let file = p.as_path().file_name().unwrap().to_str().unwrap();
                        let line_vec: Vec<&str> = file.split(".").collect::<Vec<&str>>();
                        let l_index = line_vec.len() - 1;

                        if line_vec[l_index].contains("md") {
                            files.push(line_vec[0].to_string());
                        }
                    }
                }
            }
        } else {
            println!("Generated directory read error: {:?}", gen_dir);
        }

        files
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
                format!("# Class {} [[src]]({})  \n\n", class.name, class.file_path).as_str(),
            );
        } else {
            doc.push_str(format!("# Class {}\n\n", class.name).as_str());
        }

        if class.license != "" {
            doc.push_str("<details>  \n");
            doc.push_str("  <summary>  \n");
            doc.push_str("    Show license  \n\n");
            doc.push_str("  </summary>  \n");

            doc.push_str("  <ul>  \n");
            doc.push_str(class.license.as_str());
            doc.push_str("  </ul>  \n");
            doc.push_str("</details>  \n\n");
            doc.push_str("<br/>");
        }

        doc.push_str(format!("Access: {}  \n", class.access.trim()).as_str());
        if class.description.as_str() != "" {
            doc.push_str(format!("Description:  \n > {}  \n\n", class.description.trim()).as_str());
        }
        if class.author != "" {
            doc.push_str(format!("Author: {}  \n", class.author).as_str());
        }
        if class.version != "" {
            doc.push_str(format!("Since version: {}  \n", class.version).as_str());
        }
        if class.parent != "" {
            doc.push_str(format!("Parent class: {}  \n", class.parent).as_str());
        }

        if class.interfaces.len() > 0 {
            doc.push_str("Interfaces:  \n");

            for inter in class.interfaces {
                doc.push_str(format!("- {}  \n", inter).as_str());
            }
            doc.push_str("\n");
        }

        doc.push_str(format!("package: {}  \n\n", class.package_name.trim()).as_str());

        if class.exceptions.len() > 0 {
            for exception in class.exceptions {
                doc.push_str(
                    format!(
                        "Throws {}: {}  \n\n",
                        exception.exception_type, exception.desc
                    ).as_str(),
                );
            }
            doc.push_str("\n");
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
            doc.push_str(
                format!(
                    "# Interface {} [[src]]({})  \n\n",
                    inter.name, inter.file_path
                ).as_str(),
            );
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

        doc
    }

    /// Generates the markdown documentation for the member variables of a class
    ///
    /// # Arguments
    ///
    /// * `variables` - The vector of class methods to be documented
    pub fn gen_var_docs(variables: Vec<Member>, path: String) -> String {
        let mut doc = String::new();

        if variables.len() > 0 {
            doc.push_str("## Member Variables\n\n");
        } else {
            doc.push_str("## No member variables in this class\n\n");

            return doc;
        }

        for member in variables {
            if path != "" {
                let mut file_path = path.clone();
                file_path.push_str(format!("#L{}", member.line_num).as_str());
                doc.push_str(
                    format!(
                        "#### {} {} [[src]]({})\n\n",
                        member.var_type, member.name, file_path
                    ).as_str(),
                );
            } else {
                doc.push_str(format!("#### {} {}\n\n", member.var_type, member.name).as_str());
            }

            if member.desc != "" {
                doc.push_str(format!("+ Description: {}  \n", member.desc).as_str());
            }

            if member.access == "" {
                doc.push_str("+ Access: package-private  \n");
            } else {
                doc.push_str(format!("+ Access: {}  \n", member.access).as_str());
            }

            if member.modifiers.len() > 0 {
                doc.push_str("+ Modifiers: ");

                for mem in member.modifiers {
                    doc.push_str(format!("{} ", mem).as_str())
                }

                doc.push_str("\n");
            }

            doc.push_str("\n");
        }

        doc
    }

    /// Generates the markdown documentation for the methods of a class
    ///
    /// # Arguments
    ///
    /// * `methods` - The vector of class methods to be documented
    pub fn gen_method_docs(methods: Vec<Method>, path: String) -> String {
        let mut doc = String::new();

        if methods.len() > 0 {
            doc.push_str("## Methods\n\n");
        } else {
            doc.push_str("## No methods in this class\n\n");

            return doc;
        }

        for member in methods {
            if member.name != String::from("") {
                if path != "" {
                    let mut file_path = path.clone();
                    file_path.push_str(format!("#L{}", member.line_num).as_str());
                    doc.push_str(
                        format!("### {} [[src]]({})\n\n", member.name, file_path).as_str(),
                    );
                } else {
                    doc.push_str(format!("### {}\n\n", member.name).as_str());
                }

                doc.push_str(format!("+ Description: {}  \n", member.description).as_str());

                if member.privacy == "" {
                    doc.push_str("+ Access: package-private  \n");
                } else {
                    doc.push_str(format!("+ Access: {}  \n", member.privacy).as_str());
                }

                if member.modifiers.len() > 0 {
                    doc.push_str("+ Modifiers: ");

                    for mem in member.modifiers {
                        doc.push_str(format!("{} ", mem).as_str())
                    }

                    doc.push_str("\n");
                }

                for exception in member.exceptions {
                    doc.push_str(
                        format!(
                            "+ Throws {}: {}  \n",
                            exception.exception_type, exception.desc
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
    pub fn generate_markdown(proj: Project, dest: &str, book: bool) {
        let mut app_doc = ApplicationDoc::new();

        for mut class in proj.classes {
            let name = format!("{}/{}.{}", dest, class.name, "md");
            let mut file = File::create(name).unwrap();

            let mut doc = gen_class_docs(class.clone());
            doc.push_str(gen_var_docs(class.variables, class.file_path.clone()).as_str());
            doc.push_str(gen_method_docs(class.methods, class.file_path).as_str());
            file.write(doc.as_str().as_bytes())
                .expect("Not able to write to file");

            if book {
                let name = format!("./markdown-book/src/{}.{}", class.name, "md");
                let mut file = File::create(name).unwrap();

                file.write(doc.as_str().as_bytes())
                    .expect("Not able to write to file");
            }

            app_doc.add_package_class(class.package_name, class.name.clone());

            println!("{}.{} was created", class.name, "md");
        }

        for mut inter in proj.interfaces {
            let name = format!("{}/{}.{}", dest, inter.name, "md");
            let mut file = File::create(name).unwrap();

            let mut doc = gen_interface_docs(inter.clone());
            doc.push_str(gen_var_docs(inter.variables, inter.file_path.clone()).as_str());
            doc.push_str(gen_method_docs(inter.methods, inter.file_path).as_str());
            file.write(doc.as_str().as_bytes())
                .expect("Not able to write to file");

            println!("{}.{} was created", inter.name, "md");
        }
    }

    pub fn lint_method(method: &Method) -> String {
        let mut method_errs = String::new();

        if method.description == "" {
            method_errs.push_str(
                "\tMissing description for method "
                    .yellow()
                    .to_string()
                    .as_str(),
            );
            method_errs.push_str(format!("{} (Line: {})\n", method.name, method.line_num).as_str());
        }
        if method.return_type == "" {
            method_errs.push_str(
                "\tMissing return type for method "
                    .yellow()
                    .to_string()
                    .as_str(),
            );
            method_errs.push_str(format!("{} (Line: {})\n", method.name, method.line_num).as_str());
        }

        for p in method.parameters.clone() {
            if p.desc == "" {
                method_errs.push_str(
                    "\tJavadoc parameter not found "
                        .yellow()
                        .to_string()
                        .as_str(),
                );
                method_errs.push_str(
                    format!(
                        "{} in method: {} (Line: {})\n",
                        p.name, method.name, method.line_num
                    ).as_str(),
                );
            }
        }

        method_errs
    }
    pub fn lint_var(var: &Member) -> String {
        let mut errs = String::new();

        if var.desc == "" {
            errs.push_str(
                "\tMissing description for member variable "
                    .yellow()
                    .to_string()
                    .as_str(),
            );
            errs.push_str(format!("{} (Line: {})\n", var.name, var.line_num).as_str());
        }
        if var.var_type == "" {
            errs.push_str(
                "\tMissing return type for member variable "
                    .yellow()
                    .to_string()
                    .as_str(),
            );
            errs.push_str(format!("{} (Line: {})\n", var.name, var.line_num).as_str());
        }

        errs
    }

    /// Lints the java project's javadoc comments and prints the errors
    ///
    /// # Arguments
    ///
    /// * `proj` - The project to lint
    pub fn lint_project(proj: Project) -> String {
        let mut jdoc_errs = String::new();

        for mut class in proj.classes {
            let mut temp_err = String::new();

            for v in class.variables {
                temp_err.push_str(lint_var(&v).as_str());
            }
            for m in class.methods {
                temp_err.push_str(lint_method(&m).as_str());
            }

            if temp_err != "" {
                jdoc_errs.push_str(
                    "Javadoc errors for class "
                        .green()
                        .bold()
                        .to_string()
                        .as_str(),
                );
                jdoc_errs.push_str(
                    format!(
                        "{}\nFile: {}\n",
                        class.name,
                        class.file_path.as_str().blue().to_string()
                    ).as_str(),
                );
                jdoc_errs.push_str(format!("{}\n", temp_err).as_str());
            }
        }

        for mut inter in proj.interfaces {
            let mut temp_err = String::new();

            for v in inter.variables {
                temp_err.push_str(lint_var(&v).as_str());
            }
            for m in inter.methods {
                temp_err.push_str(lint_method(&m).as_str());
            }

            if temp_err != "" {
                jdoc_errs.push_str(
                    "Javadoc errors for interface "
                        .green()
                        .bold()
                        .to_string()
                        .as_str(),
                );
                jdoc_errs.push_str(
                    format!(
                        "{}\nFile: {}\n",
                        inter.name,
                        inter.file_path.as_str().blue().to_string()
                    ).as_str(),
                );
                jdoc_errs.push_str(format!("{}\n", temp_err).as_str());
            }
        }

        jdoc_errs
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
    pub fn resolve_context(path: &PathBuf, context: &String) -> String {
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

    /// Creates a markdown book using mdbook. Uses the files in the generated documentation
    /// directory for chapters.
    ///
    /// # Arguments
    ///
    /// * `gen_dir` - The directory containing the generated documentation
    pub fn gen_md_book(gen_dir: String) {
        let name = "./markdown-book/src/SUMMARY.md";
        let res_file = File::create(name);
        let files = find_gen_files(&PathBuf::from(gen_dir.as_str()));
        let mut doc = String::new();

        if res_file.is_ok() {
            let mut file = res_file.unwrap();

            for f in files {
                let file_path = format!("./{}.md", f.clone());
                doc.push_str(format!("- [{}]({})  \n", f, file_path).as_str());
            }

            file.write(doc.as_str().as_bytes())
                .expect("Not able to write to file");

            let md = MDBook::load("./markdown-book").expect("Unable to load the book");
            md.build().expect("Building failed");

            println!("Generated the markdown book");
        } else {
            println!("Error creating file: {:?}", res_file);
        }
    }
}
