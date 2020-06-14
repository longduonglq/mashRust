use super::part::Part;
use crate::parser::xml_tag::XmlTag;
use crate::msc::gnote::note_attr;
use std::rc::Rc;
use crate::msc::measure::Measure;

#[derive(Debug)]
pub struct Stream<'a> {
    // Contains tags that we don't really care about
    _xml_tags: Vec< &'a XmlTag>,

    duration: note_attr::Duration,
    parts: Vec< Part<'a>>
}

impl<'a> Stream<'a> {
    fn from_xml_tag(xml_tag: &'a XmlTag) -> Self {
        assert_eq!(xml_tag.name.local_name, "score-partwise");
        let mut stream = Stream {
            _xml_tags: Vec::with_capacity(5),
            duration: note_attr::Duration::from(0u16),
            parts: Vec::with_capacity(4),
        };
        // Store extra_tags
        XmlTag::push_extra_tags_to(&xml_tag,
                                   &["identification", "defaults"],
                                   &mut stream._xml_tags);
        //Self::parse_part(&xml_tag);

        stream
    }

    // fn parse_part(xml_tag: &Rc< XmlTag>) -> Vec< Part> {
    //     // Extract the <attribute> from the first measure and pass as arg to initialize
    //     // every following measure
    //     let attr_tag = XmlTag::search_tag(xml_tag, "attributes")
    //         .pop_front().unwrap();
    //
    //     // create parts Vec with capacity as indicated by <staves>
    //     let mut parts: Vec< Part> = Vec::with_capacity(
    //         attrs.entry("staves".to_string())
    //             .or_insert("1".to_string())
    //             .parse::<usize>().unwrap()
    //     );
    //
    //     for measure_tag in XmlTag::search_tag(xml_tag, "measure").into_iter() {
    //         let measures = Measure::from_xml_tag(&measure_tag, &attr_tag);
    //         parts.iter_mut()
    //             .zip(measures.into_iter())
    //             .for_each(
    //                 |(part, measure)|
    //                     part.measures.push(measure)
    //             );
    //     }
    //     parts
    // }
}

mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test() {
        let xml_tree = XmlTag::from_file("src/parser/test/example6.musicxml");
        //XmlTag::print_debug_tag(&xml_tree, 0);
        let f = Stream::from_xml_tag(&xml_tree);
        //println!("{:#?}", f);
    }
}