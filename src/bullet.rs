use crate::target::*;
use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Bullet {
    pub direction: Vec3,
    pub speed: f32,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Bullet>()
            .add_systems(Update, (move_bullets, bullet_collision));
    }
}

fn move_bullets(mut bullets: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
    }
}

fn bullet_collision(
    mut commands: Commands,
    mut targets: Query<(&mut Health, &Transform), With<Target>>,
    bullets: Query<(Entity, &GlobalTransform), With<Bullet>>,
) {
    for (mut health, tar_trans) in &mut targets {
        for (bullet_ent, bullet_trans) in &bullets {
            if Vec3::distance(tar_trans.translation, bullet_trans.translation()) < 0.2 {
                commands.entity(bullet_ent).despawn_recursive();
                health.value -= 1;
                break;
            }
        }
    }
}
