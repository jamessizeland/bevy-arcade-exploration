mod collisions;
mod enemy;
mod events;
mod movement;
mod particles;
mod player;
mod resources;
mod state;
mod views;
mod walls;

use self::{
    collisions::CollisionsPlugin,
    enemy::EnemyPlugin,
    movement::MovementPlugin,
    particles::ParticlesPlugin,
    player::PlayerPlugin,
    state::{GameStatePlugin, InGameState},
    walls::WallsPlugin,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use resources::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // add plugins for modular management of game
            .add_plugins((
                GameStatePlugin,
                PlayerPlugin,
                EnemyPlugin,
                WallsPlugin,
                MovementPlugin,
                RapierPhysicsPlugin::<NoUserData>::default(),
                #[cfg(debug_assertions)]
                RapierDebugRenderPlugin::default(),
                CollisionsPlugin,
                ParticlesPlugin,
            ))
            // Initialize Game Resources
            .init_resource::<Score>()
            .init_resource::<GameTime>()
            .add_systems(
                Update,
                (update_game_time).run_if(in_state(InGameState::Play)),
            );
    }
}

fn update_game_time(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    game_time.time += time.delta_seconds();
}
