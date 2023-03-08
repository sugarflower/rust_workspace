use glob::glob;

pub struct GlobObj {
    pub iter: glob::Paths,
}

impl Iterator for GlobObj {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(path)) => {
                Some(path.to_str().expect("err").to_owned())
            }
            _ => None,
        }
    }
}

pub fn globobj(pattern: &str) -> GlobObj {
    GlobObj {
        iter: glob(pattern).expect("failed"),
    }
}

