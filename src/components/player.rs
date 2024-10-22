use crate::*;

#[derive(Component, Default)]
pub struct Player;

impl Player {
    pub const SHIFT_COLLIDER: f32 = 6.;

    pub fn spawn(mut commands: Commands, query_player: Query<Entity, Added<Player>>) {
        if let Ok(player) = query_player.get_single() {
            commands
                .entity(player)
                .insert(RigidBody::Dynamic)
                .insert(LockedAxes::ROTATION_LOCKED)
                // .insert(SpriteSheetAnimation::from(PlayerAnimationState::Idle))
                // For now does nothing, but to be used if it would become useful to track the facing direction of the player.
                // In case of a better sprite sheet or interaction with entities.
                // .insert(super::Direction::Down)
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
    state: PlayerAnimationState,
    sprite_sheet_animation: SpriteSheetAnimation,
    velocity: Velocity,
    #[worldly]
    worldly: Worldly,
    teleporting_to_entity_iid: EntityIid,
    #[from_entity_instance]
    z_from_y: DeriveZFromY,
}

#[derive(Component, Reflect)]
pub enum PlayerAnimationState {
    Idle,
    MovingUp,
    MovingDown,
    MovingLeft,
    MovingRight,
}

// Implementing Default for Direction
impl Default for PlayerAnimationState {
    fn default() -> Self {
        PlayerAnimationState::Idle
    }
}

#[derive(Resource, Default, Reflect)]
pub struct PlayerAnimations {
    pub walk_down: Vec<usize>,
    pub walk_up: Vec<usize>,
    pub walk_left: Vec<usize>,
    pub walk_right: Vec<usize>,
}
