use super::{BitMap64, PieceBoard};
use bevy::prelude::*;

pub struct PieceBoardPlugin;

impl Plugin for PieceBoardPlugin {
    fn build(&self, app: &mut App) {
        // Add a global slider component to the app
        app.add_systems(Startup, setup);
    }
}
#[derive(Component)]
struct GridPositions {
    x: usize,
    y: usize,
}
fn setup(mut commands: Commands, window_query: Query<&Window>) {
    commands.spawn(Camera2dBundle::default());
    let window = window_query.single();
    commands.spawn_batch(
        (0..6)
            .into_iter()
            .map(|x| {
                let xclone = x.clone();
                (0..6).into_iter().map(move |y| {
                    (
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::Rgba {
                                    red: 1.0,
                                    green: 1.0,
                                    blue: 1.0,
                                    alpha: 1.0,
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        GridPositions {
                            x: xclone.clone(),
                            y: y.clone(),
                        },
                    )
                })
            })
            .flatten(),
    );
}
