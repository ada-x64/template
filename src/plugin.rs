use crate::prelude::*;

/// Allows for configuration of the application. When the "dev" feature is set,
/// this should be handled via command line arguments. Otherwise, it is kept as the
/// default value.
#[derive(Resource, Default, Clone, Debug)]
pub struct AppSettings {
    pub initial_screen: Screens,
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
            crate::framework::plugin,
            crate::service::plugin,
            crate::screen::plugin,
        ))
        .insert_resource(self.settings.clone());
    }
}
