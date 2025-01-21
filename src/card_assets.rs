use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct CardAssets {
    pub cards: HashMap<String, Handle<Image>>,
}

pub struct CardAssetsPlugin;

impl Plugin for CardAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CardAssets>()
           .add_systems(Startup, load_card_assets);
    }
}

fn load_card_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut cards = HashMap::new();
    
    // Regular cards
    let ranks = ["2", "3", "4", "5"];
    let suits = ["hearts", "diamonds", "spades"]; // No clubs except for Ace
    let styles = ["colored", "outline"];
    
    // Load standard cards (2-5 for hearts, diamonds, spades)
    for rank in ranks {
        for suit in suits {
            for style in styles {
                let filename = format!("cards/{rank}_{suit}_{style}.png");
                cards.insert(
                    filename.clone(),
                    asset_server.load(&filename)
                );
            }
        }
    }
    
    // Load Ace cards (including clubs)
    for suit in ["hearts", "diamonds", "spades", "clubs"] {
        for style in styles {
            let filename = format!("cards/A_{suit}_{style}.png");
            cards.insert(
                filename.clone(),
                asset_server.load(&filename)
            );
        }
    }
    
    // Load special cards (only the ones that exist)
    let special_combinations = [
        // Based on your ls output, these are the ones you have
        (2, 8), (2, 9), (2, 10), (2, 11), (2, 12), (2, 13),
        (3, 8), (3, 9), (3, 10), (3, 11), (3, 12), (3, 13),
        (4, 8), (4, 9), (4, 10), (4, 11), (4, 12), (4, 13),
    ];

    for (i, j) in special_combinations {
        let filename = format!("cards/special_{i}_{j}.png");
        cards.insert(
            filename.clone(),
            asset_server.load(&filename)
        );
    }

    commands.insert_resource(CardAssets { cards });
} 