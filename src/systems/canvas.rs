use crate::components::*;
use crate::systems::*;
use crate::*;

impl SystemProcess for System<CContainer<Sprite>, CanvasContext> {
    fn process(sprites: &Self::Input, context: &mut Self::Output) {
        context.begin_path();
        sprites.iter().for_each(|s| {
            context
                .arc(
                    s.position.0,
                    s.position.1,
                    10.0,
                    0.0,
                    std::f64::consts::PI * 2.0,
                )
                .unwrap();
        });
        context.stroke();
    }
}
