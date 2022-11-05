use bevy::prelude::*;
use crate::{Player,Transform,GameTextures,GUN_X_OFFSET,PLAYER_SPAWN_POS,SHOTGUN_SCALE,WinSize,ENEMY_SIZE,ENEMY_SCALE,BULLET_SIZE,};
use crate::components::*;
use bevy::sprite::collide_aabb::collide;

pub struct GunPlugin;

const GUN_ROT_OFFSET:f32 = 90.;
const BULLET_X_OFFSET:f32 = 15.;

impl Plugin for GunPlugin{
fn build(&self,app:&mut App){
        app
            .add_startup_system_to_stage(StartupStage::PostStartup,gun_spawn_system)
            .add_system(update_gun_loc_system)
            .add_system(spawn_bullet_system)
            .add_system(update_bullet_system)
            .add_system(update_gun_rot_system)
            .add_system(bullet_damage_system)
            .add_system(manage_gunshot_cooldown_system); 
    }
 }

fn gun_spawn_system(mut commands:Commands,game_textures:Res<GameTextures>){
  //spawn gun {{{
    commands.spawn_bundle(SpriteBundle{
       texture: game_textures.shotgun.clone(),
       transform: Transform{
           translation: Vec3::new(PLAYER_SPAWN_POS.0+GUN_X_OFFSET,PLAYER_SPAWN_POS.1,2.),
           scale: Vec3::new(SHOTGUN_SCALE,SHOTGUN_SCALE,1.),
           ..Default::default()
       },
       ..Default::default()
    })
    .insert(Gun{rotates:true,spread:360.,bullet_count:10.,damage:10.,firerate:1.,bullet_speed:0.1,})
    .insert(Cooldown{elapsed:0.,duration:60.,available:true});
    //}}} // alright. this needs some sort of system with an if where we can insert various
    //different guns. 
}
fn update_gun_loc_system(query: Query<(&Transform, With<Player>),Without<Gun>>,mut gunquery: Query <(&mut Transform, With<Gun>), Without<Player>>){
    //update gun location{{{
    // holy shit this isnt actual ass
    // idk how this isnt spaghetti
    if let Ok(player_tf) = query.get_single(){
    if let Ok(mut gun_tf) = gunquery.get_single_mut(){
        gun_tf.0.translation.x=player_tf.0.translation.x + GUN_X_OFFSET;
        gun_tf.0.translation.y=player_tf.0.translation.y;
    }}//}}}
}

fn update_gun_rot_system(wnds: Res<Windows>,q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,player_query: Query<(&Transform, With<Player>),Without<Gun>>,mut gun_query: Query<(&mut Transform, With<Gun>),Without<Player>>) {
    //update gun rotation {{{
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();
    let player_tf = player_query.single().0;
    let mut gun_tf = gun_query.single_mut();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let bevy::render::camera::RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    // check if the cursor is inside the window and get its position
     if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        let (world_x,world_y) = (world_pos.truncate().x,world_pos.truncate().y);
        let diff = (world_x-(player_tf.translation.x+GUN_X_OFFSET),world_y-player_tf.translation.y);

        //alright. now we calculate where we need to rotate the gun for it to point at the mouse
        let mouse_angle = diff.0.atan2(diff.1) as f32 * 180. /3.1415;
        gun_tf.0.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), -(mouse_angle-GUN_ROT_OFFSET).to_radians());

    }
    //}}}
}

fn spawn_bullet_system(mut commands: Commands,game_textures:Res<GameTextures>,query: Query<(&Transform, &Gun)>,kb:Res<Input<KeyCode>>,mut cooldown_query: Query<&mut Cooldown, With<Gun>>){
    //spawn bullet system {{{
    let mut cooldown = cooldown_query.single_mut();
    if kb.pressed(KeyCode::Space) && cooldown.available{
    cooldown.available  = false;
    let gun_tf = query.single().0;
    let gun_stats = query.single().1;
    let bullet_dir: Vec3;
    if gun_stats.bullet_count == 1.{
        let bullet_dir = gun_tf.rotation*Vec3::new(50.,1.,0.);
        commands.spawn_bundle(SpriteBundle{
            texture: game_textures.bullet.clone(),
            transform: Transform{
                translation: Vec3::new(gun_tf.translation.x,gun_tf.translation.y,gun_tf.translation.z),
                ..Default::default()
            },
            ..Default::default() 
        }).insert(Bullet{direction:bullet_dir}).insert(Movable{auto_despawn:true,friction:false}).insert(Velocity{x:0.,y:0.});
    }
    else{
    // calculating the degree amount between the various bullets. I hate radians
    let increment = gun_stats.spread / gun_stats.bullet_count * 10.;
    //loop over and place bullets over said increments
    for i in 0..gun_stats.bullet_count as i32{
        //TODO: Minus half of the spread or minus the spread for the first half to make the spread
        //even
        let bullet_dir = gun_tf.rotation*Vec3::new(50.,((increment*i as f32)-gun_stats.spread*5.).to_radians(),0.);
        commands.spawn_bundle(SpriteBundle{
            texture: game_textures.bullet.clone(),
            transform: Transform{
                translation: Vec3::new(gun_tf.translation.x,gun_tf.translation.y,gun_tf.translation.z),
                ..Default::default()
            },
            ..Default::default() 
        }).insert(Bullet{direction:bullet_dir}).insert(Movable{auto_despawn:true,friction:false}).insert(Velocity{x:0.,y:0.});
         
    }
    }
    //}}}
}}
fn update_bullet_system(query: Query<(&Transform,&Gun),Without<Bullet>>,mut bullet_query: Query<(&mut Transform, &Bullet), With<Bullet>>){
    //update bullet{{{
    let gun_stats = query.single().1;
    // do something with spread and stuff
    for  (mut bullet_tf,bullet) in bullet_query.iter_mut(){
    bullet_tf.translation += gun_stats.bullet_speed*bullet.direction;
    //TODO:  make the gunstats actually do someth, add more guns - store in a bevy resource hashmap

    //}}}
}}

fn manage_gunshot_cooldown_system(mut query: Query<&mut Cooldown, With<Gun>>){
    //gun cooldown system {{{
    //makes the gun not shoot a bullet every frame
    let mut cooldown = query.single_mut();
    if cooldown.elapsed >=cooldown.duration{
        cooldown.elapsed = 0.;
        cooldown.available = true;
    }
    else if cooldown.available == false{
        cooldown.elapsed += 1.;
    }
    //}}}
}

fn bullet_damage_system(mut commands: Commands, mut query: Query<(&Transform, Entity), With<Enemy>>, bullet_query: Query<(&Transform,Entity), With<Bullet>>){
    for (enemy_tf, enemy_entity) in query.iter_mut(){  //unoptimized. Checks all the bullets for all the enemies, giving it O(n^2).
        for (bullet_tf,bullet_entity) in bullet_query.iter(){
            let bullet_size = Vec2::new(bullet_tf.scale.x,bullet_tf.scale.y);
            let enemy_size = Vec2::new(enemy_tf.scale.x,enemy_tf.scale.y);
            let collision = collide(bullet_tf.translation,bullet_size,enemy_tf.translation,enemy_size);
            if let Some(_) = collision{
                commands.entity(enemy_entity).despawn();
                commands.entity(bullet_entity).despawn();
            }

        }
    }
}
