use std::{
    f32::consts::PI,
    f32::consts::{FRAC_PI_2, FRAC_PI_8},
};

use crate::prelude::*;

fn on_rotate(trigger: On<Fire<PARotateCam>>, mut controller: Query<&mut TrackingCam>) {
    let mut controller = rq!(controller.get_mut(trigger.event().event_target()));
    controller.rotation.x = (controller.rotation.x + trigger.value.x) % (2. * PI);
    controller.rotation.y = (controller.rotation.y + trigger.value.y).clamp(-FRAC_PI_8, FRAC_PI_8);
}

fn on_zoom(trigger: On<Fire<PAZoomCam>>, mut projections: Query<&mut Projection>) {
    let mut projection = projections.get_mut(trigger.event().event_target()).unwrap();
    let Projection::Perspective(projection) = &mut *projection else {
        panic!("camera should be perspective");
    };
    projection.fov = (projection.fov + trigger.value).clamp(FRAC_PI_8, FRAC_PI_2);
}

pub fn plugin(app: &mut App) {
    app.add_observer(on_rotate).add_observer(on_zoom);
}
