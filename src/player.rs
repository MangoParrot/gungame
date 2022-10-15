use bevy::prelude::*;
use crate::{GameTextures,WinSize,PLAYER_SCALE,Velocity,Player,Movable,PLAYER_SPAWN_POS};


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self,app: &mut App){
        app
            .add_startup_system_to_stage(StartupStage::PostStartup,player_spawn_system)
            .add_system(player_movement_system);
    }
}

fn player_spawn_system(mut commands:Commands,game_textures: Res<GameTextures>,win_size: Res<WinSize>){
    //Player Spawn System{{{
    commands.spawn_bundle(SpriteBundle {
        texture: game_textures.player.clone(),
        transform: Transform{
            translation: Vec3::new(PLAYER_SPAWN_POS.0,PLAYER_SPAWN_POS.1,1.),
            scale: Vec3::new(PLAYER_SCALE,PLAYER_SCALE,1.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Velocity{x:0.,y:0.})
    .insert(Player)
    .insert(Movable{auto_despawn:false,friction:true});
    //}}}
}

fn player_movement_system(kb: Res<Input<KeyCode>>,mut query: Query<&mut Velocity,With<Player>>){
    if let Ok(mut velocity) = query.get_single_mut(){
        if kb.pressed(KeyCode::A){
            velocity.x = -1.;
        }
        if kb.pressed(KeyCode::D){
            velocity.x = 1.;
        }
        if kb.pressed(KeyCode::W){
            velocity.y = 1.;
        }
        if kb.pressed(KeyCode::S){
            velocity.y = -1.;
        }
    }
}
