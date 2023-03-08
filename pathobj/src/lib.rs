use std::path::PathBuf;
use std::env;

pub struct PathObj {
    pub path: PathBuf,
}

impl PathObj {
    pub fn new() -> Self {
        Self {
            path: PathBuf::new(),
        }
    }

    pub fn push(&mut self, value: &str){
        self.path.push(value);
    }

    pub fn join(&mut self, values: Vec<&str>) {
        for value in values {
            self.path.push(value);
        }
    }

    pub fn pop(&mut self) {
        self.path.pop();
    }

    pub fn parent(&mut self) -> String {
        let path = self.path.parent().unwrap();
        format!("{}", path.display()) 
    }

    pub fn file_name(&mut self) -> String {
        let path = self.path.file_name().unwrap();
        format!("{}", path.to_string_lossy())
    }

    pub fn extension(&mut self) -> String {
        let path = self.path.extension().unwrap();
        format!("{}", path.to_string_lossy())
    }
    
    pub fn getcwd() -> String {
        let current = env::current_dir().unwrap();
        format!("{}", current.display()) 
    }
    
}

/*

use pathobj::PathObject;

let mut path = PathObject::new();
path.push(&t);
path.push("aaa");

path.join(vec!["abc", "def", "ghi.png"]);

println!("{}", path.parent());
println!("{}", path.file_name());
println!("{}", path.extension());
println!("{}", PathObject::getcwd());

 */
