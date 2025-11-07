use crate::prelude::*;
use bevy::ecs::query::QueryFilter;
use variadics_please::all_tuples;

/// A system parameter which acts as a negation flag. [Thanks to @gibimicro on
/// Discord for this
/// solution.](https://discord.com/channels/691052431525675048/742569353878437978/1432104694024769546)
pub type Not<T> = <T as NotImpl>::Not;

#[doc(hidden)]
pub trait NotImpl {
    type Not: QueryFilter;
}

impl<T: Component> NotImpl for With<T> {
    type Not = Without<T>;
}
impl<T: Component> NotImpl for Without<T> {
    type Not = With<T>;
}

macro_rules! impl_tuple_not {
    ($(#[$meta:meta])* $($name: ident),*) => {
        $(#[$meta])*
        impl<$($name: NotImpl),*> NotImpl for ($($name,)*) {
            type Not = Or<($($name::Not,)*)>;
        }
    };
}
all_tuples!(
    // #[doc(fake_variadic)]
    impl_tuple_not,
    0,
    15,
    F
);

macro_rules! impl_or_not {
    ($(#[$meta:meta])* $($name: ident),*) => {
        $(#[$meta])*
        impl<$($name: NotImpl),*> NotImpl for Or<($($name,)*)> {
            type Not = ($($name::Not,)*);
        }
    };
}
all_tuples!(
    // #[doc(fake_variadic)]
    impl_or_not,
    0,
    15,
    F
);
