extern crate xml;
use xml::reader::{EventReader, XmlEvent};

use super::note;
use std::rc::Rc;
use std::fs::File;
use std::io::BufReader;

pub struct Part {
    notes: Vec<Rc<note::Note>>
}

impl Part {
    fn load_from_path()
}

#[cfg(test)]
mod tests{
    use super::*;

    fn test1() {

    }
}