use app::prelude::*;
use avian3d::prelude::Collider;

#[derive(Component)]
struct Cube;

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

fn update(mut query: Query<&mut Transform, With<Cube>>, time: Res<Time>) {
    let mut tf = r!(query.single_mut());
    *tf = tf.with_translation(Vec3::new(
        3. * f32::cos(time.elapsed_secs()) - 1.5,
        1.,
        3. * f32::sin(time.elapsed_secs()) - 1.5,
    ));
}

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, ServicesPlugin))
        .add_systems(Startup, init)
        .add_systems(Update, update)
        .run();
}
