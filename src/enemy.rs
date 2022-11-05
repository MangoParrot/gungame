use bevy::prelude::*;
use crate::components::*;
use crate::{WinSize,GameTextures,ENEMY_SCALE};
use rand::prelude::*;
use rust_math::trigonometry::arctan;

pub struct EnemyPlugin;

const ENEMY_SPAWN_POS: (f32,f32) = (0.,0.);
const ENEMY_SPEED:f32 = 0.01;

pub struct Wavetimer(Timer);

impl Plugin for EnemyPlugin{
fn build(&self,app:&mut App){
    app
        .insert_resource(Wavetimer(Timer::from_seconds(10.0, true)))
        .add_system(enemy_spawn_system)
        .add_system(enemy_runner_ai_system);
    }
}

//so we have some waves
//and it spawns it in accordance to the waves and the spawnpoint on the map
/*fn enemy_spawn_system(mut commands: Commands){
    let map = load_map(&"map.txt");
    println!("{:?}",map);
}*/


fn enemy_spawn_system(mut commands: Commands,time: Res<Time>, mut timer: ResMut<Wavetimer>,game_textures:Res<GameTextures>){
//enemy_spawn_system {{{
    let waves = vec![7,7,7];
    let mut current_wave = 1;
    if timer.0.tick(time.delta()).just_finished() {
            for i in 0..waves[current_wave]{
                let mut rng = rand::thread_rng(); 
                let x_offset = rng.gen_range(1..25) as f32;
                let y_offset = rng.gen_range(1..25) as f32;
                commands.spawn_bundle(SpriteBundle{
                    texture: game_textures.basic_enemy.clone(),
                    transform: Transform{
                        translation: Vec3::new(ENEMY_SPAWN_POS.0+x_offset,ENEMY_SPAWN_POS.1+y_offset,1.),
                        scale: Vec3::new(ENEMY_SCALE,ENEMY_SCALE,1.),
                        ..Default::default()
                    },
                    ..Default::default()
                }).insert(Enemy).insert(Movable{auto_despawn:false,friction:false}).insert(Velocity{x:0.,y:0.});
            }
        current_wave +=1;
        }
//}}}
}

fn enemy_runner_ai_system(query: Query<(&Transform,&Player),Without<Enemy>>,mut enemy_query: Query<(&mut Transform, &Enemy), With<Enemy>>){
    //enemy runner ai{{{
    let player_tf = query.single().0;
    for (mut enemy_tf,enemy) in enemy_query.iter_mut(){
        let diff1 = player_tf.translation.y-enemy_tf.translation.y;
        let diff2 = player_tf.translation.x-enemy_tf.translation.x;
        let target_angle = diff2.atan2(diff1) as f32 * 180. /3.1415;
        let target_angle = Quat::from_axis_angle(Vec3::new(0., 0., 1.), -(target_angle-90.).to_radians())*Vec3::new(50.,1.,0.);



//        let dir_to_player = arctan((player_tf.translation.y-enemy_tf.translation.y)/(player_tf.translation.x-player_tf.translation.x));
        let velocity = ENEMY_SPEED*target_angle;
//
        enemy_tf.translation += velocity;
    }
}
    //}}}
