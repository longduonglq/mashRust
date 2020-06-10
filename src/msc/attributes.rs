//! This module takes care of parsing the <attributes> tag
use crate::parser::xml_tag::XmlTag;
use std::rc::Rc;
use crate::parser::xml_io::XmlIO;
use std::io::Write;
use xml::EventWriter;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use std::collections::BTreeMap;

pub type TimeSignature = (u8, u8);

#[derive(Debug)]
pub enum ClefSign {
    G, F, C
}
impl ClefSign {
    fn from_str(sign: &str) -> Self {
        match sign {
            "G" => Self::G,
            "F" => Self::F,
            "C" => Self::C,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
pub struct Clef {
    sign: ClefSign,
    line: u8
}

#[derive(Debug)]
pub struct attributes {
    divisions: Option<u8>,
    key: Option<i8>,
    time: Option<TimeSignature>,
    clef: Option<Clef>
}

impl XmlIO<attributes> for attributes{
    fn from_xml_tag(xml_tag: &Rc<XmlTag>) -> attributes {
        attributes {
            divisions: xml_tag.get_tag_content_as::<u8>("divisions"),
            key: xml_tag.get_tag_content_as("key/fifths"),
            time: {
                if xml_tag.path_exists("time") {
                    Some(
                        (
                            xml_tag.get_tag_content_as("time/beats").unwrap(),
                            xml_tag.get_tag_content_as("time/beat-type").unwrap()
                        )
                    )
                } else {
                    None
                }
            },

            clef: {
                if xml_tag.path_exists("clef") {
                    Some(
                        Clef {
                            sign: ClefSign::from_str(
                                xml_tag.get_tag_content("clef/sign")
                                    .unwrap()
                                    .as_ref()
                            ),
                            line: xml_tag.get_tag_content_as("clef/line").unwrap()
                        }
                    )
                } else {
                    None
                }
            }
        }
    }

    fn to_xml_tag(&self) -> XmlTag {
        todo!()
    }
}

mod tests {
    use super::*;

    #[test]
    fn test1 () {
        let xml_tag = XmlTag::from_buffer(attr_xml());
        let attr = attributes::from_xml_tag(&xml_tag);
        println!("{:#?}", attr);
    }

    fn attr_xml () -> &'static [u8] {
        return r#"<attributes>
        <divisions>6</divisions>
        <key>
          <fifths>0</fifths>
          </key>
        <time>
          <beats>4</beats>
          <beat-type>4</beat-type>
          </time>
        <clef>
          <sign>G</sign>
          <line>2</line>
          </clef>
        </attributes>"#.as_bytes();
    }
}