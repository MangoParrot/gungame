use bevy::prelude::*;
use crate::components::*;
use crate::{WinSize,GameTextures};
use rand::prelude::*;

pub struct EnemyPlugin;


impl Plugin for EnemyPlugin{
fn build(&self,app:&mut App){
    app.add_system(enemy_spawn_system)
        .add_system(basic_enemy_ai_system);
    }
 }

fn enemy_spawn_system(mut commands: Commands,query: Query<&Transform, With<Player>>,win_size:Res<WinSize>,game_textures:Res<GameTextures>){
    let mut random = thread_rng();
    let x_span = win_size.w/2.;
    let y_span = win_size.w/2.; 
    let mut x:f32 = random.gen_range(10..50) as f32;
    let mut y:f32 = random.gen_range(10..50) as f32;
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
        texture: game_textures.shotgun.clone(),
        transform: Transform{
        translation: Vec3::new(0.,0.,0.), 
        ..Default::default()
        },
        ..Default::default()
    }).insert(Movable{auto_despawn:false,friction:true})
    .insert(Velocity{x:1.,y:1.})
    .insert(Enemy);
}

fn basic_enemy_ai_system(mut query:Query<&mut Transform, With<Enemy>>){
    if let Ok(mut enemy_tf) = query.get_single_mut(){
    enemy_tf.translation.x += 50.;
    enemy_tf.translation.y += 50.;
    }
    println!("moving rn");
}
