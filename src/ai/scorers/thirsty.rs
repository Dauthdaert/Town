use bevy::prelude::*;
use big_brain::prelude::*;

use crate::ai::behaviors::thirst::Thirst;

#[derive(Component, Clone, Copy, Debug)]
pub struct Thirsty;

pub fn thirsty_scorer(thirsts: Query<&Thirst>, mut actors: Query<(&Actor, &mut Score), With<Thirsty>>) {
    actors.par_for_each_mut(20, |(Actor(actor), mut score)| {
        if let Ok(thirst) = thirsts.get(*actor) {
            score.set(thirst.thirst / 100.0);
        }
    });
}
