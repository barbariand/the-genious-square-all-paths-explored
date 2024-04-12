use crate::{
    pieceboard::PieceBoard,
    pieces::{self, Pieces},
};
use bevy::{
    a11y::accesskit::{NodeBuilder, NodeClassSet, Role},
    prelude::*,
};
pub fn main(board: Vec<PieceBoard>) {
    App::new()
        .insert_resource(PieceBoardEnums::from(board))
        .add_plugins(PieceBoardPlugin)
        .run();
}
pub struct PieceBoardPlugin;

impl Plugin for PieceBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Resource)]
struct PieceBoardEnums {
    vec: Vec<[[Option<Pieces>; 6]; 6]>,
}
impl From<Vec<PieceBoard>> for PieceBoardEnums {
    fn from(vec: Vec<PieceBoard>) -> Self {
        let iter = vec.iter().map(|v| {
            let b = v.pieces();

            let mut arr = (0..6).into_iter().map(|i| {
                (0..6).into_iter().map(move |j| {
                    j;
                    match (
                        b[1].get_bit((i * 6 + j) as u64),
                        b[2].get_bit((i * 6 + j) as u64),
                        b[3].get_bit((i * 6 + j) as u64),
                        b[4].get_bit((i * 6 + j) as u64),
                        b[0].get_bit((i * 6 + j) as u64),
                        b[5].get_bit((i * 6 + j) as u64),
                        b[6].get_bit((i * 6 + j) as u64),
                        b[7].get_bit((i * 6 + j) as u64),
                        b[8].get_bit((i * 6 + j) as u64),
                    ) {
                        (true, false, false, false, false, false, false, false, false) => {
                            Some(pieces::Pieces::OneByFour)
                        }
                        (false, true, false, false, false, false, false, false, false) => {
                            Some(pieces::Pieces::Shape9)
                        }
                        (false, false, true, false, false, false, false, false, false) => {
                            Some(pieces::Pieces::Shape8)
                        }
                        (false, false, false, true, false, false, false, false, false) => {
                            Some(pieces::Pieces::Shape7)
                        }
                        (false, false, false, false, true, false, false, false, false) => {
                            Some(pieces::Pieces::Shape6)
                        }
                        (false, false, false, false, false, true, false, false, false) => {
                            Some(pieces::Pieces::TwoByTwo)
                        }
                        (false, false, false, false, false, false, true, false, false) => {
                            Some(pieces::Pieces::OneByThree)
                        }
                        (false, false, false, false, false, false, false, true, false) => {
                            Some(pieces::Pieces::OneByTwo)
                        }
                        (false, false, false, false, false, false, false, false, true) => {
                            Some(pieces::Pieces::OneByOne)
                        }
                        (false, false, false, false, false, false, false, false, false) => None,

                        _ => {
                            unreachable!("we have a colition")
                        }
                    }
                })
            });

            [
                match arr.next() {
                    Some(mut v) => [
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                    ],
                    None => [None; 6],
                },
                match arr.next() {
                    Some(mut v) => [
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                    ],
                    None => [None; 6],
                },
                match arr.next() {
                    Some(mut v) => [
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                    ],
                    None => [None; 6],
                },
                match arr.next() {
                    Some(mut v) => [
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                    ],
                    None => [None; 6],
                },
                match arr.next() {
                    Some(mut v) => [
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                    ],
                    None => [None; 6],
                },
                match arr.next() {
                    Some(mut v) => [
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                        v.next().flatten(),
                    ],
                    None => [None; 6],
                },
            ]
        });
        Self {
            vec: iter.collect(),
        }
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
                                    green: 0.0,
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
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((NodeBundle {
                ..Default::default()
            },));
        });
}
