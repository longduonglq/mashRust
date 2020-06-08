use xml::{reader, EventReader, EventWriter, EmitterConfig};
use std::rc::{Weak, Rc};
use std::io::{Read, Write};
use xml::reader::XmlEvent;
use xml::writer::events::XmlEvent::StartDocument;
use xml::name::{OwnedName};
use xml::attribute::OwnedAttribute;
use xml::namespace::Namespace;
use std::cell::{RefCell, Ref};
use std::mem::size_of_val;
use std::path::Path;
use std::fs::File;
use std::collections::{HashMap, LinkedList};
use std::borrow::{Cow, BorrowMut};

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
    // Init methods
    pub fn from_file<T: AsRef<Path>> (path: T) -> Rc< Self>
    {
        let xml_file = File::open(path);
        XmlTag::from_buffer(xml_file.unwrap())
    }

    pub fn from_buffer(buffer: impl Read) -> Rc< Self>
    {
        let event_reader = EventReader::new(buffer);
        Self::from_event_reader(event_reader)
    }

    pub fn from_event_reader(reader: EventReader<impl Read>) -> Rc< Self>
    {
        // Uses stack instead of recursion. When encounter a beginTag, push new XmlTag
        // onto the stack. If an endTag event encountered, pop the beginTag from stack
        // then wrap it around an Rc::new() add it to the .child_tags field of
        // the parent tag.
        let mut stack: Vec< XmlTag> = Vec::with_capacity(15);
        for xml_event in reader {
            match xml_event {
                Ok(XmlEvent::StartElement {name, attributes, namespace})
                => {
                    stack.push(XmlTag {
                            name, attributes, namespace,
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
                        parent.child_tags.push(Rc::new(begin_tag) );
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

    // Probing methods
    /// Recursively search for a tag with name tag_name
    /// Returns a LinkedList of Rc to XmlTags found.
    pub fn search_tag<T: AsRef<str>>(tag_name: T, xml_tag: &Rc<XmlTag>)
        -> LinkedList< Rc< XmlTag>>
    {
        let mut results = LinkedList::new();
        if xml_tag.name.local_name.as_str() == tag_name.as_ref() {
            results.push_back(Rc::clone(xml_tag));
        }
        for child_tag in xml_tag.child_tags.iter() {
            let mut search_res = Self::search_tag(tag_name.as_ref(), &child_tag);
            if !search_res.is_empty() {
                results.append(&mut search_res);
            }
        }
        return results;
    }

    /// Create a HashMap from tag_name to tag_content.
    pub fn extract_tags<T: AsRef<str>>(tag_names: Vec<T>, xml_tag: &Rc<XmlTag>)
        -> (HashMap< String, String>, )
    {
        unimplemented!()
    }

    pub fn write_to_event_writer<W>(xml_tag: &XmlTag, writer: &mut EventWriter<W>)
    where W: Write
    {
        let mut start_event =
            xml::writer::events::XmlEvent::start_element(xml_tag.name.local_name.as_ref());
        for attr in xml_tag.attributes.iter() {
            start_event = start_event.attr(attr.name.local_name.as_ref(), attr.value.as_ref());
        }
        writer.write(start_event);
        // if text exists, write characters event
        if xml_tag.text.is_some() {
            let text_event = xml::writer::events::XmlEvent::characters(xml_tag.text.as_ref().unwrap());
            writer.write(text_event);
        }
        // recursively writes children
        for child_tag in xml_tag.child_tags.iter() {
            Self::write_to_event_writer(child_tag, writer);
        }
        let end_event = xml::writer::events::XmlEvent::end_element();
        writer.write(end_event);
    }

    pub fn write_to_buffer<B: Write>(&self, mut buffer: &mut B) {
        let mut event_writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(&mut buffer);

        Self::write_to_event_writer(&self, &mut event_writer);
    }

    // Debug methods
    pub fn print_debug_tag(xml_tag: &Rc<Self>, depth: usize) {
        let indent = |size: usize| -> String {
            const INDENT: &'static str = "    ";
            (0..size).map(|_| INDENT)
                .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
        };
        // print tag with indent
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
            println!("{}<{}>", indent(depth + 1), xml_tag.text.as_ref().unwrap())
        }
        // print all child tags
        for child_tag in &xml_tag.child_tags {
            Self::print_debug_tag(child_tag, depth + 1);
        }
        println!("{}-{}", indent(depth), xml_tag.name);
    }

    pub fn print_debug(&self) {
        println!("+{}", self.name);
        // print attributes if present
        for attr in &self.attributes {
            print!("  {}={}", attr.name, attr.value)
        }
        // print tag's text content
        if self.text.is_some() {
            println!("    <{}>", self.text.as_ref().unwrap())
        }
        // print all child tags
        for child_tag in &self.child_tags {
            Self::print_debug_tag(child_tag, 1);
        }
        println!("-{}", self.name);
    }

    pub fn print_debug_tags<'a, I>(tags: I)
    where
        I: Iterator<Item = &'a Rc<XmlTag>>
    {
        for tag in tags {
            Self::print_debug_tag(tag, 0);
        }
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
    use std::hint::unreachable_unchecked;
    use std::io;

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
        XmlTag::print_debug_tag(&xml_tree, 0);
        println!("{}", XmlTag::repr_size(&xml_tree));
        xml_tree.print_debug();
    }

    #[test]
    fn test_search() {
        let xml_tree = XmlTag::from_buffer(xml_info());
        XmlTag::print_debug_tag(&xml_tree, 0);
        println!("{}", XmlTag::repr_size(&xml_tree));
        let res = XmlTag::search_tag(
            "supports", &xml_tree);
        for x in res {
            XmlTag::print_debug_tag(&x, 0);
        }
    }

    #[test]
    fn test_search2 () {
        let xml_tree = XmlTag::from_file("src/parser/test/example6.musicxml");
        //XmlTag::print_debug(&xml_tree);
        let res = XmlTag::search_tag("supports", &xml_tree);
        if !res.is_empty() {
            XmlTag::print_debug_tags(res.iter());
        }
    }

    #[test]
    fn test_pointer() {
        let xml_tree = XmlTag::from_buffer(xml_info());
        XmlTag::print_debug_tag(&xml_tree, 0);
        let child = Rc::clone(&xml_tree.child_tags[0]);
        std::mem::drop(xml_tree);
        XmlTag::print_debug_tag(&child, 0);
    }

    #[test]
    fn test_file() {
        let xml_file = File::open("src/parser/test/example6.musicxml");
        let xml_tree = XmlTag::from_buffer(xml_file.unwrap());
        XmlTag::print_debug_tag(&xml_tree, 0);
    }

    #[test]
    fn test_event_writer() {
        let xml_tree = XmlTag::from_file("src/parser/test/example6.musicxml");
        let mut file = File::create("src/parser/test/test_xml_write.musicxml").unwrap();
        let partial = XmlTag::search_tag("measure", &xml_tree);
        partial.front().unwrap().write_to_buffer(&mut file);
    }
}