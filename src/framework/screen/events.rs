use crate::prelude::*;

fn log_screen_change(trigger: Trigger<StateTransitionEvent<CurrentScreen>>) {
    debug!("{:?}", *trigger);
}
fn log_state_change(trigger: Trigger<StateTransitionEvent<CurrentScreenStatus>>) {
    debug!("{:?}", *trigger);
}

pub fn plugin(app: &mut App) {
    #[cfg(debug_assertions)]
    app.add_observer(log_state_change)
        .add_observer(log_screen_change);
}
