use std::ops::Range;

use crate::*;

#[derive(Component)]
pub struct SpriteSheetAnimation {
    pub state_range: Option<Range<usize>>,
    pub timer: Timer,
}

impl Default for SpriteSheetAnimation {
    fn default() -> Self {
        Self {
            state_range: None,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

pub fn animation(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
    mut query: Query<(&mut TextureAtlas, &mut SpriteSheetAnimation)>,
) {
    for (mut texture_atlas, mut animation) in &mut query {
        if let Some(state_range) = animation.state_range.clone() {
            if animation.is_changed() {
                texture_atlas.index = state_range.start;
            } else {
                animation.timer.tick(time.delta());
                if animation.timer.finished() {
                    let next_index = texture_atlas.index + 1;
                    texture_atlas.index = if next_index < state_range.end {
                        next_index
                    } else {
                        state_range.start
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
