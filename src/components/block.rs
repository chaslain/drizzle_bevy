use bevy::prelude::Component;

pub const STAGE_BOTTOM: f32 = 48.;
pub const ACCELERATION_SECOND: f32 = 256. + 128.;
pub const TERMINAL_VELOCITY: f32 = 256.;



#[derive(Component)]
pub struct Block {
    pub is_bottom: bool,
    pub progress: f32,
    pub velocity: f32,
    pub floor: i32,
    pub used: bool,
    pub column: i8
}

impl Block {
    pub fn new(floor: i32, column: i8) -> Block
    {
        Block {
            is_bottom: false,
            progress: 0.,
            velocity: 0.,
            floor,
            used: false,
            column
        }
    }
}