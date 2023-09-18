mod bullet;
mod target;
mod tower;

pub use bullet::*;
pub use target::*;
pub use tower::*;

use crate::TimerMode::Repeating;
use bevy::prelude::*;
use bevy::render::color::Color;
use bevy::utils::FloatOrd;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TowerPlugin, TargetPlugin, BulletPlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, (spawn_camera, spawn_basic_scene))
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 5.0,
                ..default()
            })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Name::new("Plane"));
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("light"));
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.68, 0.84, 0.92).into()),
            transform: Transform::from_xyz(0.0, 0.5, -1.0),
            ..default()
        })
        .insert(Tower {
            shooting_timer: Timer::from_seconds(1.0, Repeating),
            bullet_offset: Vec3::ZERO,
        })
        .insert(Name::new("Tower"));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
            material: materials.add(Color::rgb(0.68, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-2.0, 0.2, 0.5),
            ..default()
        })
        .insert(Target { speed: 0.3 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
            material: materials.add(Color::rgb(0.68, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-3.0, 0.2, 0.5),
            ..default()
        })
        .insert(Target { speed: 0.3 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));
}
