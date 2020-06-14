use std::hash::Hash;
use std::collections::{HashSet, LinkedList};
use std::rc::Rc;
use crate::parser::xml_tag::XmlTag;
use std::borrow::BorrowMut;
use std::ops::Deref;
use xml::attribute::OwnedAttribute;
use crate::msc::attributes::attributes;
use crate::libs::frac::Frac;
use crate::msc::gnote::note_attr::{Tie, Lyric, Pitch, Accidental, TimeModification};

pub mod note_attr {
    use std::collections::HashMap;
    use crate::libs::frac::Frac;
    use crate::parser::xml_tag::XmlTag;

    #[derive(Debug, Eq, PartialEq, Hash, Clone)]
    pub enum Accidental {
        sharp,
        dsharp,
        flat,
        dflat,
        natural,
    }
    impl Accidental {
        pub fn from_xml_tag(accidental_tag: &XmlTag) -> Self {
            match accidental_tag.text.as_ref().unwrap().as_str() {
                "sharp" => Accidental::sharp,
                "double-sharp" => Accidental::dsharp,
                "flat" => Accidental::flat,
                "flat-flat" => Accidental::dflat,
                "natural" => Accidental::natural,
                _ => unreachable!()
            }
        }

        pub fn to_xml_tag(&self) -> XmlTag {
            todo!()
        }
    }

    #[derive(Debug, Eq, PartialEq, Hash, Clone)]
    pub enum Step {
        A,
        B,
        C,
        D,
        E,
        F,
        G
    }

    #[derive(Debug, Eq, PartialEq, Hash, Clone)]
    pub struct Pitch {
        pub step: Step,
        pub alter: Option<i8>,
        pub octave: Octave
    }
    impl Pitch {
        pub fn from_xml_tag(pitch_tag: &XmlTag) -> Self {
            Self {
                step: match pitch_tag
                    .search_path_unique("step").unwrap()
                    .text.as_ref().unwrap()
                    .as_str()
                {
                    "A" => Step::A,
                    "B" => Step::B,
                    "C" => Step::C,
                    "D" => Step::D,
                    "E" => Step::E,
                    "F" => Step::F,
                    "G" => Step::G,
                    _ => unreachable!()
                },
                octave: pitch_tag.get_tag_content_as("octave").unwrap(),
                alter: pitch_tag.search_path_unique("alter")
                    .and_then(|x| x.text.as_ref().unwrap().parse::<i8>().ok())
            }
        }

        pub fn to_xml_tag(&self) -> XmlTag {
            let mut builder = XmlTag::new_tag_builder();
            let pitch = builder.add_tag("pitch");
            // step
            pitch.add_tag("step").add_text({
                match self.step {
                    Step::A => "A",
                    Step::B => "B",
                    Step::C => "C",
                    Step::D => "D",
                    Step::E => "E",
                    Step::F => "F",
                    Step::G => "G",
                }
            });
            // octave
            pitch.add_tag("octave").add_text(self.octave.to_string().as_str());
            // alter
            if self.alter.is_some() {
                pitch.add_tag("alter").add_text(self.alter.unwrap().to_string().as_str());
            }
            builder.built_tag()
        }
    }

    #[derive(Debug, Eq, PartialEq, Hash, Clone)]
    pub enum Tie {
        start,
        stop
    }
    impl Tie {
        pub fn from_xml_tag(tie_tag: &XmlTag) -> Self
        {
            match tie_tag.get_attribute_value("type").unwrap().as_str()
            {
                "start" => Self::start,
                "end" => Self::stop,
                _ => unreachable!()
            }
        }

        /// Returns two necessary tags <tie> and <notations>
        pub fn to_xml_tag(&self) -> XmlTag
        {
            let variant_str = match self {
                Self::start => "start",
                Self::stop => "stop",
                _ => unreachable!()
            };
            let mut builder = XmlTag::new_tag_builder();

            let mut note_tag = builder.add_tag("note");
            note_tag.add_tag("tie").add_attribute("type", variant_str);
            note_tag.add_tag("notations").add_tag("tied").add_attribute("type", variant_str);

            builder.built_tag()
        }
    }

