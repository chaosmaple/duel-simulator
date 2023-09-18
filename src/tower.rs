use crate::bullet::Bullet;
use crate::target::Target;
use bevy::prelude::*;
use bevy::utils::FloatOrd;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
            .add_systems(Update, tower_shooting);
    }
}

fn tower_shooting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    time: Res<Time>,
) {
    for (tower_ent, mut tower, tower_g_tran) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let bullet_spawn = tower_g_tran.translation() + tower.bullet_offset;

            let direction = targets
                .iter()
                .min_by_key(|target_tran| {
                    FloatOrd(Vec3::distance(target_tran.translation(), bullet_spawn))
                })
                .map(|closet_target| closet_target.translation() - bullet_spawn);
            if let Some(direction) = direction {
                commands.entity(tower_ent).with_children(|commands| {
                    commands
                        .spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                            material: materials.add(Color::rgb(0.87, 0.44, 0.2).into()),
                            transform: Transform::from_translation(tower.bullet_offset),
                            ..default()
                        })
                        .insert(Bullet {
                            direction,
                            speed: 2.5,
                        })
                        .insert(Name::new("bullet"));
                });
            }
        }
    }
}
