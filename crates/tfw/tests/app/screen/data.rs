use crate::prelude::*;

#[derive(Default, PartialEq, Eq, Hash, Debug, Clone, Copy, Reflect)]
pub enum Screens {
    #[default]
    Empty,
    NamedEntity,
    ScopedSystem,
    Lifecycle,
}
