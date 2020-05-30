use crate::msc::types::{Octave, Duration};

#[derive(Debug)]
pub enum Accidental {
    Sharp, DSharp,
    Flat, DFlat,
    Natural
}

#[derive(Debug)]
pub enum Step {
    A, B, C, D, E, F, G
}

/// Type indicating beat location
#[derive(Debug)]
pub struct Note {
    pub step: Step,
    pub accidental: Option<Accidental>,
    pub octave: Octave,
    pub duration: Duration,
}

pub struct Rest {
    pub duration: Duration
}

pub enum GeneralNote {
    Note(Note),
    Rest(Rest)
}
impl GeneralNote {
    fn new_note (
        step: Step,
        accidental: Option<Accidental>,
        octave: Octave,
        duration: Duration
    ) -> Note
    {
        Note { step, accidental, octave, duration }
    }

    fn new_rest (
        duration: Duration
    ) -> Rest
    {
        Rest {duration}
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_1(){
        let nt = GeneralNote::new_note(
            Step::A,
           Some(Accidental::Natural),
            5,
            0,
        );
        print!("nt1: {:?}", nt);
        assert_eq!(1, 1)
    }
}