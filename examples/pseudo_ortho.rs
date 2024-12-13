//! Demonstrates a pseudo ortho camera - a camera that uses a very narrow perspective projection.
//! This might be useful if certain features are not supported in ortho.

use bevy::prelude::*;
use bevy_editor_cam::prelude::*;
use indoc::indoc;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin, DefaultEditorCamPlugins))
        .add_systems(Startup, (setup, setup_ui))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let diffuse_map = asset_server.load("environment_maps/diffuse_rgb9e5_zstd.ktx2");
    let specular_map = asset_server.load("environment_maps/specular_rgb9e5_zstd.ktx2");

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(1000.0, 1000.0, 1000.0).looking_at(Vec3::ZERO, Vec3::Y),
        Projection::Perspective(PerspectiveProjection {
            fov: 0.001,
            ..default()
        }),
        Camera {
            hdr: true,
            ..Default::default()
        },
        Msaa::Off,
        EnvironmentMapLight {
            intensity: 1000.0,
            diffuse_map: diffuse_map.clone(),
            specular_map: specular_map.clone(),
            rotation: default(),
        },
        // This component makes the camera controllable with this plugin:
        EditorCam::default(),
        // This is an extension made specifically for orthographic cameras. Because an ortho camera
        // projection has no field of view (and a pseudo-ortho projection has a very small field of
        // view), a skybox can't be sensibly rendered, only a single point on the skybox would be
        // visible to the camera at any given time. While this is technically correct to what the
        // camera would see, it is not visually helpful nor appealing. It is common for CAD software
        // to render a skybox with a field of view that is decoupled from the camera field of view.
        bevy_editor_cam::extensions::independent_skybox::IndependentSkybox::new(diffuse_map, 500.0),
    ));

    spawn_gltf(27, &asset_server, &mut commands);
}

fn spawn_gltf(n: usize, asset_server: &AssetServer, commands: &mut Commands) {
    let half_width = (((n as f32).powf(1.0 / 3.0) - 1.0) / 2.0) as i32;
    let scene = asset_server.load("models/PlaneEngine/scene.gltf#Scene0");
    let width = -half_width..=half_width;
    for x in width.clone() {
        for y in width.clone() {
            for z in width.clone() {
                commands.spawn((
                    SceneRoot(scene.clone()),
                    Transform::from_translation(IVec3::new(x, y, z).as_vec3() * 2.0)
                        .with_scale(Vec3::splat(1.)),
                ));
            }
        }
    }
}

fn setup_ui(mut commands: Commands) {
    let text = indoc! {"
        Left Mouse - Pan
        Right Mouse - Orbit
        Scroll - Zoom
    "};
    commands.spawn((
        Text::new(text),
        TextFont {
            font_size: 20.0,
            ..default()
        },
    ));
}