    #[derive(Debug, Eq, PartialEq, Hash, Clone)]
    pub struct TimeModification {
        actual_notes: u8,
        normal_notes: u8,
    }
    impl TimeModification {
        pub fn from_xml_tag(xml_tag: &XmlTag) -> Self {
            Self {
                actual_notes: xml_tag.search_path_unique("actual-notes").unwrap()
                    .text.as_ref().unwrap()
                    .parse().unwrap(),
                normal_notes: xml_tag.search_path_unique("normal-notes").unwrap()
                    .text.as_ref().unwrap()
                    .parse().unwrap()
            }
        }

        pub fn to_xml_tag(&self) -> XmlTag {
            let mut builder = XmlTag::new_tag_builder();
            let mut time_mod_tag = builder.add_tag("time-modification");
            time_mod_tag.add_tag("actual-notes").add_text(self.actual_notes.to_string().as_str());
            time_mod_tag.add_tag("normal-notes").add_text(self.normal_notes.to_string().as_str());
            builder.built_tag()
        }
    }

    pub type Duration = Frac;

    pub type LengthType = String;
    lazy_static! {
        pub static ref LENGTH_TYPE_TABL: HashMap<&'static str, u16> = {
            let mut map = HashMap::new();
            map.insert("whole", 2u16.pow(0));
            map.insert("half", 2u16.pow(1));
            map.insert("quarter", 2u16.pow(2));
            map.insert("eighth", 2u16.pow(3));
            map.insert("16th", 2u16.pow(4));
            map.insert("32nd", 2u16.pow(5));
            map.insert("64th", 2u16.pow(6));
            map.insert("128th", 2u16.pow(7));
            map.insert("256th", 2u16.pow(8));
            map.insert("512th", 2u16.pow(9));
            map
        };
    }

    pub type Offset = Frac;
    pub type Octave = u8;

    #[derive(Debug, Eq, PartialEq, Hash, Clone)]
    pub enum Syllabic {single, begin, end, middle}

    #[derive(Debug, Eq, PartialEq, Hash, Clone)]
    pub struct Lyric {
        syllabic: Syllabic,
        text: String,
        number: u8, // used to specify which line the lyric is on
    }
    impl Lyric {
        pub fn from_xml_tag(lyric_tag: &XmlTag) -> Self {
            Self
            {
                syllabic: match lyric_tag.get_tag_content("syllabic")
                    .unwrap().as_str()
                {
                    "single" => Syllabic::single,
                    "begin" => Syllabic::begin,
                    "end" => Syllabic::end,
                    "middle" => Syllabic::middle,
                    _ => unreachable!()
                },

                text: lyric_tag.get_tag_content("text").as_ref().unwrap().clone(),

                number: lyric_tag.get_attribute_as("number").or(Some(1)).unwrap()
            }
        }
    }
}
/// Represents a generalized note that could be an actual note, rest, or chord
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum GnoteVariants {
    Note, Rest, Chord
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Gnote<'a> {
    // attributes that we dont care about
    pub _xml_attrs: &'a Vec<OwnedAttribute>,

    pub variant: GnoteVariants,
    // pitch information
    pub pitch: Vec< note_attr::Pitch>,
    pub accidental: Option<Accidental>,

    // duration information
    pub duration: note_attr::Duration,
    pub time_mod: Option< note_attr::TimeModification>,

    length_type: note_attr::LengthType,
    dot: u8, // the number of dots in this note
    tie: Option< note_attr::Tie>,

    // lyrics
    pub lyrics: Vec< note_attr::Lyric>,
}

