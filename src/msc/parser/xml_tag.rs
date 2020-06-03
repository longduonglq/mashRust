use xml::{reader, EventReader};
use std::rc::{Weak, Rc};
use std::io::Read;
use xml::reader::XmlEvent;
use xml::writer::events::XmlEvent::StartDocument;
use xml::name::OwnedName;
use xml::attribute::OwnedAttribute;
use xml::namespace::Namespace;
use std::cell::{RefCell, Ref};
use std::mem::size_of_val;

#[derive(Debug)]
pub struct XmlTag {
    pub name: OwnedName, // tag name
    pub attributes: Vec<OwnedAttribute>,
    pub namespace: Namespace,
    pub text: Option<String>, // content between tag pair

    // Uses Rc here bc an XmlTag might be referred to by user
    pub child_tags: Vec< Rc< XmlTag>>
}

impl XmlTag {
    pub fn from_buffer(buffer: impl Read) -> Rc< Self>
    {
        let event_reader = EventReader::new(buffer);
        Self::from_event_reader(event_reader)
    }

    pub fn from_event_reader(reader: EventReader<impl Read>) -> Rc< Self>
    {
        let mut stack: Vec< XmlTag> = Vec::with_capacity(15);
        for xml_event in reader {
            match xml_event {
                Ok(XmlEvent::StartElement {name, attributes, namespace})
                => {
                    stack.push(
                        XmlTag {
                            name,
                            attributes,
                            namespace,
                            text: None,
                            child_tags: Vec::with_capacity(10)
                        }
                    );
                }

                Ok(XmlEvent::EndElement {name, ..})
                => {
                    // pop the begin_tag that pairs with the current EndElement event
                    let begin_tag = stack.pop().unwrap();
                    assert_eq!(begin_tag.name, name);
                    // if stack is not empty then add child XmlTag, which is begin_tag
                    // if stack is empty, parse is complete
                    if let Some(parent) = stack.last_mut() {
                        parent.child_tags.push(
                            Rc::new(begin_tag)
                        );
                    } else {
                        return Rc::new(begin_tag)
                    }
                }

                Ok(XmlEvent::Characters (d))
                => {
                    let last = stack.last_mut().unwrap();
                    last.text = Some(d);
                }

                _ => {}
            }
        }
        unreachable!()
    }

    pub fn search_tag(tag_name: &String, xml_tag: Rc<XmlTag>) -> Result<Rc<XmlTag>, ()> {
        if xml_tag.name.local_name.as_str() == tag_name {
            Result::Ok(Rc::clone(&xml_tag))
        } else {
            for child_tag in xml_tag.child_tags.iter() {
                let search_res = Self::search_tag(&tag_name, Rc::clone(child_tag));
                if search_res.is_ok() {
                    return search_res;
                }
            }
            Result::Err(())
        }
    }

    pub fn print_debug(xml_tag: Rc<Self>, depth: usize) {
        let indent = |size: usize| -> String {
            const INDENT: &'static str = "    ";
            (0..size).map(|_| INDENT)
                .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
        };
        print!("{}+{}",
                 indent(depth), xml_tag.name,
        );
        // print pointer count
        print!("  PtrCnt({},{})", Rc::strong_count(&xml_tag), Rc::weak_count(&xml_tag));
        // print attributes if present
        for attr in &xml_tag.attributes {
            print!("  {}={}", attr.name, attr.value)
        }
        println!();
        // print tag's text content
        if xml_tag.text.is_some() {
            println!("{}{}", indent(depth + 1), xml_tag.text.as_ref().unwrap())
        }
        // print all child tags
        for child_tag in &xml_tag.child_tags {
            Self::print_debug(Rc::clone(child_tag), depth + 1);
        }
        println!("{}-{}", indent(depth), xml_tag.name);
    }

    pub fn repr_size(xml_tag: &Rc<Self>) -> usize {
        let mut mem_size = 0;
        mem_size += size_of_val(xml_tag);
        for child_tag in &xml_tag.child_tags {
            mem_size += Self::repr_size(child_tag);
        }
        mem_size
    }
}

mod tests {
    use super::*;
    use xml::EventReader;
    use std::mem::{size_of, size_of_val};
    use std::fs::File;
    use std::borrow::{Borrow, BorrowMut};

    fn indent (size: usize) -> String {
        const INDENT: &'static str = "    ";
        (0..size)
            .map(|_| INDENT)
            .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
    }

    fn xml_info() -> &'static [u8] {
        r#"
                <print>
                <system-layout>
                  <system-margins>
                    <left-margin>21.00</left-margin>
                    <right-margin>0.00</right-margin>
                    </system-margins>
                  <top-system-distance>170.00</top-system-distance>
                  </system-layout>
                <staff-layout number="2">
                  <staff-distance>65.00</staff-distance>
                  </staff-layout>
                </print>
              <attributes>
                <divisions>6</divisions>
                <key>
                  <fifths>0</fifths>
                  </key>
                <time>
                  <beats>4</beats>
                  <beat-type>4</beat-type>
                  </time>
                <staves>2</staves>
                <clef number="1">
                  <sign>G</sign>
                  <line>2</line>
                  </clef>
                <clef number="2">
                  <sign>F</sign>
                  <line>4</line>
                  </clef>
                </attributes>
        "#.as_bytes()
    }

    #[test]
    fn test_print(){
        let xml_raw = xml_info();
        let parser = EventReader::new(xml_raw);
        let mut depth = 0;
        for e in parser {
            match e {
                Ok(reader::XmlEvent::StartElement {name, ..}) => {
                    //println!("{}+{}", indent(depth), name);
                    depth += 1;
                }
                Ok(reader::XmlEvent::EndElement {name}) => {
                    depth -= 1;
                    //println!("{}-{}", indent(depth), name);
                }
                Err(e) => {
                    println!("ERROR!!: {}", e);
                    break;
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_from() {
        let xml_tree = XmlTag::from_buffer(xml_info());
        XmlTag::print_debug(Rc::clone(&xml_tree), 0);
        println!("{}", XmlTag::repr_size(&xml_tree));
    }

    #[test]
    fn test_search() {
        let xml_tree = XmlTag::from_buffer(xml_info());
        XmlTag::print_debug(Rc::clone(&xml_tree), 0);
        println!("{}", XmlTag::repr_size(&xml_tree));
        let res = XmlTag::search_tag(
            &"staff-layout".to_string(), Rc::clone(&xml_tree));

        XmlTag::print_debug(res.unwrap(), 0);
    }

    #[test]
    fn test_pointer() {
        let xml_tree = XmlTag::from_buffer(xml_info());
        XmlTag::print_debug(Rc::clone(&xml_tree), 0);
        let child = Rc::clone(&xml_tree.child_tags[1]);
        XmlTag::print_debug(Rc::clone(&xml_tree), 0);
    }

    #[test]
    fn test_file() {
        let xml_file = File::open("src/msc/parser/test/example6.musicxml");
        let xml_tree = XmlTag::from_buffer(xml_file.unwrap());
        XmlTag::print_debug(Rc::clone(&xml_tree), 0);
    }
}