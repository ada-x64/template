use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum WorldInspectorState {
    #[default]
    Invisible,
    Visible,
}
