use super::part::Part;
use crate::parser::xml_tag::XmlTag;
use crate::msc::general_note::note_attr;

pub struct Stream {
    duration: note_attr::Duration,
    parts: Vec< Part>
}

impl Stream {
    fn from_xml_tag(xml_tag: &XmlTag) {
        println!("{:?}", xml_tag.name.local_name);
        match xml_tag.name.local_name.as_str() {
            "part" => {
                println!("{:?}", xml_tag.attributes)
            }
            _ => {}
        }
    }
}

mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test() {
        let xml_file = File::open("src/msc/parser/test/example6.musicxml");
        let xml_tree = XmlTag::from_buffer(xml_file.unwrap());
        XmlTag::print_debug_tag(&xml_tree, 0);
        let f = Stream::from_xml_tag(&xml_tree);
    }
}