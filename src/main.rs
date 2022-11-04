use bevy::prelude::*;
use player::*;
use components::*;
use gun::*;
use enemy::*;
use std::env;
use std::fs;

//Asset Consts{{{

const PLAYER_SPRITE: &str = "topdown_shooter/characters/3.png";
const PLAYER_SCALE: f32 = 3.;
const PLAYER_SIZE: (f32,f32) = (18.,21.);

const SHOTGUN_SPRITE: &str = "topdown_shooter/guns/shotgun/sprite.png";
const SHOTGUN_SCALE: f32 = 2.;

const BULLET_SPRITE: &str = "topdown_shooter/other/bulleta.png";

const BASIC_ENEMY_SPRITE: &str ="topdown_shooter/characters/3.png" ;
const ENEMY_SCALE: f32 = 2.;

pub struct GameTextures {
    player: Handle<Image>,
    shotgun: Handle<Image>,
    bullet: Handle<Image>,
    basic_enemy: Handle<Image>,
}

//Asset Consts}}}

//Resources{{{
pub struct WinSize{
    w:f32,
    h:f32,
}
//}}}

//Game Constants {{{


const TIME_STEP: f32 = 1./60.;
const BASE_SPEED: f32 = 100.;
const PLAYER_SPAWN_POS: (f32,f32) = (0.,0.);

//}}}

//Weapon Constants {{{

const GUN_X_OFFSET:f32 = 20.;

//}}}

mod gun;
mod player;
mod components;
mod enemy;

fn main() {
    //Main{{{
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04,0.04,0.04)))
        .insert_resource(WindowDescriptor{
            title: "Top Down Shooter".to_string(),
            width: 900.,
            height: 600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(GunPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(setup_system)
        .add_system(move_system)
        .run();
    //}}}
}
// Experimental LoadMap {{{
/*fn load_map(map: &str) -> Vec<Vec<char>>{
    let location = format!("{}{}","files/maps/",map);
    let contents = fs::read_to_string(location).expect("Failed to read file");
    let mut map: Vec<Vec<i32>> = Vec::new(); //could be optimized. doesnt need i32
    let chars = contents.chars();
    for i in 0..contents.len()-1{
        let mut curchar = match chars.nth(i){
            Some(curchar) => curchar,
            None => return Vec<Vec<char>>::new(),
        };
        curchar.push(chars.nth(i+1));
        if curchar == "\n"{
        }
    return contents
}*/
//}}}

fn setup_system(mut commands: Commands, asset_server:Res<AssetServer>,mut windows: ResMut<Windows>){
    //Setup System       ---{{{
 
    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    let game_textures = GameTextures{
        player: asset_server.load(PLAYER_SPRITE),
        shotgun: asset_server.load(SHOTGUN_SPRITE),
        bullet: asset_server.load(BULLET_SPRITE),
        basic_enemy: asset_server.load(BASIC_ENEMY_SPRITE),
    };
    commands.insert_resource(game_textures);
    
    let window= windows.get_primary_mut().unwrap();
    let win_size = WinSize{w:window.width(),h:window.height()};
    commands.insert_resource(win_size)

    //Setup System       ---}}}
}

fn move_system(mut commands: Commands, mut query: Query<(Entity,&mut Velocity,&mut Transform,&Movable)>,win_size: Res<WinSize>){
    //Move System{{{
   for (entity,mut velocity,mut transform,movable) in query.iter_mut(){
        let translation = &mut transform.translation;
        translation.x += velocity.x *TIME_STEP*BASE_SPEED;
        translation.y += velocity.y *TIME_STEP*BASE_SPEED;
            
        if movable.friction{
            velocity.x=0.;
            velocity.y=0.;
        }

        if movable.auto_despawn{
            if translation.y > win_size.h /2.  || translation.y < -win_size.h/2. || translation.x > win_size.w /2. || translation.x < -win_size.w/2. {
                commands.entity(entity).despawn();
            }
        }
   }
   //}}}
}
