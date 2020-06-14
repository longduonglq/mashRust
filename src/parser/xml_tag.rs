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
use std::borrow::{Cow, BorrowMut, Borrow};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use std::ops::Deref;
use std::hint::unreachable_unchecked;
use std::collections::linked_list::Iter;

#[derive(Debug, Clone)]
pub struct XmlTag {
    pub name: OwnedName, // tag name
    pub attributes: Vec<OwnedAttribute>,
    pub text: Option<String>, // content between tag pair

    pub child_tags: LinkedList< XmlTag>
}

// Core methods implementation
impl XmlTag {
    // Init methods
    pub fn from_file<T: AsRef<Path>> (path: T) -> Self
    {
        let xml_file = File::open(path);
        XmlTag::from_buffer(xml_file.unwrap())
    }

    pub fn from_buffer(buffer: impl Read) -> Self
    {
        let event_reader = EventReader::new(buffer);
        Self::from_event_reader(event_reader)
    }

    pub fn from_event_reader(reader: EventReader<impl Read>) -> Self
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
                            child_tags: LinkedList::new()
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
                        parent.child_tags.push_back(begin_tag);
                    } else {
                        return begin_tag
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
    /// Returns a LinkedList of references to XmlTag
    pub fn search_tags(
        &self,
        tag_name: &str
    ) -> LinkedList< &XmlTag>
    {
        let mut results = LinkedList::new();
        if self.name.local_name.as_str() == tag_name {
            results.push_back(self);
        }
        for child_tag in self.child_tags.iter() {
            let mut search_res = Self::search_tags(&child_tag, tag_name);
            if !search_res.is_empty() {
                results.append(&mut search_res);
            }
        }
        return results;
    }

    /// This method is used when there are multiple <tag> present.
    /// It will return the nth tag
    pub fn get_nth_tag(
        &self,
        path: &str,
        n: u8
    ) -> &XmlTag
    {
        self.search_path(path).split_off((n - 1) as usize).pop_front().unwrap()
    }

    /// Return the count of a certain tag. Fn takes path to tag
    pub fn count_tag(
        &self,
        path: &str,
    ) -> u8
    {
        self.search_path(path).len() as u8
    }

    /// A more general method than search_tag. It accepts path rather than tag_name
    /// Path must have format  : tag_name1/tag_name2/tag_name3
    pub fn search_path(
        &self,
        path: &str
    ) -> LinkedList< &XmlTag>
    {
        let tags: Vec<&str> = path.split('/').collect();
        let mut ret = LinkedList::new();
        // returns self in the case path is empty
        if tags.is_empty() {
            ret.push_back(self);
            return ret;
        }

        fn _recursive_search<'a>(xml_tag: &'a XmlTag,
                             tag_names: &Vec< &str>,
                             name_index: usize,
                             res: &mut LinkedList< &'a XmlTag>)
        {
            if name_index == tag_names.len() {
                // Reached the end of the recursive tree, start dumping result into LinkedList res
                res.push_back(xml_tag);
                return;
            }
            for found_sub_tag in xml_tag.search_tags(tag_names[name_index])
            {
                _recursive_search(found_sub_tag,
                                  tag_names,
                                  name_index + 1,
                                  res);
            }
        }
        _recursive_search(self, &tags, 0, &mut ret);
        ret
    }

    /// Check if path exists
    pub fn path_exists(
        self: &XmlTag,
        path: &str
    ) -> bool
    {
        let res = self.search_path(path);
        !res.is_empty()
    }

    /// A special case of search_path for paths that are guaranteed to be fail-free
    /// and unique
    pub fn search_path_unique(
        &self,
        path: &str
    ) -> Option<&XmlTag>
    {
        let mut res = self.search_path(path);
        if res.is_empty() {
            return None;
        }
        res.pop_front()
    }

    /// Get the text between tag pair
    pub fn get_tag_content(
        self: &XmlTag,
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
        self: &XmlTag,
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

    /// Get attribute as string
    pub fn get_attribute_value (
        &self,
        attr_name: &str
    ) -> Option<String>
    {
        self.attributes
            .iter()
            .find(|attr| attr.name.local_name == attr_name)
            .and_then(|attr| Some(attr.value.clone()))
    }

    /// Get attribute with name attr_name then cast
    pub fn get_attribute_as <T> (
        self: &XmlTag,
        attr_name: &str
    ) -> Option<T>
    where T: FromStr
    {
        self.attributes
            .iter()
            .find(|attr| attr.name.local_name == attr_name)
            .and_then(|attr| attr.value.parse().ok())

    }

    /// Push child tags with name in tag_names in self to extra_tags: mut Vec<...>
    /// This method is used to push tags that we dont care about into a storage
    /// that can be taken out when we need to write object to file
    pub fn push_extra_tags_to<'a>(
        self: &'a XmlTag,
        tag_names: &[&str],
        sink: &mut Vec< &'a XmlTag>,
    ) {
        for tag_name in tag_names {
            self.search_tags(tag_name)
                .iter()
                .for_each(|tag| sink.push(tag))
        }
    }

    // Builder methods
    pub fn new_tag_builder() -> Self {
        Self {
            name: OwnedName {
                local_name: "tag_builder_begin".to_string(),
                namespace: None,
                prefix: None
            },
            attributes: vec![],
            text: None,
            child_tags: LinkedList::new()
        }
    }

    pub fn built_tag(&mut self) -> Self {
        self.child_tags.pop_back().unwrap()
    }

    /// Add a tag as child of self
    pub fn add_tag(&mut self, name: &str) -> &mut Self {
        if self.child_tags
            .iter_mut()
            .find(|child_tag| child_tag.name.local_name == name)
            .is_some()
        {
            self.child_tags.back_mut().unwrap()
        } else {
            // If tag not already exists in child_tags, create tag
            let new_tag = XmlTag {
                name: OwnedName {
                    local_name: name.to_string(),
                    namespace: None,
                    prefix: None
                },
                attributes: Vec::with_capacity(5),
                text: None,
                child_tags: LinkedList::new()
            };
            self.child_tags.push_back(new_tag);
            self.child_tags.back_mut().unwrap()
        }
    }

    /// Directly add XmlTag other as a child of self
    pub fn direct_add_tag(&mut self, other: XmlTag) -> &mut Self {
        self.child_tags.push_back(other);
        self
    }

    /// Add attribute to previous tag
    pub fn add_attribute(&mut self,
                         key: impl AsRef<str>,
                         value: impl AsRef<str>)
        -> &mut Self
    {
        if self.attributes
            .iter()
            .find(|attr| attr.name.local_name == key.as_ref())
            .is_none()
        {
            self.attributes.push(OwnedAttribute {
                name: key.as_ref().parse().unwrap(),
                value: value.as_ref().parse().unwrap()
            });
            self
        } else {
            // panic if attribute already exists
            unreachable!()
        }
    }

    /// Add multiple attributes. Panic if attribute already exists
    pub fn add_attributes(&mut self, attrs: &Vec<OwnedAttribute>) -> &mut Self {
        attrs.iter()
            .for_each(
                |attr: &OwnedAttribute| {
                    self.add_attribute(
                        attr.name.local_name.as_str(), attr.value.as_str()
                    );
                }
            );
        self
    }

    /// Add text to tag pair. This method usually at the end of a chain.
    /// Can be used to fix value of a text
    pub fn add_text(&mut self, text: impl AsRef<str>) {
        self.text = Some(text.as_ref().to_string())
    }

    /// Recursively travels down self and compare self.name.local_name with
    /// other.name.local_name. If matched, merge their children together
    /// Function returns a bool indicating success or failure
    pub fn merge_tag(
        &mut self,
        other: &mut XmlTag,
    ) -> bool
    {
        if self.name.local_name == other.name.local_name {
            self.child_tags.append(&mut other.child_tags);
            return true;
        }
        for child_tag in self.child_tags.iter_mut() {
            if child_tag.merge_tag(other) {
                return true;
            }
        }
        false
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
    pub fn print_debug(self: &Self, depth: usize) {
        let indent = |size: usize| -> String {
            const INDENT: &'static str = "    ";
            (0..size).map(|_| INDENT)
                .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
        };
        // print tag with indent
        print!("{}+{}",
                 indent(depth), self.name,
        );
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
    fn test_from() {
        let xml_tree = XmlTag::from_buffer(xml_info());
        xml_tree.print_debug(0);
    }

    #[test]
    fn test_search() {
        let xml_tree = XmlTag::from_buffer(xml_info());
        xml_tree.print_debug(0);

        xml_tree.search_path("print/staff-layout/staff-distance")
            .front().unwrap()
            .print_debug(0);

        xml_tree.search_path("system-layout").front().unwrap().print_debug(0);

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
        let res = xml_tree.search_path("supports");
        res.iter().for_each(|tag| tag.print_debug(0));
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
        let partial = XmlTag::search_tags(&xml_tree, "measure");
        partial.front().unwrap().write_to_buffer(&mut file);
    }

    #[test]
    fn test_mutability() {
        let xml_tree = &mut XmlTag::from_buffer(xml_info());
        xml_tree.print_debug(0);
        xml_tree.child_tags.pop_back();
        xml_tree.print_debug(0);
    }

    #[test]
    fn test_builder () {
        let mut builder = XmlTag::new_tag_builder();
        builder.add_tag("attributes")
            .add_attribute("pos", "24.4")
            .add_tag("print")
            .add_text("G2");

        builder.add_tag("attributes")
            .add_attribute("fkey", "sdr");

        builder.add_tag("attributes")
            .add_tag("layout");
        builder.built_tag().print_debug(0);
    }
}