// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------

#[cfg(not(feature = "dev"))]
use app::AppSettings;
use app::prelude::*;

#[cfg(feature = "dev")]
mod clap;

fn main() {
    #[cfg(feature = "dev")]
    let settings = clap::parse_args();

    #[cfg(not(feature = "dev"))]
    let settings = AppSettings::default();

    let mut app = App::new();
    app.add_plugins((DefaultPlugins, app::AppPlugin { settings }));
    app.run();
}
