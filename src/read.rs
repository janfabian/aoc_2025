use std::{fs::read_to_string, io::Result};

pub trait Solution {
    fn new(input: &str) -> Self;
    fn execute(&self);
}

pub fn read_file(path: &str) -> Result<String> {
    read_to_string(path)
}
