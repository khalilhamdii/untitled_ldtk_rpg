use crate::*;

use crate::colliders::ColliderBundle;

#[derive(Bundle, LdtkEntity, Default)]
pub struct FrogBundle {
    #[with(derive_z_from_y)]
    z_from_y: DeriveZFromY,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    sprite_sheet_animation: SpriteSheetAnimation,
    #[with(collider)]
    collider: Collider,
    #[ldtk_entity]
    pub patrol: Patrol,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Enemy;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct MobBundle {
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub enemy: Enemy,
    #[ldtk_entity]
    pub patrol: Patrol,
}

fn derive_z_from_y(_: &EntityInstance) -> DeriveZFromY {
    12.into()
}

fn collider(_: &EntityInstance) -> Collider {
    Collider::ball(12.)
}
