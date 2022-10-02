
use std::any::{Any, TypeId, self};

use crate::common::Component;

pub struct ComponentHandler {
    components: Vec<Box<dyn Component>>,
    timer: f32,
}

impl ComponentHandler {
    pub fn new() -> Self{
        Self{
            components: vec![],
            timer: 0.0,
        }
    }

    pub fn add_component<T: Component + 'static>(&mut self, component: T)
    {
        let component = Box::new(component);
        self.components.push(component);
    }

    pub fn update(&mut self, elapsed_time: f32) {
        self.timer += elapsed_time;

        for component in self.components.iter_mut() {
            if self.timer > 0.1 {
                self.timer = 0.0;
                component.update(elapsed_time);
            }
        }
    }

    pub fn poll_inputs(&mut self, elapsed_time: f32) {
        for component in self.components.iter_mut() {
            component.poll_inputs(elapsed_time);
        }
    }


    pub fn draw(&mut self) {
        for component in self.components.iter_mut() {
            component.draw();
        }
    }
}
