use bevy::prelude::*;

#[derive(Component)]
pub struct TextBoxContainer;

#[derive(Component)]
pub struct TextBox;

pub fn spawn_children_text(
    font_handle: Handle<Font>,
    text: String,
) -> impl FnOnce(&mut ChildBuilder) {
    const FONT_SIZE: f32 = 30.;
    move |parent: &mut ChildBuilder| {
        parent.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font: font_handle.clone(),
                font_size: FONT_SIZE,
                color: Color::WHITE,
            },
        ));
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Bottom text box
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Px(200.),
            // align container to the bottom
            align_self: AlignSelf::FlexEnd,
            // makes space bellow the box
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        // transparent container
        background_color: Color::rgba(0., 0., 0., 0.).into(),
        // Debug - comment out invisibility on initial creation of the box to see it
        visibility: Visibility::Hidden,
        ..Default::default()
    })
    .insert(TextBoxContainer)
    .with_children(|parent| {
        // box size, border thickness and color
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(80.),
            height: Val::Percent(80.),
                border: UiRect::all(Val::Px(6.0)),
                ..Default::default()
            },
            background_color: Color::srgb(0.25, 0.64, 0.27).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // text background
            let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");
            parent.spawn(NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(10.)),
                    width: Val::Percent(100.),
                    ..Default::default()
                },
                background_color: Color::srgb(0.169, 0.169, 0.169).into(),
                ..Default::default()
            })
            .insert(TextBox)
            .insert(font_handle.clone())
            .with_children(spawn_children_text(font_handle, String::from(
                "Text Example a little longer trying to cross the width... Text Example a little longer trying to cross the width"
            )));
        });
    });
}
