use bevy::{
    prelude::*,
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
            NodeBundle {
                background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                transform: Transform::from_xyz(0.0, -200.0, 0.0),
                ..default()
            },
            Button,
            DealButton,
        ))
        .with_children(|parent| {
            parent.spawn(Text::new("Deal"));
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