//! This file contains definitions related to a Stream.

use std::rc::Rc;
use super::types;
use std::path::Path;
use crate::msc::types::{Offset, Duration};
use crate::msc::chord::Chord;
use crate::msc::note::{GeneralNote};

pub struct Temporal<T> {
    offset: Offset,
    elem: T
}
impl PartElem for Temporal<Chord> {}
impl PartElem for Temporal<GeneralNote> {}

/// Represents a stream segments that contains note objects
pub struct Part {
    notes: Vec< Temporal<GeneralNote> >,

    duration: Duration
}
/// Elements stored in Part needs to impl this trait
pub trait PartElem {}

impl Part {
    fn from_path(path: &Path) -> Self {
        todo!()
    }

    fn at_offset(&self, offset: Offset) -> Box<dyn PartElem> {
        todo!()
    }

    fn at_exact_offset(&self, offset: Offset) -> Option<Box<dyn PartElem>> {
        todo!()
    }

    fn append_notes(&mut self, notes: Vec<GeneralNote>) {
        todo!()
    }

}