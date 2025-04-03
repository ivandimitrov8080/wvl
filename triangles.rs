use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub struct HelloPlugin;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cubes
    for i in 0..100 {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(i as f32, 0.5, 0.0),
        ));
    }
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn camera_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion_reader: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let mut speed = 0.1;
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            speed = 0.8;
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        let forward = transform.rotation * Vec3::Z;
        let right = transform.rotation * Vec3::X;
        let left = transform.rotation * Vec3::Y;
        transform.translation += (forward * direction.z + right * direction.x + left * direction.y) * speed;

        for event in mouse_motion_reader.read() {
            let yaw = Quat::from_rotation_y(-event.delta.x * 0.001);
            let pitch = Quat::from_rotation_x(-event.delta.y * 0.001);
            transform.rotation = yaw * transform.rotation * pitch;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyG) {
        cursor_grab(&mut q_windows);
    }
    if keyboard_input.just_pressed(KeyCode::KeyU) {
        cursor_ungrab(&mut q_windows);
    }
}

fn cursor_ungrab(q_windows: &mut Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();

    primary_window.cursor_options.grab_mode = CursorGrabMode::None;
    primary_window.cursor_options.visible = true;
}

fn cursor_grab(q_windows: &mut Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();

    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor_options.visible = false;
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, camera_movement);
    }
}

pub fn triangles() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}

