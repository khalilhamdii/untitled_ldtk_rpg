use crate::*;

#[derive(Bundle, LdtkEntity, Default)]
pub struct FrogBundle {
    #[with(derive_z_from_y)]
    z_from_y: DeriveZFromY,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    sprite_sheet_animation: SpriteSheetAnimation,
    #[with(collider)]
    collider: Collider,
}

fn derive_z_from_y(_: &EntityInstance) -> DeriveZFromY {
    12.into()
}

fn collider(_: &EntityInstance) -> Collider {
    Collider::ball(12.)
}
