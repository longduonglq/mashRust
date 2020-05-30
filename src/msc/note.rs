use std::rc::Weak;

#[derive(Debug)]
enum Accidental {
    Sharp, DSharp,
    Flat, DFlat,
    Natural
}

#[derive(Debug)]
enum Pitch {
    A, B, C, D, E, F, G,
    Rest
}

/// Type indicating beat location
type Offset = fraction::GenericFraction<u16>;

#[derive(Debug)]
pub struct Note {
    pub pitch: Pitch,
    pub accidental: Option<Accidental>,
    pub octave: i8,

    pub duration: i8,
    pub offset: Offset
}

impl Note {
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_1(){
        let nt = GeneralNote::Note (Note{
            pitch: Pitch::A,
            accidental: Some(Accidental::Natural),
            octave: 5,
            duration: 0,
            offset: Offset::new (1u16, 3u16)
        });
        print!("nt1: {:?}", nt);
        assert_eq!(1, 1)
    }
}