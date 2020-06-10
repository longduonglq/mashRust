use std::rc::Rc;
use crate::msc::gnote::Gnote;
use crate::parser::xml_tag::XmlTag;
use crate::msc::attributes::attributes;


#[derive(Debug)]
pub struct Measure {
    // Contains extra tags we dont care about
    extra_tags: Vec< Rc< XmlTag>>,

    pub number: u16,
    pub notes: Vec< Rc< Gnote>>
}

impl Measure {
    pub fn from_xml_tag(xml_tag: &Rc< XmlTag>, attrs: &attributes)
        -> Vec< Self>
    {
        assert_eq!(xml_tag.name.local_name, "measure");
        let measure = Measure {
            extra_tags: Vec::with_capacity(10),
            number: xml_tag.attributes
                .iter()
                .find(|attr| attr.name.local_name == "number")
                .unwrap()
                .name.local_name
                .parse().unwrap(),
            notes: Vec::with_capacity(16)
        };

        Vec::new()
    }
}

mod tests {
    use super::*;
    use crate::parser::xml_io::XmlIO;

    #[test]
    fn test_01 () {
        let xml_tag = XmlTag::from_buffer(measure_xml());
        let attr_tag = xml_tag.search_path_unique("attributes");
        let attr_tag = attributes::from_xml_tag(&attr_tag);
        let m = Measure::from_xml_tag(&xml_tag, &attr_tag);
        println!("{:?}", m);
    }

    fn measure_xml () -> &'static [u8]{
        return r#"<measure number="1" width="247.20">
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
      <direction placement="above">
        <direction-type>
          <words relative-y="20.00">mel2</words>
          </direction-type>
        <staff>1</staff>
        </direction>
      <note default-x="86.27" default-y="-50.00" dynamics="112.22">
        <pitch>
          <step>C</step>
          <octave>4</octave>
          </pitch>
        <duration>6</duration>
        <voice>1</voice>
        <type>quarter</type>
        <stem>up</stem>
        <staff>1</staff>
        </note>
      <note default-x="127.71" default-y="-5.00" dynamics="112.22">
        <pitch>
          <step>E</step>
          <octave>5</octave>
          </pitch>
        <duration>9</duration>
        <voice>1</voice>
        <type>quarter</type>
        <dot/>
        <stem>down</stem>
        <staff>1</staff>
        </note>
      <note default-x="178.25" default-y="-10.00" dynamics="112.22">
        <pitch>
          <step>D</step>
          <octave>5</octave>
          </pitch>
        <duration>3</duration>
        <voice>1</voice>
        <type>eighth</type>
        <stem>down</stem>
        <staff>1</staff>
        </note>
      <note default-x="204.15" default-y="0.00" dynamics="112.22">
        <pitch>
          <step>F</step>
          <octave>5</octave>
          </pitch>
        <duration>6</duration>
        <tie type="start"/>
        <voice>1</voice>
        <type>quarter</type>
        <stem>down</stem>
        <staff>1</staff>
        <notations>
          <tied type="start"/>
          </notations>
        </note>
      <backup>
        <duration>24</duration>
        </backup>
      <note default-x="82.95" default-y="-130.00">
        <pitch>
          <step>C</step>
          <octave>3</octave>
          </pitch>
        <duration>24</duration>
        <voice>5</voice>
        <type>whole</type>
        <staff>2</staff>
        </note>
      <note default-x="82.95" default-y="-120.00">
        <chord/>
        <pitch>
          <step>E</step>
          <octave>3</octave>
          </pitch>
        <duration>24</duration>
        <voice>5</voice>
        <type>whole</type>
        <staff>2</staff>
        </note>
      <note default-x="82.95" default-y="-110.00">
        <chord/>
        <pitch>
          <step>G</step>
          <octave>3</octave>
          </pitch>
        <duration>24</duration>
        <voice>5</voice>
        <type>whole</type>
        <staff>2</staff>
        </note>
      </measure>"#.as_bytes();
    }
}