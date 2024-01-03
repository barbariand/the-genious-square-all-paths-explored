use super::{pieces::Pieces, BitMap64, PieceBoard};
use bevy::prelude::*;

pub struct PieceBoardPlugin;

impl Plugin for PieceBoardPlugin {
    fn build(&self, app: &mut App) {
        // Add a global slider component to the app
        app.add_systems(Startup, setup);
    }
}
struct PieceBoardEnum {
    vec: [[Option<Pieces>; 6]; 6],
}
/* impl From<PieceBoard> for PieceBoardEnum {
    fn from(v: PieceBoard) -> Self {
        let b = v.pieces;
        let a = [[None; 6]; 6];
        for (i, row) in a.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                match (
                    b[0].get_bit(i * 6 + j),
                    b[1].get_bit(i * 6 + j),
                    b[2].get_bit(i * 6 + j),
                    b[3].get_bit(i * 6 + j),
                    b[4].get_bit(i * 6 + j),
                    b[5].get_bit(i * 6 + j),
                    b[6].get_bit(i * 6 + j),
                    b[7].get_bit(i * 6 + j),
                    b[8].get_bit(i * 6 + j),

                ) {
                    (true, false, false, false, false, false, false, false, false) => {}
                    (false, true, false, false, false, false, false, false, false) => {}
                    (false, false, true, false, false, false, false, false, false) => {}
                    (false, false, false, true, false, false, false, false, false) => {}
                    (false, false, false, false, true, false, false, false, false) => {}
                    (false, false, false, false, false, true, false, false, false) => {}
                    (false, false, false, false, false, false, true, false, false) => {}
                    (false, false, false, false, false, false, false, true, false) => {}
                    (false, false, false, false, false, false, false, false, true) => {}
                    (false, false, false, false, false, false, false, false, false) => {}

                    _ => {
                        unreachable!()
                    }
                }
            }
        }
    }
} */
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
