use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::components::*;
use crate::systems::*;
use crate::*;

pub(crate) struct World {
    window_env: WindowEnv,

    entity_id_generator: IdGenerator<EntityID>,
    hero_entity_id: EntityID,

    player_input: Rc<RefCell<PlayerInput>>,
    move_intents: CContainer<MoveIntent>,
    enemy_ais: CContainer<EnemyAI>,
    colliders: CContainer<Collider>,
    velocities: CContainer<Velocity>,
    positions: CContainer<Position>,
    sprites: CContainer<Sprite>,
    // move_position: MovePositionSystem,
    // draw: System<CanvasContext, CContainer<Sprite>, ()>,
}

impl World {
    pub fn new(env: WindowEnv) -> Self {
        let input = Rc::new(RefCell::new(PlayerInput::default()));
        {
            let input = input.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                let mut i = input.borrow_mut();
                *i = PlayerInput::default();
                if event.key() == "a" {
                    i.left_arrow = true;
                }
                if event.key() == "d" {
                    i.right_arrow = true;
                }
                if event.key() == "w" {
                    i.up_arrow = true;
                }
                if event.key() == "s" {
                    i.down_arrow = true;
                }
            }) as Box<dyn FnMut(_)>);
            env.window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
            closure.forget();
        }
        {
            let input = input.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                let mut i = input.borrow_mut();
                *i = PlayerInput::default();
                if event.key() == "a" {
                    i.left_arrow = false;
                }
                if event.key() == "d" {
                    i.right_arrow = false;
                }
                if event.key() == "w" {
                    i.up_arrow = false;
                }
                if event.key() == "s" {
                    i.down_arrow = false;
                }
            }) as Box<dyn FnMut(_)>);
            env.window.add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref());
            closure.forget();
        }

        let mut w = Self {
            window_env: env, 

            entity_id_generator: IdGenerator::<EntityID>::new(),
            hero_entity_id: 0,

            player_input: input,
            move_intents: CContainer::new(),
            enemy_ais: CContainer::new(),
            colliders: CContainer::new(),
            velocities: CContainer::new(),
            positions: CContainer::new(),
            sprites: CContainer::new(),
        };
        w.create_player((50f64, 50f64));
        w
    }
    fn create_player(&mut self, pos: (f64, f64)) {
        let entity_id = self.entity_id_generator.next();
        self.hero_entity_id = entity_id;
        self.move_intents.push(Component::<MoveIntent>::new(
            entity_id,
            MoveIntent(0f64, 0f64),
        ));
        self.colliders.push(Component::<Collider>::new(
            entity_id,
            Collider { radius: 10.0 },
        ));
        self.velocities
            .push(Component::<Velocity>::new(entity_id, Velocity(0f64, 0f64)));
        self.positions.push(Component::<Position>::new(
            entity_id,
            Position(pos.0, pos.1),
        ));
        self.sprites.push(Component::<Sprite>::new(
            entity_id,
            Sprite {
                position: (pos.0, pos.1),
            },
        ));
    }
    pub fn update(&mut self) {
        // self.move_position.process(&self.colliders, &self.velocities, &mut self.positions);
        // ViewSystem::process(&self.positions, &mut self.sprites);
        {
            let input = self.player_input.borrow_mut();
            let intent = self
                .move_intents
                .get_mut_by_entity_id(self.hero_entity_id)
                .unwrap();
            System::<PlayerInput, MoveIntent>::process(&input, intent);
        }
        System::<CContainer<MoveIntent>, CContainer<Velocity>>::process(
            &self.move_intents,
            &mut self.velocities,
        );
        System::<(&CContainer<Collider>, &CContainer<Velocity>), CContainer<Position>>::process(
            &(&self.colliders, &self.velocities),
            &mut self.positions,
        );
        System::<CContainer<Position>, CContainer<Sprite>>::process(
            &self.positions,
            &mut self.sprites,
        );
        System::<CContainer<Sprite>, CanvasContext>::process(
            &self.sprites,
            &mut self.window_env.canvas_context,
        );
        // self.draw.process(&self.sprites);
    }
}
