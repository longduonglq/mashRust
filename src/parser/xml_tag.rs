use xml::{reader, EventReader, EventWriter, EmitterConfig};
use std::rc::{Weak, Rc};
use std::io::{Read, Write};
use xml::reader::XmlEvent;
use xml::writer::events::XmlEvent::StartDocument;
use xml::name::{OwnedName};
use xml::attribute::{OwnedAttribute, Attribute};
use std::cell::{RefCell, Ref};
use std::mem::size_of_val;
use std::path::Path;
use std::fs::File;
use std::collections::{HashMap, LinkedList, BTreeMap};
use std::borrow::{Cow, BorrowMut};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use std::ops::Deref;

#[derive(Debug)]
pub struct XmlTag {
    pub name: OwnedName, // tag name
    pub attributes: Vec<OwnedAttribute>,
    pub text: Option<String>, // content between tag pair

    // Uses Rc here bc an XmlTag might be referred to by user
    pub child_tags: Vec< Rc< RefCell< XmlTag>>>
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
                            name, attributes,
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
                            Rc::new(RefCell::from(begin_tag))
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

    // Probing methods
    /// Recursively search for a tag with name tag_name
    /// Returns a LinkedList of Rc to XmlTags found.
    pub fn search_tag(
        self: &Rc< RefCell< XmlTag>>,
        tag_name: &str
    ) -> LinkedList< Rc< RefCell< XmlTag>>>
    {
        let mut results = LinkedList::new();
        if self.as_ref().name.local_name.as_str() == tag_name {
            results.push_back(Rc::clone(self));
        }
        for child_tag in self.as_ref().child_tags.iter() {
            let mut search_res = Self::search_tag(&child_tag, tag_name);
            if !search_res.is_empty() {
                results.append(&mut search_res);
            }
        }
        return results;
    }

    /// A more general method than search_tag. It accepts path rather than tag_name
    /// Path must have format  : tag_name1/tag_name2/tag_name3
    pub fn search_path(
        self: &Rc< XmlTag>,
        path: &str
    ) -> LinkedList< Rc< XmlTag>>
    {
        let paths: Vec<&str> = path.split('/').collect();
        paths
            .iter()
            .fold(
                self.search_tag(paths.first().unwrap()),
                |acc, x| acc.front().unwrap().search_tag(x)
            )
    }

    /// Check if path exists
    pub fn path_exists(
        self: &Rc< XmlTag>,
        path: &str
    ) -> bool
    {
        let res = self.search_path(path);
        !res.is_empty()
    }

    /// A special case of search_path for paths that are guaranteed to be fail-free
    /// and unique
    pub fn search_path_unique(
        self: &Rc< XmlTag>,
        path: &str
    ) -> Rc< XmlTag>
    {
        let mut res = self.search_path(path);
        assert_eq!(res.len(), 1);
        res.pop_front().unwrap()
    }

    pub fn get_tag_content(
        self: &Rc < XmlTag>,
        tag_path: &str
    ) -> Option<String>
    {
        let mut res = self.search_path(tag_path);
        if res.is_empty() {
            return None;
        }
        Some(res.pop_front().unwrap()
            .text.clone().unwrap())
    }

    /// Search for tag at tag_path. Cast the string content to type T and return
    pub fn get_tag_content_as<T>(
        self: &Rc <XmlTag>,
        tag_path: &str
    ) -> Option<T>
    where T: FromStr,
          <T as std::str::FromStr>::Err: std::fmt::Debug
    {
        let mut res = self.search_path(tag_path);
        if res.is_empty() {
            return None;
        }
        Some(res.pop_front().unwrap()
            .text.clone().unwrap()
            .parse().unwrap())
    }

    /// Push child tags with name in tag_names in self to extra_tags: mut Vec<...>
    /// This method is used to push tags that we dont care about into a storage
    /// that can be taken out when we need to write object to file
    pub fn push_extra_tags_to(
        self: &Rc< XmlTag>,
        tag_names: &[&str],
        sink: &mut Vec< Rc< XmlTag>>,
    ) {
        for tag_name in tag_names {
            self.search_tag(tag_name)
                .into_iter()
                .for_each(|tag| sink.push(tag))
        }
    }

