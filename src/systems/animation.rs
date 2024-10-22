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

#[derive(Component, Reflect)]
pub enum AnimationState {
    Idle,
    MovingUp,
    MovingDown,
    MovingLeft,
    MovingRight,
}

impl Default for AnimationState {
    fn default() -> Self {
        AnimationState::Idle
    }
}

#[derive(Component, Default, Reflect)]
pub struct SpriteMovementAnimations {
    pub walk_down: Vec<usize>,
    pub walk_up: Vec<usize>,
    pub walk_left: Vec<usize>,
    pub walk_right: Vec<usize>,
}

pub fn animation(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
    mut query: Query<(
        &mut TextureAtlas,
        &mut SpriteSheetAnimation,
        Option<&AnimationState>,
        Option<&SpriteMovementAnimations>,
    )>,
) {
    for (mut texture_atlas, mut animation, sprite_state, sprite_mvt_animations) in &mut query {
        if let Some(state) = sprite_state {
            if let Some(animations) = sprite_mvt_animations {
                animation.timer.tick(time.delta());
                if animation.timer.finished() {
                    match state {
                        AnimationState::MovingUp => {
                            texture_atlas.index = animations.walk_up
                                [animation.current_frame % animations.walk_up.len()];
                            animation.current_frame += 1;
                        }
                        AnimationState::MovingDown => {
                            texture_atlas.index = animations.walk_down
                                [animation.current_frame % animations.walk_down.len()];
                            animation.current_frame += 1;
                        }
                        AnimationState::MovingLeft => {
                            texture_atlas.index = animations.walk_left
                                [animation.current_frame % animations.walk_left.len()];
                            animation.current_frame += 1;
                        }
                        AnimationState::MovingRight => {
                            texture_atlas.index = animations.walk_right
                                [animation.current_frame % animations.walk_right.len()];
                            animation.current_frame += 1;
                        }
                        AnimationState::Idle => {
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
            }
        } else {
            // for simple sheets (without states)
            animation.timer.tick(time.delta());
            if animation.timer.finished() {
                let texture_atlas_layout = texture_atlases.get(&texture_atlas.layout).unwrap();

                texture_atlas.index =
                    (texture_atlas.index + 1) % texture_atlas_layout.textures.clone().len();
            }
        };
    }
}
