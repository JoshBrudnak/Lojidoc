#[derive(Clone)]
/// Struct for a java package. stores the name and member files
pub struct Package {
    name: String,
    members: Vec<String>,
}

impl Package {
    pub fn add_class(&mut self, class_name: String) {
        self.members.push(class_name);
    }
    pub fn clone(&mut self) -> Package {
        let mut new_members: Vec<String> = Vec::new();

        for m in self.members.clone() {
            new_members.push(m);
        }

        Package {
            name: self.name.clone(),
            members: new_members,
        }
    }
}

/// Struct representing all the application data
pub struct ApplicationDoc {
    pub file_num: i32,
    pub class_num: i32,
    pub interface_num: i32,
    pub enum_num: i32,
    pub packages: Vec<Package>,
}

impl ApplicationDoc {
    pub fn new() -> ApplicationDoc {
        ApplicationDoc {
            file_num: 0,
            enum_num: 0,
            class_num: 0,
            interface_num: 0,
            packages: Vec::new(),
        }
    }
    pub fn add_package_class(&mut self, package: String, class: String) {
        let mut found = false;

        for (i, p) in self.packages.clone().iter().enumerate() {
            if package == p.name {
                self.packages[i].add_class(class.clone());
                found = true;
            }
        }

        if !found {
            self.packages.push(Package {
                name: package,
                members: vec![class],
            });
        }
    }
}
