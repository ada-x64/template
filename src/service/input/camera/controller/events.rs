use crate::prelude::*;

pub fn insert_camera_controller<'w>(mut world: DeferredWorld<'w>, ctx: HookContext) {
    let entity = ctx.entity;
    let controller = world.get::<CameraController>(entity).cloned();
    if let Some(mut camera) = world.get_mut::<Camera>(entity)
        && let Some(controller) = controller
    {
        camera.is_active = controller.active;
    }
}

pub fn plugin(_app: &mut App) {}
