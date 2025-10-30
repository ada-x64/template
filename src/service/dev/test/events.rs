use crate::prelude::*;

fn on_event(trigger: Trigger<TestEvent>) {
    // Replace me!
}

pub fn plugin(app: &mut App) {
    app.add_observer(on_event);
}
