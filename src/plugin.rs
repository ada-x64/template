use tfw::TfwSettings;

use crate::prelude::*;

/// Allows for configuration of the application. When the "dev" feature is set,
/// this should be handled via command line arguments. Otherwise, it is kept as the
/// default value.
#[derive(Resource, Clone, Debug)]
pub struct AppSettings {
    pub initial_screen: Screens,
    pub use_physics: bool,
}
#[allow(clippy::derivable_impls)]
impl Default for AppSettings {
    fn default() -> Self {
        #[cfg(not(test))]
        {
            Self {
                initial_screen: Default::default(),
                use_physics: true,
            }
        }
        #[cfg(test)]
        {
            Self {
                initial_screen: Default::default(),
                use_physics: false,
            }
        }
    }
}

/// The main exported plugin for the application.
#[derive(Default, Clone)]
pub struct AppPlugin {
    pub settings: AppSettings,
}
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.world_mut().insert_resource(self.settings.clone());
        app.add_plugins((
            tfw::TfwPlugin {
                settings: TfwSettings {
                    initial_screen: self.settings.initial_screen.into(),
                },
            },
            crate::service::plugin,
            crate::screen::plugin,
        ))
        .insert_resource(self.settings.clone());
    }
}
