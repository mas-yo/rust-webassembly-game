use crate::components::*;
use crate::systems::*;

impl SystemProcess for System<CContainer<Position>, CContainer<Sprite>> {
    fn process(positions: &Self::Input, sprites: &mut Self::Output) {
        sprites.iter_mut().for_each(|s| {
            if let Some(p) = positions.get_by_entity_id(s.entity_id()) {
                s.position.0 = p.0;
                s.position.1 = p.1;
            }
        });
    }
}
