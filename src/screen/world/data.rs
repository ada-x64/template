use crate::prelude::*;

#[derive(States, PartialEq, Eq, Hash, Debug, Clone, Copy, Reflect, Default)]
pub enum WorldScreenStates {
    #[default]
    Loading,
    Ready,
}