impl<'a> Gnote<'a>  {
    fn from_xml_tag(xml_tag: &'a XmlTag, attrs: &attributes) -> Gnote<'a> {
        Gnote {
            _xml_attrs: &xml_tag.attributes,

            variant: {
                if xml_tag.path_exists("rest") { GnoteVariants::Rest }
                else if xml_tag.path_exists("chord") { GnoteVariants::Chord }
                else { GnoteVariants::Note }
            },

            pitch: vec![
                Pitch::from_xml_tag(xml_tag.search_path_unique("pitch").unwrap())
            ],

            accidental: xml_tag.search_path_unique("accidental")
                .and_then(|acc| Some(Accidental::from_xml_tag(acc))),

            duration: Frac::new(
                xml_tag.get_tag_content_as("duration").unwrap(),
                attrs.divisions.unwrap() as u32
            ),

            time_mod: xml_tag.search_path_unique("time-modification")
                .and_then(|time_mod| Some(TimeModification::from_xml_tag(time_mod))),

            length_type: xml_tag
                .search_path_unique("type").as_ref().unwrap()
                .text.as_ref().unwrap()
                .clone(),

            dot: xml_tag.count_tag("dot"),

            tie: xml_tag.search_path_unique("tie")
                .and_then(|x| Some(Tie::from_xml_tag(x))),

            lyrics: xml_tag.search_path("lyric")
                .iter()
                .map(|x| -> Lyric {Lyric::from_xml_tag(x)} )
                .collect()
        }
    }

    fn to_xml_tag(&self) -> XmlTag {
        let mut builder = XmlTag::new_tag_builder();
        let mut note = builder.add_tag("note");
        note.add_attributes(self._xml_attrs);
        // variant
        if self.variant != GnoteVariants::Note {
            note.add_tag({
                match self.variant {
                    GnoteVariants::Rest => "rest",
                    GnoteVariants::Chord => "chord",
                    _ => unreachable!()
                }
            });
        }
        // pitch
        if self.variant == GnoteVariants::Note {
            note.direct_add_tag(self.pitch.last().unwrap().to_xml_tag());
        } else if self.variant == GnoteVariants::Chord {
            todo!()
        } else { unreachable!() }
        // accidental
        if self.accidental.is_some() {
            note.direct_add_tag(self.accidental.as_ref().unwrap().to_xml_tag());
        }
        // duration
        todo!();
        // length_type
        todo!();
        // dot
        todo!();
        // tie
        todo!();
        // lyrics
        todo!();
        todo!()
    }

    /// Merge the notes that belong to a chord in given LinkedList.
    fn merge_notes_in_chords(gnotes: &mut LinkedList<Gnote>) {
        todo!()
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use std::borrow::BorrowMut;
    use crate::msc::gnote::note_attr::*;

    #[test]
    fn test_note_split() {
        // let nt = Gnote {
        //     variant: GnoteVariants::Note,
        //
        // }
    }

    #[test]
    fn test_note(){
        let note_xml = XmlTag::from_buffer(note_xml1());
        let attr_xml = XmlTag::from_buffer(attr());
        let attr = attributes::from_xml_tag(&attr_xml);
        println!("{:#?}", attr);
        let gnote = Gnote::from_xml_tag(&note_xml, &attr);
        println!("{:#?}", gnote);

    }

    #[test]
    fn test_2() {

    }

    fn note_xml1() -> &'static [u8] {
        return r#"<note default-x="116.30" default-y="-120.00" dynamics="97.78">
        <pitch>
          <step>E</step>
          <octave>3</octave>
          </pitch>
        <duration>12</duration>
        <voice>5</voice>
        <tie type="start" />
        <lyric name="1" number="1">
          <syllabic>single</syllabic>
          <text>0.0</text>
        </lyric>
        <lyric name="2" number="2">
          <syllabic>single</syllabic>
          <text>dfs</text>
        </lyric>
        <type>half</type>
        <stem>down</stem>
        <dot/>
        <staff>2</staff>
        </note>"#.as_bytes();
    }

    fn note_xml2 () -> &'static [u8] {
        return r#"<note default-x="116.30" default-y="-110.00" dynamics="97.78">
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
        </note>"#.as_bytes();
    }

    fn attr () -> &'static [u8] {
        return r#"<attributes>
        <divisions>6</divisions>
        <key>
          <fifths>0</fifths>
          </key>
        <time>
          <beats>4</beats>
          <beat-type>4</beat-type>
          </time>
        <clef>
          <sign>F</sign>
          <line>4</line>
          </clef>
        </attributes>"#.as_bytes();
    }
}