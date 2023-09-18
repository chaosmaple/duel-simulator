use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: i32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Target {
    pub speed: f32,
}

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Target>()
            .register_type::<Health>()
            .add_systems(Update, (move_targets, target_death));
    }
}

fn move_targets(mut targets: Query<(&Target, &mut Transform)>, time: Res<Time>) {
    for (target, mut transform) in &mut targets {
        transform.translation.x += target.speed * time.delta_seconds();
    }
}

fn target_death(mut commands: Commands, mut targets: Query<(Entity, &Health)>) {
    for (target_ent, health) in &mut targets {
        if health.value <= 0 {
            commands.entity(target_ent).despawn_recursive();
        }
    }
}
