use crate::components::*;
use crate::systems::*;

impl SystemProcess for System<PlayerInput, MoveIntent> {
    fn process(input: &Self::Input, intent: &mut Self::Output) {
        let mut d = (0f64, 0f64);
        if input.up_arrow {
            d.1 = -1f64;
        }
        if input.down_arrow {
            d.1 = 1f64;
        }
        if input.left_arrow {
            d.0 = -1f64;
        }
        if input.right_arrow {
            d.0 = 1f64;
        }
        intent.0 = d.0;
        intent.1 = d.1;
    }
}
