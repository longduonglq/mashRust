use std::hash::Hash;
use std::collections::HashSet;
use std::rc::Rc;
use crate::parser::xml_tag::XmlTag;
use std::borrow::BorrowMut;
use std::ops::Deref;

pub mod note_attr {
    use std::collections::HashMap;

    #[derive(Debug, Eq, PartialEq, Hash, Clone)]
    pub enum Accidental {
        Sharp,
        DSharp,
        Flat,
        DFlat,
        Natural,
        None
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
        pub accidental: Accidental,
        pub octave: Octave
    }

    #[derive(Debug, Eq, PartialEq, Hash, Clone)]
    pub enum Tie {
        start,
        stop
    }

    // #[derive(Debug, Eq, PartialEq, Hash, Clone)]
    // pub struct TimeModification {
    //     actual: u8,
    //     normal: u8,
    //     normal_type: LengthType,
    // }
    pub type Duration = fraction::GenericFraction<u16>;

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

    pub type Offset = fraction::GenericFraction<u16>;
    pub type Octave = u8;

    #[derive(Debug, Eq, PartialEq, Hash, Clone)]
    pub enum Syllabic {single, begin, end, middle}

    #[derive(Debug, Eq, PartialEq, Hash, Clone)]
    pub struct Lyric {
        syllabic: Syllabic,
        text: String,
        number: u8, // used to specify which line the lyric is on
    }
}
/// Represents a generalized note that could be an actual note, rest, or chord
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum GnoteVariants {
    Note, Rest, Chord
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Gnote {
    pub variant: GnoteVariants,
    // pitch information
    pub pitch: Vec< note_attr::Pitch>,

    // duration information
    pub duration: note_attr::Duration,
    length_type: note_attr::LengthType,
    dot: i8, // the number of dots in this note
    tie: Option< note_attr::Tie>,

    pub lyrics: Vec< note_attr::Lyric>,

    // indicate position of self in a stream object.
    pub offset: Option< note_attr::Offset>
}

impl Gnote  {
    fn note_from_xml (xml_note: &XmlTag) -> Self {
        unimplemented!()
    }


    fn split (&mut self, duration: note_attr::Duration) -> (Gnote, Gnote) {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use std::borrow::BorrowMut;
    use crate::msc::general_note::note_attr::*;

    #[test]
    fn test_note_split() {
        // let nt = Gnote {
        //     variant: GnoteVariants::Note,
        //
        // }
    }

    #[test]
    fn test_note(){
        println!("{:?}", LENGTH_TYPE_TABL["eighth"])
    }

    #[test]
    fn test_2() {

    }
}