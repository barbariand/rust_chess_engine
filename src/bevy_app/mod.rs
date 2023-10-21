use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_sprite_animation::prelude::*;
mod chess_engine_glue;
pub fn run() {
    let mut app = App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((game::GamePlugin))
        .run();
}
mod game {
    use bevy::{prelude::*, transform};

    use super::chess_engine_glue::prelude::*;

    #[derive(Default, Debug, Hash, PartialEq, PartialOrd, Eq, Clone, States)]
    enum GameStates {
        #[default]
        WhitesTurn,
        BlacksTurn,
        GameOver,
    }
    #[derive(Default, Debug, Hash, PartialEq, PartialOrd, Eq, Clone, States)]
    enum GameWon {
        #[default]
        Playing,
        WhiteWon,
        BlackWon,
    }
    pub struct GamePlugin;
    impl Plugin for GamePlugin {
        fn build(&self, app: &mut App) {
            app.init_resource::<Board>()
                .add_state::<GameStates>()
                .add_state::<GameWon>()
                .add_systems(OnEnter(GameWon::Playing), setup_game)
                .add_systems(Update, check_click);
        }
    }

    fn check_click(game: Res<Board>) {}
    fn setup_game(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        game: Res<Board>,
        window: Query<&Window>,
    ) {
        commands.spawn(Camera2dBundle::default());
        let res = window.get_single().unwrap().resolution.clone();
        let minw = res.height().min(res.width()) / 2.0;
        let colorscheme = "colortheme1";
        let path = format!("sprites/pieces/{}", colorscheme);
        let black_bishop = asset_server.load(format!("{}/{}", path, "black/bishop.png"));
        let black_king = asset_server.load(format!("{}/{}", path, "black/king.png"));
        let black_knight = asset_server.load(format!("{}/{}", path, "black/knight.png"));
        let black_pawn = asset_server.load(format!("{}/{}", path, "black/pawn.png"));
        let black_queen = asset_server.load(format!("{}/{}", path, "black/queen.png"));
        let black_rook = asset_server.load(format!("{}/{}", path, "black/rook.png"));
        let white_bishop = asset_server.load(format!("{}/{}", path, "white/bishop.png"));
        let white_king = asset_server.load(format!("{}/{}", path, "white/king.png"));
        let white_knight = asset_server.load(format!("{}/{}", path, "white/knight.png"));
        let white_pawn = asset_server.load(format!("{}/{}", path, "white/pawn.png"));
        let white_queen = asset_server.load(format!("{}/{}", path, "white/queen.png"));
        let white_rook = asset_server.load(format!("{}/{}", path, "white/rook.png"));
        let mut i: f32 = 0.0;
        println!("window size {},{}", minw, minw);
        let it: Vec<SpriteBundle> = game
            .into_iter()
            .flat_map(|p| {
                if let Some(piece) = p {
                    use crate::chess_engine::pieces::Color as C;
                    use crate::chess_engine::pieces::InnerPiece as PT;
                    let mut transform = Transform::from_xyz(
                        (i % 8.0) / 8.0 * minw - minw / 2.0,
                        (i / 8.0).floor() * minw / 8.0 - minw / 2.0,
                        0.0,
                    );
                    i += 1.0;
                    transform = transform.with_scale(Vec3 {
                        x: 0.2,
                        y: 0.2,
                        z: 1.0,
                    });
                    Some(match (piece.color, piece.get_piece_type()) {
                        (C::Black, PT::Bishop) => SpriteBundle {
                            texture: black_bishop.clone(),
                            transform,
                            ..Default::default()
                        },
                        (C::Black, PT::King) => SpriteBundle {
                            texture: black_king.clone(),
                            transform,
                            ..Default::default()
                        },
                        (C::Black, PT::Knight) => SpriteBundle {
                            texture: black_knight.clone(),
                            transform,
                            ..Default::default()
                        },
                        (C::Black, PT::Pawn) => SpriteBundle {
                            texture: black_pawn.clone(),
                            transform,
                            ..Default::default()
                        },
                        (C::Black, PT::Queen) => SpriteBundle {
                            texture: black_queen.clone(),
                            transform,
                            ..Default::default()
                        },
                        (C::Black, PT::Rook) => SpriteBundle {
                            texture: black_rook.clone(),
                            transform,
                            ..Default::default()
                        },
                        (C::White, PT::Bishop) => SpriteBundle {
                            texture: white_bishop.clone(),
                            transform,
                            ..Default::default()
                        },
                        (C::White, PT::King) => SpriteBundle {
                            texture: white_king.clone(),
                            transform,
                            ..Default::default()
                        },
                        (C::White, PT::Knight) => SpriteBundle {
                            texture: white_knight.clone(),
                            transform,
                            ..Default::default()
                        },
                        (C::White, PT::Pawn) => SpriteBundle {
                            texture: white_pawn.clone(),
                            transform,
                            ..Default::default()
                        },
                        (C::White, PT::Queen) => SpriteBundle {
                            texture: white_queen.clone(),
                            transform,
                            ..Default::default()
                        },
                        (C::White, PT::Rook) => SpriteBundle {
                            texture: white_rook.clone(),
                            transform,
                            ..Default::default()
                        },
                    })
                } else {
                    i += 1.0;
                    None
                }
            })
            .collect();
        commands.spawn_batch(it);
    }
}

fn hello_world() {
    println!("hello world!");
}
