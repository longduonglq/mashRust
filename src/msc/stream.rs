use crate::msc::duration::Duration;
use super::part::Part;
use crate::msc::parser::xml_tag::XmlTag;

pub struct Stream {
    duration: Duration,
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
        //XmlTag::print_debug(&xml_tree, 0);
        let f = Stream::from_xml_tag(&xml_tree);
    }
}