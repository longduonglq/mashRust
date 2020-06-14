use std::rc::Rc;
use std::borrow::Cow;
use crate::msc::measure::{Measure};
use crate::msc::gnote::{Gnote, note_attr};
use crate::msc::gnote::note_attr::{Offset, Duration};
use std::cell::RefCell;
use crate::parser::xml_tag::XmlTag;
use crate::msc::attributes::{TimeSignature, Clef, attributes};
use std::collections::{BTreeMap, LinkedList};

#[derive(Debug)]
pub struct Part<'a> {
    // Tags that we don't care about
    _xml_tags: Vec< &'a XmlTag>,

    pub attrs: attributes,
    pub duration: note_attr::Duration,
    pub notes: BTreeMap< Offset, Gnote<'a>>
}

impl<'a> Part<'a> {
    fn from_xml_tag(xml_tag: &'a XmlTag) -> Part<'a> {
        let mut part = Part {
            _xml_tags: Vec::with_capacity(5),
            attrs: attributes {
                divisions: None,
                key: None,
                time: None,
                clef: None,
                staves: None
            },
            duration: Duration::from(0u32),
            notes: BTreeMap::new()
        };
        let attrs = attributes::from_xml_tag(xml_tag
                                                  .search_path("attributes")
                                                  .pop_front().unwrap());

        part
    }

    fn to_xml_tag(&self) -> XmlTag {
        unimplemented!()
    }
}

mod tests {
    use super::*;
    use std::borrow::Borrow;
    use crate::msc::gnote::Gnote;
    use crate::msc::gnote::note_attr::*;

    #[test]
    fn test_1 () {
        let mea_tag = XmlTag::from_buffer(measure_xml());
        //mea_tag.print_debug(0);
        let part = Part::from_xml_tag(&mea_tag);
        println!("{:#?}", part.attrs);
    }

