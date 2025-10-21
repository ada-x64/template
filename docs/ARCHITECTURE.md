# Architecture

How are the modules arranged?

## Top-level architecture

Generally, the crate is split between _screens_ and _services._ Screens handle
initialization, asset loading, and define simualation scopes. Services provide
the actual functionality for the application.

```
./src
├── screens
│   ├── world/
│   ├── splash/
│   ├── main_menu/
│   ├── data.rs
│   └── mod.rs
├── services
│   ├── dev/
│   ├── input/
│   ├── player/
│   ├── text/
│   ├── ui/
│   ├── worldgen/
│   ├── data.rs
│   ├── mod.rs
│   └── third_party.rs
├── lib.rs
└── main.rs
```

### Screens

These are the equivalent of scenes or rooms in other game engines. Not to be
confused with Bevy's
[Scene](https://docs.rs/bevy/latest/bevy/prelude/struct.Scene.html) concept,
which is just a serialized world.

Screens define a simulation's _scope_ and gates off functionality behind
_states._ For example, `WorldScreenStates` may be `Loading` or `Ready` depending
on whether or not all of the assets have finished loading. We use
`bevy_asset_loader` for this.

### Services

Services are the meat of the framework. They contain modularized plugins for
defining simulation state. In other words, they provide the functionality
which makes the up the game itself. Note that there is really no strict
architectural difference between services and screens, but we treat them
as such because doing so allows us to be clear about what is state and what
is simulation.

## Modules

Each module is organized like this:

```
./src/services/input/camera
├── controller
│   ├── data.rs
│   ├── events.rs
│   └── mod.rs
├── data
│   ├── input_ctx.rs
│   └── mod.rs
├── fly
│   ├── bundle.rs
│   ├── data.rs
│   ├── events.rs
│   └── mod.rs
├── tracking
│   ├── bundle.rs
│   ├── data.rs
│   ├── events.rs
│   ├── mod.rs
│   └── systems.rs
└── mod.rs
```

Modules may have the following files:

- `mod.rs` - the entrypoint. Should contain a prelude and a plugin.
- `data.rs` - Components, Assets, and other datatypes required for the module.
    - If necessary, this can be split up.
- `systems.rs` - Systems which run directly in schedules.
- `events.rs` - Event observers.
    - NOTE: Buffered event handling should go in `systems.rs`, as it involves updating at a particular schedule.
- `bundle.rs` - A function which returns a bundle.
- `state.rs` - State management. Typically handles asset loading and screen scoping.

### `mod.rs`

Here's a nice simple example.

```rust
// mod.rs
use crate::prelude::*;

// Private modules. Avoid exposing anything unecessary.
// Using the prelude pattern helps a lot with imports!
mod controller;
mod data;
mod fly;
mod tracking;

// The prelude for this module. It should contain all the datatypes
// and the preludes of its submodules, as well as anything which may
// need to be publicly exposed.
pub mod prelude {
    pub use super::controller::prelude::*;
    pub use super::data::*;
    pub use super::fly::prelude::*;
    pub use super::tracking::prelude::*;
}

// The plugin is how this module exposes itself to the application.
// Add all your submodule plugins here, as well as any resources and assets.
pub fn plugin(app: &mut App) {
    app.add_plugins((tracking::plugin, fly::plugin, controller::plugin))
        .init_resource::<CameraList>()
        .init_resource::<ActiveCamera>();
}
```

### `data.rs`

This file should include _all_ data. Everything in it should be publically accessible.

Data includes:

- Components
- Resources
- Assets
- Helper structs like bitmasks
- etc.!

As a rule of thumb, if it's not _doing_ something it belongs in `data.rs`.

```rust
// data.rs
use crate::prelude::*;

#[derive(Component)]
#[require(CameraController::new(CameraControllerKind::Fly), ICtxFlyCam)]
pub struct FlyCam;

#[derive(Component, Default)]
pub struct ICtxFlyCam;
```

### `bundle.rs`

If you need to expose a bundle, do so here. Prefer a functional approach.

```rust
// bundle.rs
use crate::prelude::*;

pub fn flycam_bundle() -> impl Bundle {
    (
        FlyCam,
        // ...
    )
}
```

`bundle.rs` should not expose a plugin. Instead, the bundle function should be
exposed in the module prelude.

```rust
// mod.rs
//...
pub mod prelude {
    pub use super::bundle::flycam_bundle;
    pub use super::data::*;
}
```

You can think of bundles as something like a prefab in Unity or an actor in
Unreal.

### `events.rs`

This should include every observer event. Note that before 0.17, `Message`s were
_also_ called `Event`s. Do not include message handling here. That should happen
in `systems.rs`.

```rust
// events.rs

use crate::prelude::*;

fn on_rotate(/* ... */) {}
fn on_zoom(/* ... */) {}

pub fn plugin(app: &mut App) {
    app.add_observer(on_rotate).add_observer(on_zoom);
}
```

### `systems.rs`

Anything that must happen every frame on a fixed schedule should be implemented
here. Generally, you should prefer using events to implement reactive game logic
and scheduled systems for implementing the global simulation.

```rust
// systems.rs

use crate::prelude::*;

fn init(/* ... */) {}

fn apply() {}

/// global startup code should be here, if it's always running
pub fn plugin(app: &mut App) {
    app.add_systems(Startup, apply);
}

/// to scope things, place them like this, then integrate them
/// into the screen's systems module
pub fn systems() -> ServiceSystems {
    ServiceSystems::new(apply)
}
```

### `state.rs`

Handles states, like asset loading. When possible, **services should not have state.**

```rust
// state.rs
use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<WorldScreenStates>();
    app.add_loading_state(
        LoadingState::new(WorldScreenStates::Loading)
            .continue_to_state(WorldScreenStates::Ready)
            .load_collection::<PlayerAssets>(),
    );
}
```

You may be tempted to set up system scopes within a service. Do not do this!
Only call `app.config_sets` within `state.rs` - i.e., within a screen module.

## Special modules

Outside of services and screens, we need to integrate with third-party modules
and create an application.

The top-level services module includes a `third_party` submodule.
This simply integrates all third party plugins and exposes their preludes.

```rust
/// third_party.rs

use crate::prelude::*;

pub mod prelude {
    pub use avian3d::prelude::*;
    // ...
}

pub fn plugin(app: &mut App) {
    app.add_plugins((
        avian3d::PhysicsPlugins::default(),
        // ...
    ));
}
```

The top-level service module itself exposes _two_ plugins, one for public
functionality and one for private functionality.

And finally, we have the standard `lib.rs` and `main.rs`. We split the crate
into a library and executable so that we can have "example" executables for
testing service functionality. `lib.rs` simply exposes the `ScreenPlugin` and
`ServicesPlugin` as well as the prelude. `main.rs` adds them all and runs the
application.
