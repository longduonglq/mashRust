use crate::msc::key_sig::KeySignature;
use std::rc::Rc;
use crate::msc::general_note::Gnote;

#[derive(Debug)]
pub enum Clef {
    G, F, Alto
}

pub type TimeSignature = (u8, u8);

pub struct Measure {
    pub number: Option<u16>,

    pub clef: Option<Clef>,
    pub time_sig: Option<TimeSignature>,
    pub key_sig: Option<KeySignature>,

    pub notes: Vec< Rc< Gnote>>
}

impl Measure {
    fn new(clef: Option<Clef>, time_sig: Option<TimeSignature>, key_sig: Option<KeySignature>) -> Self {
        Measure {
            number: None,
            clef,
            time_sig,
            key_sig,
            notes: vec![]
        }
    }

}

mod tests {
    #[test]
    fn test_01 () {

    }
}