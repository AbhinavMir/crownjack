use bevy::prelude::*;
use std::collections::HashMap;
use bevy::input::mouse::MouseButton;
use bevy::window::PrimaryWindow;

pub struct ChessPlugin;

#[derive(Component)]
pub struct ChessPiece {
    pub piece_type: PieceType,
    pub color: PieceColor,
}

#[derive(Resource, Default)]
pub struct ChessAssets {
    pub pieces: HashMap<String, Handle<Image>>,
}

#[derive(Resource, Default)]
struct SelectedPiece {
    entity: Option<Entity>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct DraggedPiece;

#[derive(PartialEq, PartialOrd, Debug)]
struct FloatOrd(f32);

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChessAssets>()
           .init_resource::<SelectedPiece>()
           .add_systems(PreStartup, spawn_camera)
           .add_systems(Startup, (
               load_chess_assets,
               setup_chess_board.after(load_chess_assets)
           ))
           .add_systems(Update, (
               piece_drag_system,
               piece_release_system,
           ));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(0.0, 0.0, 1000.0),
        MainCamera,
    ));
}

fn load_chess_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut pieces = HashMap::new();
    
    for color in ["W", "B"] {
        for piece_type in ["_Pawn", "_Knight", "_Bishop", "_Rook", "_Queen", "_King"] {
            let filename = format!("chess/{}{}.png", color, piece_type);
            pieces.insert(filename.clone(), asset_server.load(&filename));
        }
    }

    commands.insert_resource(ChessAssets { pieces });
}

fn setup_chess_board(
    mut commands: Commands,
    chess_assets: Res<ChessAssets>,
) {
    // Create the chess board (8x8 grid)
    for rank in 0..8 {
        for file in 0..8 {
            let position = Vec3::new(
                (file as f32 - 3.5) * 100.0,
                (rank as f32 - 3.5) * 100.0,
                0.0,
            );

            // Spawn square
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: if (rank + file) % 2 == 1 {
                        Color::srgb(0.4, 0.25, 0.15) // Darker, warmer brown for dark squares
                    } else {
                        Color::srgb(0.93, 0.93, 0.82) // Slightly cream color for light squares
                    },
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            });

            // Spawn pieces in their initial positions
            if let Some((piece_type, piece_color)) = get_initial_piece(rank, file) {
                let texture_path = format!("chess/{}{}.png",
                    if piece_color == PieceColor::White { "W" } else { "B" },
                    match piece_type {
                        PieceType::Pawn => "_Pawn",
                        PieceType::Knight => "_Knight",
                        PieceType::Bishop => "_Bishop",
                        PieceType::Rook => "_Rook",
                        PieceType::Queen => "_Queen",
                        PieceType::King => "_King",
                    }
                );

                if let Some(texture) = chess_assets.pieces.get(&texture_path) {
                    // Spawn shadow
                    commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(80.0, 80.0)),
                            color: Color::rgba(0.0, 0.0, 0.0, 0.15),
                            ..default()
                        },
                        transform: Transform::from_translation(position.with_z(9.0) + Vec3::new(6.0, -6.0, 0.0)),
                        ..default()
                    });

                    // Spawn piece without forcing square dimensions
                    commands.spawn((
                        ChessPiece { 
                            piece_type, 
                            color: piece_color 
                        },
                        SpriteBundle {
                            sprite: Sprite {
                                // Remove custom_size to maintain original aspect ratio
                                image: texture.clone(),
                                ..default()
                            },
                            transform: Transform::from_translation(position.with_z(10.0))
                                .with_scale(Vec3::splat(0.8)), // Scale uniformly to fit the squares
                            ..default()
                        },
                    ));
                }
            }
        }
    }
}

fn get_initial_piece(rank: i32, file: i32) -> Option<(PieceType, PieceColor)> {
    match rank {
        0 => Some((
            match file {
                0 | 7 => PieceType::Rook,
                1 | 6 => PieceType::Knight,
                2 | 5 => PieceType::Bishop,
                3 => PieceType::Queen,
                4 => PieceType::King,
                _ => return None,
            },
            PieceColor::Black,
        )),
        1 => Some((PieceType::Pawn, PieceColor::Black)),
        6 => Some((PieceType::Pawn, PieceColor::White)),
        7 => Some((
            match file {
                0 | 7 => PieceType::Rook,
                1 | 6 => PieceType::Knight,
                2 | 5 => PieceType::Bishop,
                3 => PieceType::Queen,
                4 => PieceType::King,
                _ => return None,
            },
            PieceColor::White,
        )),
        _ => None,
    }
}

fn piece_drag_system(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut selected_piece: ResMut<SelectedPiece>,
    pieces: Query<(Entity, &Transform), With<ChessPiece>>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = window_q.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            // Find the piece closest to the click
            let closest_piece = pieces.iter()
                .min_by_key(|(_, transform)| {
                    FloatOrd(transform.translation.truncate().distance(world_position))
                });

            if let Some((entity, transform)) = closest_piece {
                // Only select if click is close enough (within 40 units)
                if transform.translation.truncate().distance(world_position) < 40.0 {
                    selected_piece.entity = Some(entity);
                    commands.entity(entity).insert(DraggedPiece);
                }
            }
        }

        // Update dragged piece position
        if let Some(entity) = selected_piece.entity {
            if let Ok((_, mut transform)) = pieces.get_component_mut::<Transform>(entity) {
                transform.translation.x = world_position.x;
                transform.translation.y = world_position.y;
            }
        }
    }
}

fn piece_release_system(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut selected_piece: ResMut<SelectedPiece>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        if let Some(entity) = selected_piece.entity.take() {
            commands.entity(entity).remove::<DraggedPiece>();
            // Here you could add logic to snap to grid or validate moves
        }
    }
} 