use std::hash::Hash;
use std::collections::HashSet;
use std::rc::Rc;
use crate::msc::duration::Duration;
use crate::msc::general_note::note_attr::TimeModification;
use crate::msc::parser::xml_tag::XmlTag;

pub mod note_attr {
    use std::collections::HashMap;

    #[derive(Debug, Eq, PartialEq, Hash)]
    pub enum Accidental {
        Sharp,
        DSharp,
        Flat,
        DFlat,
        Natural,
        None
    }

    #[derive(Debug, Eq, PartialEq, Hash)]
    pub enum Step {
        A,
        B,
        C,
        D,
        E,
        F,
        G
    }

    #[derive(Debug, Eq, PartialEq, Hash)]
    pub enum Tie {
        start,
        stop
    }

    #[derive(Debug, Eq, PartialEq, Hash)]
    pub struct TimeModification {
        actual: u8,
        normal: u8,
        normal_type: LengthType,
    }

    pub type LengthType = String;
    lazy_static! {
        pub static ref LengthTypeMap: HashMap<&'static str, u16> = {
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

    #[derive(Debug, Eq, PartialEq, Hash)]
    pub enum Syllabic {single, begin, end, middle}

    #[derive(Debug, Eq, PartialEq, Hash)]
    pub struct Lyric {
        syllabic: Syllabic,
        text: String,
        number: u8, // used to specify which line the lyric is on
    }
}

/// Represents an actual note
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct note {
    step: note_attr::Step,
    accidental: note_attr::Accidental,
    octave: note_attr::Octave,
    duration: Duration,
    length_type: note_attr::LengthType,

    tie: Option<note_attr::Tie>,
    time_mod: Option<note_attr::TimeModification>,
    lyrics: Vec<note_attr::Lyric>,
}


/// Simple Rest
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct rest {
    duration: Duration,
}


/// Represents a chord.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct chord {
    notes: Vec<note>,
    duration: Duration,
}

/// A general note object that gets stored in Part object
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum GenNote {
    Note(note),
    Rest(rest),
    Chord(chord),
}

#[cfg(test)]
mod tests{
    use super::*;
    use std::borrow::BorrowMut;

    #[test]
    fn test_note(){

    }

    #[test]
    fn test_2() {

    }
}