    // Write methods
    /// Recursive method that writes xml_tag to writer
    pub fn write_to_event_writer<W>(
        xml_tag: &XmlTag,
        writer: &mut EventWriter<W>
    )
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

    /// Write self to buffer given
    pub fn write_to_buffer<B: Write>(
        &self,
        mut buffer: &mut B
    ) {
        let mut event_writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(&mut buffer);

        Self::write_to_event_writer(&self, &mut event_writer);
    }

    // Debug methods
    pub fn print_debug(self: &Rc<Self>, depth: usize) {
        let indent = |size: usize| -> String {
            const INDENT: &'static str = "    ";
            (0..size).map(|_| INDENT)
                .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
        };
        // print tag with indent
        print!("{}+{}",
                 indent(depth), self.name,
        );
        // print pointer count
        print!("  PtrCnt({},{})", Rc::strong_count(&self), Rc::weak_count(&self));
        // print attributes if present
        for attr in &self.attributes {
            print!("  {}={}", attr.name, attr.value)
        }
        println!();
        // print tag's text content
        if self.text.is_some() {
            println!("{}<{}>", indent(depth + 1), self.text.as_ref().unwrap())
        }
        // print all child tags
        for child_tag in &self.child_tags {
            child_tag.print_debug(depth + 1)
        }
        println!("{}-{}", indent(depth), self.name);
    }

    pub fn print_debug_tags<'a, I>(tags: I)
    where
        I: Iterator<Item = &'a Rc<XmlTag>>
    {
        for tag in tags {
            tag.print_debug(0);
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
        xml_tree.print_debug(0);
        println!("{}", XmlTag::repr_size(&xml_tree));
        xml_tree.print_debug(0);
    }

    #[test]
    fn test_search() {
        let xml_tree = XmlTag::from_buffer(xml_info());
        xml_tree.print_debug(0);
        println!("{}", XmlTag::repr_size(&xml_tree));
        let res = XmlTag::search_tag(
            &xml_tree, "supports");
        for x in res {
            x.print_debug(0);
        }
        xml_tree.search_path("print/staff-layout/staff-distance")
            .front().unwrap()
            .print_debug(0);

        xml_tree.search_path_unique("print/staff-layout/staff-distance")
            .print_debug(0);

        println!("{:?}",
                 xml_tree.get_tag_content_as::<f32>(
                     "print/staff-layout/staff-distance"
                 )
        );
    }

    #[test]
    fn test_search2 () {
        let xml_tree = XmlTag::from_file("src/parser/test/example6.musicxml");
        //XmlTag::print_debug(&xml_tree);
        let res = xml_tree.search_tag("supports");
        if !res.is_empty() {
            XmlTag::print_debug_tags(res.iter());
        }
    }

    #[test]
    fn test_pointer() {
        let xml_tree = XmlTag::from_buffer(xml_info());
        xml_tree.print_debug(0);
        let child = Rc::clone(&xml_tree.child_tags[0]);
        std::mem::drop(xml_tree);
        child.print_debug(0);
    }

    #[test]
    fn test_file() {
        let xml_file = File::open("src/parser/test/example6.musicxml");
        let xml_tree = XmlTag::from_buffer(xml_file.unwrap());
        xml_tree.print_debug(0);
    }

    #[test]
    fn test_event_writer() {
        let xml_tree = XmlTag::from_file("src/parser/test/example6.musicxml");
        let mut file = File::create("src/parser/test/test_xml_write.musicxml").unwrap();
        let partial = XmlTag::search_tag(&xml_tree, "measure");
        partial.front().unwrap().write_to_buffer(&mut file);
    }

    #[test]
    fn test_extract_tags() {
        let xml_tree = XmlTag::from_buffer(xml_info());
        xml_tree.print_debug(0);
    }

}