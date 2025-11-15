use bevy::reflect::enum_hash;

use crate::prelude::*;

#[derive(Default, PartialEq, Eq, Hash, Debug, Clone, Copy, Reflect)]
pub enum Screens {
    #[default]
    Empty,
    NamedEntity,
    ScopedSystem,
}
impl From<Screens> for ScreenName {
    fn from(value: Screens) -> Self {
        enum_hash(&value).unwrap().into()
    }
}
