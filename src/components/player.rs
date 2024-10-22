use crate::*;

#[derive(Component, Default)]
pub struct Player;

impl Player {
    pub const SHIFT_COLLIDER: f32 = 6.;

    pub fn spawn(mut commands: Commands, query_player: Query<Entity, Added<Player>>) {
        if let Ok(player) = query_player.get_single() {
            commands
                .entity(player)
                .insert(SpriteMovementAnimations {
                    walk_down: vec![0, 4, 8, 12],
                    walk_up: vec![2, 6, 10, 14],
                    walk_left: vec![1, 5, 9, 13],
                    walk_right: vec![3, 7, 11, 15],
                })
                .insert(RigidBody::Dynamic)
                .insert(LockedAxes::ROTATION_LOCKED)
                // Position the collider relative to the rigid-body.
                .with_children(|parent| {
                    parent.spawn((
                        TransformBundle::from(Transform::from_xyz(0., -Player::SHIFT_COLLIDER, 0.)),
                        Collider::ball(6.),
                        Friction::new(0.),
                    ));
                });
        }
    }
}

#[derive(Bundle, LdtkEntity, Default)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    state: AnimationState,
    sprite_sheet_animation: SpriteSheetAnimation,
    sprite_movement_animations: SpriteMovementAnimations,
    velocity: Velocity,
    #[worldly]
    worldly: Worldly,
    teleporting_to_entity_iid: EntityIid,
    #[from_entity_instance]
    z_from_y: DeriveZFromY,
}
