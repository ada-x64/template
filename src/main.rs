// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------

use app::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, ScreensPlugin, ServicesPlugin));
    app.run();
}
