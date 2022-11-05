use bevy::prelude::*;
//Game Components {{{

#[derive(Component)]
// literally tagged on to the maincam to make the gun mouse logic ez
pub struct MainCamera;

#[derive(Component)]
pub struct Velocity{
    pub x: f32,
    pub y: f32
}

#[derive(Component)]
pub struct Movable{
    pub auto_despawn:bool,
    pub friction:bool,
}

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32,f32)> for SpriteSize{
     fn from(val: (f32,f32))-> Self{
        SpriteSize(Vec2::new(val.0,val.1))
     }
}

#[derive(Component)]
pub struct Enemy;
//}}}

//Player Components{{{

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Gun{
    pub rotates: bool,
    pub spread: f32,
    pub bullet_count: f32,
    pub damage: f32,
    pub firerate: f32, 
    pub bullet_speed: f32,
}

#[derive(Component)]
pub struct Bullet{
    pub direction: Vec3,
}
//}}}

//Game Mechanics {{{

#[derive(Component)]
pub struct Cooldown{
    pub elapsed:f32, //frames and stuff since the last use
    pub duration:f32, // in frames. need to make sure on slower computers it still works
    pub available:bool,
}

//}}}
