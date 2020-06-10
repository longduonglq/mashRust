// use crate::parser::xml_tag::XmlTag;
// use std::rc::Rc;
// use std::collections::BTreeMap;
// use xml::attribute::OwnedAttribute;
// use xml::name::OwnedName;
//
// pub struct XmlBuilder {
//     xml_tag: XmlTag
// }
//
// impl XmlBuilder {
//     pub fn new() -> Self {
//         Self {
//             xml_tag: XmlTag{
//                 name: OwnedName {
//                     local_name: "".to_string(),
//                     namespace: None,
//                     prefix: None
//                 },
//                 attributes: Vec::with_capacity(5),
//                 text: None,
//                 child_tags: Vec::with_capacity(15)
//             }
//         }
//     }
//     pub fn tag(&mut self, name: &str) -> Self {
//         let tag = self.xml_tag.child_tags
//             .iter()
//             .find(|child_tag| child_tag.name.local_name == name);
//
//         if tag.is_some() {
//             Self {xml_tag:  }
//         } else {
//             let new_builder = Self::new();
//             self.xml_tag.child_tags.push(Rc::clone(&new_builder.xml_tag));
//             new_builder
//         }
//     }
//
//     pub fn attribute(&mut self, key: &str, value: &str) -> Self {
//         todo!()
//     }
//
//     pub fn attributes(&mut self, attrs: Vec<OwnedAttribute>) -> Self {
//         todo!()
//     }
//
//     pub fn character(&mut self, text: impl ToString) -> Self {
//         todo!()
//     }
//
//     /// Returns a real xml_tag
//     pub fn get_xml_tag(&self) -> Rc< XmlTag> {
//         self.xml_tag.child_tags
//             .first()
//             .unwrap().clone()
//     }
// }
//
// mod tests {
//     use crate::parser::xml_builder::XmlBuilder;
//
//     #[test]
//     fn test1 () {
//         let mut xml_builder = XmlBuilder::new();
//         xml_builder.tag("attributes");
//         xml_builder.get_xml_tag().print_debug(0);
//     }
// }