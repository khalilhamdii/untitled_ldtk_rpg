use crate::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SpriteSheetAnimation {
    pub current_frame: usize,
    pub timer: Timer,
}

impl Default for SpriteSheetAnimation {
    fn default() -> Self {
        Self {
            current_frame: 0,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

pub fn animation(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
    mut query: Query<(
        &mut TextureAtlas,
        &mut SpriteSheetAnimation,
        Option<&PlayerAnimationState>,
    )>,
    animations: Res<PlayerAnimations>,
) {
    for (mut texture_atlas, mut animation, sprite_state) in &mut query {
        if let Some(state) = sprite_state {
            animation.timer.tick(time.delta());
            if animation.timer.finished() {
                match state {
                    PlayerAnimationState::MovingUp => {
                        texture_atlas.index =
                            animations.walk_up[animation.current_frame % animations.walk_up.len()];
                        animation.current_frame += 1;
                    }
                    PlayerAnimationState::MovingDown => {
                        texture_atlas.index = animations.walk_down
                            [animation.current_frame % animations.walk_down.len()];
                        animation.current_frame += 1;
                    }
                    PlayerAnimationState::MovingLeft => {
                        texture_atlas.index = animations.walk_left
                            [animation.current_frame % animations.walk_left.len()];
                        animation.current_frame += 1;
                    }
                    PlayerAnimationState::MovingRight => {
                        texture_atlas.index = animations.walk_right
                            [animation.current_frame % animations.walk_right.len()];
                        animation.current_frame += 1;
                    }
                    PlayerAnimationState::Idle => {
                        if animations.walk_up.contains(&texture_atlas.index) {
                            texture_atlas.index = animations.walk_up[0];
                        }
                        if animations.walk_down.contains(&texture_atlas.index) {
                            texture_atlas.index = animations.walk_down[0];
                        }
                        if animations.walk_right.contains(&texture_atlas.index) {
                            texture_atlas.index = animations.walk_right[0];
                        }
                        if animations.walk_left.contains(&texture_atlas.index) {
                            texture_atlas.index = animations.walk_left[0];
                        }
                    }
                }
            }
        } else {
            // for simple sheets (without states)
            // so that you don't have to always setup the exact number of frames if a sheet has just one state
            animation.timer.tick(time.delta());
            if animation.timer.finished() {
                let texture_atlas_layout = texture_atlases.get(&texture_atlas.layout).unwrap();

                texture_atlas.index =
                    (texture_atlas.index + 1) % texture_atlas_layout.textures.clone().len();
            }
        };
    }
}
