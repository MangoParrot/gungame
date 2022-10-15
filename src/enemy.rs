use bevy::prelude::*;
use crate::components::*;
use crate::{WinSize,GameTextures};
use rand::prelude::*;

pub struct EnemyPlugin;


impl Plugin for EnemyPlugin{
fn build(&self,app:&mut App){
    }
 }

fn enemy_spawn_system(mut commands: Commands,query: Query<&Transform, With<Player>>,win_size:Res<WinSize>,game_textures:Res<GameTextures>){
    let mut random = thread_rng();
    let x_span = win_size.w/2.;
    let y_span = win_size.w/2.; 
    let mut x:f32 = random.gen_range(10..100) as f32;
    let mut y:f32 = random.gen_range(10..100) as f32;
    let hemisphere = random.gen_range(0..4);
    
    if hemisphere == 0{
        x = x_span +x;
        y = y_span +y; 
    } //going clockwise

    if hemisphere == 1{
        x = x_span +x;
        y = -y_span -y; 
    }
    
    if hemisphere == 2{
        x = -x_span -x;
        y = -y_span -y; 
    }

    if hemisphere == 3{
        x = -x_span -x;
        y = y_span +y; 
    }
    
    
    commands.spawn_bundle(SpriteBundle{
        texture: game_textures.player.clone(),
        transform: Transform{
        translation: Vec3::new(x,y,0.), 
        ..Default::default()
        },
        ..Default::default()
    });

}

//fn basic_enemy_ai_system(query:<>){


//}
