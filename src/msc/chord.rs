use crate::msc::types::Duration;
use crate::msc::note::{Note};

pub struct Chord {
    notes: Vec<Note>,
    duration: Duration
}