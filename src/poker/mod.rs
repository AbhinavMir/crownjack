use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct PokerPlugin;

#[derive(Resource, Default)]
pub struct Deck {
    cards: Vec<Card>,
}

#[derive(Clone, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Rank {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
    Jack, Queen, King, Ace,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Suit {
    Hearts, Diamonds, Clubs, Spades,
}

impl Plugin for PokerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Deck>()
           .add_systems(Startup, setup_deck)
           .add_systems(Update, (
               shuffle_deck,
               deal_cards,
               check_hands,
           ));
    }
}

fn setup_deck(mut commands: Commands) {
    let mut deck = Vec::with_capacity(52);
    
    for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
        for rank in [
            Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six,
            Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
            Rank::Jack, Rank::Queen, Rank::King, Rank::Ace
        ] {
            deck.push(Card {
                rank: rank.clone(),
                suit: suit.clone(),
            });
        }
    }

    commands.insert_resource(Deck { cards: deck });
}

fn shuffle_deck(mut deck: ResMut<Deck>) {
    deck.cards.shuffle(&mut thread_rng());
}

fn deal_cards(
    mut _commands: Commands,
    _deck: Res<Deck>,
    // Add query for players when you have player components
) {
    // TODO: Implement dealing logic based on your game rules
}

fn check_hands(
    // Add query for players and their cards when you have those components
) {
    // TODO: Implement poker hand checking logic
}



#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PokerHand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
} 