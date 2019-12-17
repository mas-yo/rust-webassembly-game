use crate::EntityID;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::slice::IterMut;

mod collider;
mod enemy_ai;
mod move_intent;
mod player_input;
mod position;
mod sprite;
mod velocity;

pub(crate) use collider::*;
pub(crate) use enemy_ai::*;
pub(crate) use move_intent::*;
pub(crate) use player_input::*;
pub(crate) use position::*;
pub(crate) use sprite::*;
pub(crate) use velocity::*;

pub(crate) type CContainer<T> = ComponentContainer<T>;

pub(crate) struct Component<T> {
    entity_id: EntityID,
    inner: T,
}

impl<T> Component<T> {
    pub fn new(id: EntityID, inner: T) -> Self {
        Self {
            entity_id: id,
            inner: inner,
        }
    }
}

impl<T> Deref for Component<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Component<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> Component<T> {
    pub fn entity_id(&self) -> EntityID {
        self.entity_id
    }
}

pub(crate) struct ComponentContainer<T> {
    map: HashMap<EntityID, usize>,
    components: Vec<Component<T>>,
}

impl<T> ComponentContainer<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            components: Vec::new(),
        }
    }
    pub fn push(&mut self, com: Component<T>) {
        self.map.insert(com.entity_id(), self.components.len());
        self.components.push(com);

    }
    pub fn get_by_entity_id(&self, entity_id: EntityID) -> Option<&Component<T>> {
        let find = self.map.get(&entity_id)?;
        self.components.get(*find)
    }
    pub fn get_mut_by_entity_id(&mut self, entity_id: EntityID) -> Option<&mut Component<T>> {
        let find = self.map.get(&entity_id)?;
        self.components.get_mut(*find)
    }

    pub fn iter_mut(&mut self) -> IterMut<Component<T>> {
        self.components.iter_mut()
    }
}

impl<T> Deref for ComponentContainer<T> {
    type Target = Vec<Component<T>>;
    fn deref(&self) -> &Self::Target {
        &self.components
    }
}

// impl<T> DerefMut for ComponentContainer<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.components
//     }
// }
