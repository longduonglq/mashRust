use std::rc::Rc;
use std::borrow::Cow;
use crate::msc::general_note::{GenNote};
use crate::msc::measure::Measure;
use crate::msc::duration::Duration;

pub struct Part {
    duration: Duration,
    measures: Vec< Measure>
}

impl Part {

}
