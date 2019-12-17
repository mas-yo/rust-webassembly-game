use crate::components::*;
use crate::systems::*;

impl SystemProcess for System<(&CContainer<Collider>, &CContainer<Velocity>), CContainer<Position>> {
    fn process(col_vel: &Self::Input, positions: &mut Self::Output) {
        let (_, vel) = col_vel; //TODO process collider
        for p in positions.iter_mut() {
            if let Some(v) = vel.get_by_entity_id(p.entity_id()) {
                p.0 += v.0;
                p.1 += v.1;
            }
        }
    }
}
