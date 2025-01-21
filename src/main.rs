use bevy::prelude::*;
mod card_assets;
mod poker;
mod ui;
use card_assets::CardAssetsPlugin;
use poker::PokerPlugin;
use ui::GameUiPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CardAssetsPlugin,
            PokerPlugin,
            GameUiPlugin,
        ))
        .run();
}

