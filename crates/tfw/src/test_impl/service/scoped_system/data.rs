use crate::prelude::*;

// Add all your data types here.
// That includes all helper types, components, and their derivatives
// (e.g. Resources, Events)

#[derive(Resource, Default, Deref, DerefMut, Copy, Clone, Hash, Debug, PartialEq, Eq)]
pub struct ScopedSystemValue(pub u32);
