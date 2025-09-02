pub(crate) mod data;

use avian3d::prelude::*;
use bevy::prelude::*;
use data::*;

use crate::screens::ScreenStates;

#[derive(Component, PartialEq, Eq, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct WorldgenRoot;
pub fn spawn_worldgen_root(
    mut commands: Commands,
    assets: Res<WorldgenHandles>,
    meshes: Res<Assets<Mesh>>,
) {
    commands.spawn((
        WorldgenRoot,
        GlobalTransform::IDENTITY,
        Visibility::Hidden,
        Name::new("WorldgenRoot"),
        StateScoped(ScreenStates::InWorld),
        children![(
            Name::new("World Mesh"),
            Collider::convex_hull_from_mesh(meshes.get(&assets.mesh).unwrap()).unwrap(),
            Mesh3d(assets.mesh.clone()),
            Visibility::Visible,
            MeshMaterial3d(assets.material.clone()),
            Transform::from_xyz(0., 0., 0.)
        )],
    ));
}

pub fn plugin(app: &mut App) {
    app.register_type::<WorldgenRoot>()
        .init_resource::<WorldgenHandles>()
        .register_type::<WorldgenHandles>();
}