    fn measure_xml () -> &'static [u8] {
        return r#"<part id="P1">
    <measure number="1" width="217.25">
      <print>
        <system-layout>
          <system-margins>
            <left-margin>0.00</left-margin>
            <right-margin>-0.00</right-margin>
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
          <words relative-y="20.00">mel1</words>
          </direction-type>
        <staff>1</staff>
        </direction>
      <note default-x="89.59" default-y="-30.00" dynamics="112.22">
        <pitch>
          <step>G</step>
          <octave>4</octave>
          </pitch>
        <duration>6</duration>
        <voice>1</voice>
        <type>quarter</type>
        <stem>up</stem>
        <staff>1</staff>
        </note>
      <note default-x="122.38" default-y="-10.00" dynamics="112.22">
        <pitch>
          <step>D</step>
          <octave>5</octave>
          </pitch>
        <duration>9</duration>
        <voice>1</voice>
        <type>quarter</type>
        <dot/>
        <stem>down</stem>
        <staff>1</staff>
        </note>
      <note default-x="162.36" default-y="-5.00" dynamics="112.22">
        <pitch>
          <step>E</step>
          <octave>5</octave>
          </pitch>
        <duration>3</duration>
        <voice>1</voice>
        <type>eighth</type>
        <stem>down</stem>
        <staff>1</staff>
        </note>
      <note default-x="182.86" default-y="5.00" dynamics="141.11">
        <pitch>
          <step>G</step>
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
      <note default-x="86.27" default-y="-110.00" dynamics="97.78">
        <pitch>
          <step>G</step>
          <octave>3</octave>
          </pitch>
        <duration>24</duration>
        <voice>5</voice>
        <type>whole</type>
        <staff>2</staff>
        </note>
      <note default-x="86.27" default-y="-100.00" dynamics="97.78">
        <chord/>
        <pitch>
          <step>B</step>
          <octave>3</octave>
          </pitch>
        <duration>24</duration>
        <voice>5</voice>
        <type>whole</type>
        <staff>2</staff>
        </note>
      <note default-x="86.27" default-y="-90.00" dynamics="97.78">
        <chord/>
        <pitch>
          <step>D</step>
          <octave>4</octave>
          </pitch>
        <duration>24</duration>
        <voice>5</voice>
        <type>whole</type>
        <staff>2</staff>
        </note>
      </measure>
    <measure number="2" width="226.42">
      <note default-x="14.16" default-y="5.00" dynamics="141.11">
        <pitch>
          <step>G</step>
          <octave>5</octave>
          </pitch>
        <duration>3</duration>
        <tie type="stop"/>
        <voice>1</voice>
        <type>eighth</type>
        <stem>down</stem>
        <staff>1</staff>
        <beam number="1">begin</beam>
        <notations>
          <tied type="stop"/>
          </notations>
        </note>
      <note default-x="39.79" default-y="-10.00" dynamics="112.22">
        <pitch>
          <step>D</step>
          <octave>5</octave>
          </pitch>
        <duration>3</duration>
        <voice>1</voice>
        <type>eighth</type>
        <stem>down</stem>
        <staff>1</staff>
        <beam number="1">continue</beam>
        </note>
      <note default-x="65.41" default-y="-10.00" dynamics="112.22">
        <pitch>
          <step>D</step>
          <octave>5</octave>
          </pitch>
        <duration>3</duration>
        <voice>1</voice>
        <type>eighth</type>
        <stem>down</stem>
        <staff>1</staff>
        <beam number="1">continue</beam>
        </note>
      <note default-x="91.04" default-y="-5.00" dynamics="112.22">
        <pitch>
          <step>E</step>
          <octave>5</octave>
          </pitch>
        <duration>3</duration>
        <voice>1</voice>
        <type>eighth</type>
        <stem>down</stem>
        <staff>1</staff>
        <beam number="1">end</beam>
        </note>
      <note default-x="116.66" default-y="5.00">
        <pitch>
          <step>G</step>
          <octave>5</octave>
          </pitch>
        <duration>2</duration>
        <voice>1</voice>
        <type>eighth</type>
        <time-modification>
          <actual-notes>3</actual-notes>
          <normal-notes>2</normal-notes>
          </time-modification>
        <stem>down</stem>
        <staff>1</staff>
        <notations>
          <tuplet type="start" bracket="yes"/>
          </notations>
        </note>
      <note>
        <rest/>
        <duration>2</duration>
        <voice>1</voice>
        <type>eighth</type>
        <time-modification>
          <actual-notes>3</actual-notes>
          <normal-notes>2</normal-notes>
          </time-modification>
        <staff>1</staff>
        </note>
      <note default-x="154.60" default-y="-10.00">
        <pitch>
          <step>D</step>
          <octave>5</octave>
          </pitch>
        <duration>2</duration>
        <voice>1</voice>
        <type>eighth</type>
        <time-modification>
          <actual-notes>3</actual-notes>
          <normal-notes>2</normal-notes>
          </time-modification>
        <stem>down</stem>
        <staff>1</staff>
        <notations>
          <tuplet type="stop"/>
          </notations>
        </note>
      <note>
        <rest/>
        <duration>3</duration>
        <voice>1</voice>
        <type>eighth</type>
        <staff>1</staff>
        </note>
      <note default-x="199.19" default-y="-5.00" dynamics="112.22">
        <pitch>
          <step>E</step>
          <octave>5</octave>
          </pitch>
        <duration>3</duration>
        <voice>1</voice>
        <type>eighth</type>
        <stem>down</stem>
        <staff>1</staff>
        </note>
      <backup>
        <duration>24</duration>
        </backup>
      <note default-x="13.80" default-y="-110.00" dynamics="97.78">
        <pitch>
          <step>G</step>
          <octave>3</octave>
          </pitch>
        <duration>12</duration>
        <voice>5</voice>
        <type>half</type>
        <stem>down</stem>
        <staff>2</staff>
        </note>
      <note default-x="13.80" default-y="-100.00" dynamics="97.78">
        <chord/>
        <pitch>
          <step>B</step>
          <octave>3</octave>
          </pitch>
        <duration>12</duration>
        <voice>5</voice>
        <type>half</type>
        <stem>down</stem>
        <staff>2</staff>
        </note>
      <note default-x="13.80" default-y="-90.00" dynamics="97.78">
        <chord/>
        <pitch>
          <step>D</step>
          <octave>4</octave>
          </pitch>
        <duration>12</duration>
        <voice>5</voice>
        <type>half</type>
        <stem>down</stem>
        <staff>2</staff>
        </note>
      <note default-x="116.30" default-y="-120.00" dynamics="97.78">
        <pitch>
          <step>E</step>
          <octave>3</octave>
          </pitch>
        <duration>12</duration>
        <voice>5</voice>
        <type>half</type>
        <stem>down</stem>
        <staff>2</staff>
        </note>
      <note default-x="116.30" default-y="-110.00" dynamics="97.78">
        <chord/>
        <pitch>
          <step>G</step>
          <octave>3</octave>
          </pitch>
        <duration>12</duration>
        <voice>5</voice>
        <type>half</type>
        <stem>down</stem>
        <staff>2</staff>
        </note>
      <note default-x="116.30" default-y="-95.00" dynamics="97.78">
        <chord/>
        <pitch>
          <step>C</step>
          <octave>4</octave>
          </pitch>
        <duration>12</duration>
        <voice>5</voice>
        <type>half</type>
        <stem>down</stem>
        <staff>2</staff>
        </note>
      </measure>
      </part>"#.as_bytes();
    }
}
