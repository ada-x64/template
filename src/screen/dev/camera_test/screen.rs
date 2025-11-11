use crate::{prelude::*, screen::dev::camera_test};

#[derive(PartialEq, Eq, Clone, Debug, Hash, Reflect, Default, Resource)]
pub struct CameraTestSettings;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct CameraTestScreen;
impl Screen for CameraTestScreen {
    type SETTINGS = CameraTestSettings;

    fn name() -> ScreenType {
        Screens::CameraTest.into()
    }

    fn init<'w>(mut world: DeferredWorld<'w>, _ctx: HookContext) {
        world.commands().run_system_cached(init);
    }
}

/// spawn the scene.
/// this is temp, ideally load the scene from file
/// then spawn it
fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // spawn everything
    let cube = meshes.add(Cuboid::default());
    let plane = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(100.)));
    let wall = meshes.add(Cuboid::new(100., 100., 1.));
    let material = materials.add(StandardMaterial::default());

    let cube_entt = commands
        .spawn((
            Cube,
            Transform::default(),
            Mesh3d(cube),
            MeshMaterial3d(material.clone()),
            Collider::cuboid(0., 0., 0.),
        ))
        .id();
    commands.spawn((
        Transform::default(),
        Mesh3d(plane),
        MeshMaterial3d(material.clone()),
        Collider::half_space(Vec3::Y),
    ));
    commands.spawn((
        Transform::from_xyz(0., 0., -10.),
        Mesh3d(wall),
        MeshMaterial3d(material),
        Collider::half_space(Vec3::Z),
    ));
    commands.spawn((PointLight::default(), Transform::from_xyz(0., 3., 0.)));
    commands.trigger(SpawnGlobalCtx);
    commands.trigger(SpawnCursorCapture);
    commands.spawn((tracking_cam_bundle(cube_entt), Name::new("Tracking Cam")));
    commands.spawn((flycam_bundle(), Name::new("Fly Cam")));
}

pub fn plugin(app: &mut App) {
    ScreenScopeBuilder::<CameraTestScreen>::fixed()
        .add_systems(camera_test::systems().take())
        .add_systems(camera_systems().take())
        .build(app);
}
