use std::time::Duration;

use bevy::prelude::*;

use crate::game::components::{BoxType, CountDownTimer};

use super::{events::OnFactoryProduced, recipes::Recipes, OnDropInFactoryInput};

#[derive(Default, Debug)]
pub struct Factory {
    pub inputs: [Option<BoxType>; 2],
    output_queue: Vec<BoxType>,
    timer: Option<CountDownTimer>,
}

impl Factory {
    pub fn drop(&mut self, box_type: BoxType) {
        if self.inputs[0].is_none() {
            self.inputs[0] = Some(box_type);
        } else {
            self.inputs[1] = Some(box_type);
        }
    }

    /// Update the timers and return a box type if something has been produced
    pub fn update(&mut self, delta: Duration) -> Option<BoxType> {
        if self.timer.is_none() {
            return None;
        }

        let timer = self.timer.as_mut().unwrap();
        timer.0.tick(delta);

        if !timer.0.just_finished() {
            return None;
        }

        let built = self.output_queue.remove(0);
        info!("Finished production of {:?}", built);

        if self.output_queue.is_empty() {
            self.timer = None;
        } else {
            info!("Production of {:?} started", self.output_queue[0]);
            self.timer = self.new_timer();
        }

        Some(built)
    }

    /// Checks if there are enough items in there to start production
    pub fn check_production_inputs_and_start_building(&mut self, recipes: &Recipes) {
        if !self.is_complete() {
            return;
        }

        let bt = if let Some(bt) = recipes.get_output(&self.inputs) {
            bt
        } else {
            info!("No recipe available yet, factory contains {:?}", self);
            self.reset();
            return;
        };

        info!(
            "Factory receive two inputs {:?} and {:?} which has a recipe of {:?}",
            self.inputs[0], self.inputs[1], bt
        );

        self.output_queue.push(bt);

        if self.timer.is_none() {
            info!("Started production of {:?}", bt);
            self.timer = self.new_timer();
        } else {
            info!("Queued {:?}", bt);
        }

        self.reset();
    }

    fn new_timer(&self) -> Option<CountDownTimer> {
        Some(CountDownTimer(Timer::new(
            Duration::from_secs_f32(5.0),
            false,
        )))
    }

    fn reset(&mut self) {
        self.inputs[0] = None;
        self.inputs[1] = None;
    }

    fn is_complete(&self) -> bool {
        self.inputs[0].is_some() && self.inputs[1].is_some()
    }

    /// Gets the current output being produced
    pub fn get_current_output(&self) -> Option<BoxType> {
        match self.output_queue.get(0) {
            Some(bt) => Some(*bt),
            None => None,
        }
    }
}

pub fn add_item_to_factory(
    recipes: Res<Recipes>,
    mut factory: ResMut<Factory>,
    mut drop_events: EventReader<OnDropInFactoryInput>,
) {
    for drop_event in drop_events.iter() {
        factory.drop(drop_event.box_type);
        factory.check_production_inputs_and_start_building(&recipes);
    }
}

pub fn update_factory(
    time: Res<Time>,
    mut factory: ResMut<Factory>,
    mut produced_events: EventWriter<OnFactoryProduced>,
) {
    match factory.update(time.delta()) {
        Some(produced) => {
            produced_events.send(OnFactoryProduced { box_type: produced });
        }
        _ => {}
    }
}
