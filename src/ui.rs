use bevy::{
    prelude::*,
    text::{Text, TextSection, TextStyle},
};

pub struct GameUiPlugin;

#[derive(Component)]
pub struct DealButton;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
           .add_systems(Update, button_system);
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Button,
            DealButton,
            ButtonBundle {
                background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                transform: Transform::from_xyz(0.0, -200.0, 0.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(Text::new("Deal").with_sections([TextSection {
                value: "Deal".to_string(),
                style: TextStyle {
                    font_size: 20.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                    ..default()
                },
            }]));
        });
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<DealButton>)
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                color.0 = Color::srgb(0.35, 0.35, 0.35);
                info!("Deal button pressed!");
            }
            Interaction::Hovered => {
                color.0 = Color::srgb(0.25, 0.25, 0.25);
            }
            Interaction::None => {
                color.0 = Color::srgb(0.2, 0.2, 0.2);
            }
        }
    }
} 