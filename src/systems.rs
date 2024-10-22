pub mod animation;
pub mod collision_events;
pub use animation::*;
pub use collision_events::*;

use crate::*;

// Systems - in this module, I keep systems that run on every tick.
// Systems for spawning entities and theirs components I keep in components.

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut PlayerAnimationState), With<Player>>,
) {
    if let Ok((mut velocity, mut state)) = query.get_single_mut() {
        const SPEED: f32 = 150.;

        let default = Vect::default();
        if velocity.linvel != default {
            velocity.linvel = default;
        }

        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            velocity.linvel += Vect::new(0., SPEED);
            *state = PlayerAnimationState::MovingUp;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            velocity.linvel += Vect::new(-SPEED, 0.);
            *state = PlayerAnimationState::MovingLeft;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            velocity.linvel += Vect::new(0., -SPEED);
            *state = PlayerAnimationState::MovingDown;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            velocity.linvel += Vect::new(SPEED, 0.);
            *state = PlayerAnimationState::MovingRight;
        }

        if velocity.linvel == default {
            *state = PlayerAnimationState::Idle;
        }
    }
}

pub fn derive_z_from_y_after_move(
    mut player_query: Query<(&mut Transform, &DeriveZFromY), Changed<Transform>>,
) {
    if let Ok((mut transform, dzfy)) = player_query.get_single_mut() {
        transform.translation.z = dzfy.get(transform.translation.y);
    }
}

pub fn move_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}
