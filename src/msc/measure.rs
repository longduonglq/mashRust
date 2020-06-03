use crate::msc::general_note::GenNote;
use crate::msc::key_sig::KeySignature;

pub enum Clef {
    G, F, Alto
}

pub type TimeSignature = (u8, u8);

pub struct Measure {
    number: u16,

    clef: Option<Clef>,
    time_sig: Option<TimeSignature>,
    key_sig: Option<KeySignature>,

    notes: Vec< GenNote>,
}

impl Measure {
}