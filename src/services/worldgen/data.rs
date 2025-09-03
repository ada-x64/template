use bevy::{color::palettes::css::SILVER, prelude::*};

#[derive(Event, Reflect, Debug, Copy, Clone)]
pub struct SpawnWorldgenRoot;

#[derive(Resource, Asset, Reflect, Debug)]
pub struct WorldgenHandles {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for WorldgenHandles {
    fn from_world(world: &mut World) -> Self {
        // this will eventually need to be asynchronous
        let mesh = Plane3d::default()
            .mesh()
            .size(50.0, 50.0)
            .subdivisions(10)
            .build();
        let mesh = world.resource_mut::<Assets<Mesh>>().add(mesh);
        let material = world
            .resource_mut::<Assets<StandardMaterial>>()
            .add(StandardMaterial {
                cull_mode: None,
                base_color: SILVER.into(),
                ..Default::default()
            });
        WorldgenHandles { mesh, material }
    }
}
