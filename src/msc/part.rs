use std::rc::Rc;
use std::borrow::Cow;
use crate::msc::measure::Measure;
use crate::msc::general_note::{Gnote, note_attr};
use crate::msc::general_note::note_attr::Offset;
use std::cell::RefCell;

pub struct Part {
    duration: note_attr::Duration,
    measures: Vec< Measure>,
    notes: Vec< Rc< RefCell<Gnote>>>
}

impl Part {
    fn new(measure_n: usize) -> Self {
        Part {
            duration: note_attr::Duration::from(0u8),
            measures: Vec::with_capacity(measure_n),
            notes: Vec::with_capacity(measure_n * 16)
        }
    }

    // fn push_note(&mut self, note: Gnote) {
    //     let offset: Offset = Offset::from(0u16);
    //     if self.notes.last().is_some() {
    //         let last_note = self.notes.last().unwrap().get_mut();
    //         offset = last_note.offset + last_note.duration;
    //     }
    //     note.offset = offset;
    //     self.notes.push(Rc::new(RefCell::new (note)));
    // }
}

mod tests {
    use super::*;
    use std::borrow::Borrow;
    use crate::msc::general_note::Gnote;
    use crate::msc::general_note::note_attr::*;

    #[test]
    fn test_1 () {
    }
}
