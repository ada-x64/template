use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(WorldScreenStates::Ready),
        (|mut commands: Commands| {
            commands.trigger(SpawnPlayerRoot);
            commands.trigger(SpawnWorldgenRoot);
        })
        .run_if(in_state(ScreenStates::InWorld)),
    );
}
