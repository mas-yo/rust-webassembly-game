use crate::components::*;
use crate::systems::*;

impl SystemProcess for System<CContainer<MoveIntent>, CContainer<Velocity>> {
    fn process(intents: &Self::Input, velocities: &mut Self::Output) {
        velocities.iter_mut().for_each(|v| {
            if let Some(i) = intents.get_by_entity_id(v.entity_id()) {
                v.0 = i.0 * 2f64;
                v.1 = i.1 * 2f64;
            }
        });
    }
}
