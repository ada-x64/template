use crate::prelude::*;

#[derive(States, Debug, Default, Deref, DerefMut, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Step(pub u32);
