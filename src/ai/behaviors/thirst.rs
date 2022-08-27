use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct Thirst {
    //Rate of thirst accumulation
    pub per_second: f32,
    //Current drink progress
    pub drink_progress: f32,
    //Current thirst
    pub thirst: f32,
}

impl Thirst {
    pub fn new(thirst: f32, per_second: f32) -> Self {
        Self {
            per_second,
            thirst,
            drink_progress: 0.0,
        }
    }
}

pub fn handle_thirst(time: Res<Time>, mut thirsts: Query<&mut Thirst>) {
    thirsts.par_for_each_mut(20, |mut thirst| {
        thirst.thirst += thirst.per_second * time.delta_seconds();

        //Clamp to 100.0
        thirst.thirst = f32::min(thirst.thirst, 100.0);
    });
}
