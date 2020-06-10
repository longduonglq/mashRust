use crate::parser::xml_tag::XmlTag;
use std::rc::Rc;
use xml::EventWriter;
use std::io::Write;

/// Score objects must implement this trait as it is required for writing to file.
pub trait XmlIO<T> {
    /// Construct object from xml
    fn from_xml_tag(xml_tag: &Rc< XmlTag>) -> T;
    /// Destruct object back to xml
    fn to_xml_tag(&self) -> XmlTag;
}