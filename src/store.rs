use crate::float::Float;
use std::cell::RefCell;
use std::collections::HashMap;
pub type Store = HashMap<usize, RefCell<Float>>;
