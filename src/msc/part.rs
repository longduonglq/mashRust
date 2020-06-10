use std::rc::Rc;
use std::borrow::Cow;
use crate::msc::measure::{Measure};
use crate::msc::gnote::{Gnote, note_attr};
use crate::msc::gnote::note_attr::Offset;
use std::cell::RefCell;
use crate::parser::xml_tag::XmlTag;
use crate::msc::attributes::{TimeSignature, Clef};

#[derive(Debug)]
pub struct Part {
    // Tags that we don't care about
    extra_tags: Vec< Rc< XmlTag>>,

    pub id: String,

    pub key_sig: i8,
    pub time_sig: TimeSignature,
    pub clef: Clef,
    pub divisions: u8,

    pub duration: note_attr::Duration,
    pub measures: Vec< Measure>,
}

impl Part {
    fn from_xml(xml_tag: Rc< XmlTag>) {

    }
}

mod tests {
    use super::*;
    use std::borrow::Borrow;
    use crate::msc::gnote::Gnote;
    use crate::msc::gnote::note_attr::*;

    #[test]
    fn test_1 () {
    }
}
