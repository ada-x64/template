use crate::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug, Hash, Reflect, Default, Resource)]
pub struct CameraTestSettings;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct CameraTestScreen;
impl Screen for CameraTestScreen {
    type SETTINGS = CameraTestSettings;
    const NAME: ScreenType = Screens::CameraTest.as_screen_type();

    fn init<'w>(mut world: DeferredWorld<'w>, _ctx: HookContext) {
        let _settings = world.resource::<Self::SETTINGS>().clone();
        // ...
        // spawn the scene
        // note this is temp, ideally load the scene from file
        // then spawn it
        world.commands().run_system_cached(init);
    }
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cam_list: ResMut<CameraList>,
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
    let tc = commands
        .spawn((tracking_cam_bundle(cube_entt), Name::new("Tracking Cam")))
        .id();
    let fc = commands.spawn((flycam_bundle(), Name::new("Fly Cam"))).id();
    **cam_list = vec![fc, tc];
}

pub fn plugin(app: &mut App) {
    ScreenScopeBuilder::<CameraTestScreen>::fixed().build(app);
}
