//! This file contains definitions related to a Stream.

use std::rc::Rc;
use super::part;

/// A stream stores its elements by reference
struct Stream {
    parts: Vec<Rc<part::Part>>
}

