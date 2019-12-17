use std::marker::PhantomData;

mod canvas;
mod move_intent;
mod position;
mod sprite;
mod velocity;

pub(crate) trait SystemInterface {
    type Input;
    type Output;
}
pub(crate) trait SystemProcess: SystemInterface {
    fn process(input: &Self::Input, output: &mut Self::Output);
}

#[derive(Default)]
pub(crate) struct System<I, O> {
    phantom: PhantomData<(I, O)>,
}

impl<I, O> SystemInterface for System<I, O> {
    type Input = I;
    type Output = O;
}
