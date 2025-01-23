use bevy::prelude::*;
mod card_assets;
mod poker;
mod ui;
use card_assets::CardAssetsPlugin;
use poker::PokerPlugin;
use ui::GameUiPlugin;
mod chess;
use chess::ChessPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            CardAssetsPlugin,
            PokerPlugin,
            GameUiPlugin,
            ChessPlugin,
        ))
        .run();
}

