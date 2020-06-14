//! This module takes care of parsing the <attributes> tag
use crate::parser::xml_tag::XmlTag;
use std::rc::Rc;
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

    fn to_str(&self) -> &str {
        match self {
            Self::G => "G",
            Self::F => "F",
            Self::C => "C",
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
    pub divisions: Option<u8>,
    pub key: Option<i8>,
    pub time: Option<TimeSignature>,
    pub clef: Option<Clef>,
    pub staves: Option<u8>
}

impl attributes {
    pub fn from_xml_tag(xml_tag: &XmlTag) -> attributes {
        assert_eq!(xml_tag.name.local_name, "attributes", "Tag is not <attributes>");
        attributes {
            divisions: xml_tag.get_tag_content_as::<u8>("divisions"),
            key: xml_tag.get_tag_content_as("key/fifths"),
            time: {
                xml_tag.search_path_unique("time")
                    .and(Some(
                            (
                                xml_tag.get_tag_content_as("time/beats").unwrap(),
                                xml_tag.get_tag_content_as("time/beat-type").unwrap()
                            )
                        )
                    )
            },

            clef: {
                xml_tag.search_path_unique("clef")
                    .and(Some(
                        Clef {
                            sign: ClefSign::from_str(
                                xml_tag.get_tag_content("clef/sign")
                                    .unwrap()
                                    .as_ref()
                            ),
                            line: xml_tag.get_tag_content_as("clef/line").unwrap()
                        }
                    ))
            },

            staves: {
                xml_tag.search_path_unique("staves")
                    .and_then(|tag| Some(tag.text.as_ref().unwrap().parse().unwrap()))
            }
        }
    }

    fn to_xml_tag(&self) -> XmlTag {
        let mut builder = XmlTag::new_tag_builder();
        let attr = builder.add_tag("attributes");
        // <divisions>
        if self.divisions.is_some() {
            attr.add_tag("divisions").add_text(self.divisions.unwrap().to_string());
        }
        // <key>
        if self.key.is_some() {
            attr.add_tag("key").add_tag("fifths").add_text("0");
        }
        // <time>
        if self.time.is_some() {
            let time = attr.add_tag("time");
            time.add_tag("beats").add_text(self.time.unwrap().0.to_string());
            time.add_tag("beat-type").add_text(self.time.unwrap().1.to_string());
        }
        // <clef>
        if self.clef.is_some() {
            let clef = attr.add_tag("clef");
            clef.add_tag("sign")
                .add_text(self.clef.as_ref().unwrap()
                    .sign.to_str()
                );

            clef.add_tag("line")
                .add_text(self.clef.as_ref().unwrap()
                    .line.to_string()
                );
        }
        // <staves>
        if self.staves.is_some() {
            attr.add_tag("staves").add_text(self.staves.unwrap().to_string());
        }
        builder.built_tag()
    }
}

mod tests {
    use super::*;

    #[test]
    fn test1 () {
        let xml_tag = XmlTag::from_buffer(attr_xml());
        let attr = attributes::from_xml_tag(&xml_tag);
        println!("{:#?}", attr);
        attr.to_xml_tag().print_debug(0);
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
        <staves>2</staves>
        </attributes>"#.as_bytes();
    }
}