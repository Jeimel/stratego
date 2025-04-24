use crate::stratego::StrategoState;
use tch::{
    nn::{self},
    Tensor,
};

#[derive(Debug)]
pub struct Network {
    l1: nn::Linear,
    l2: nn::Linear,
    l3: nn::Linear,
    l4: nn::Linear,
}

unsafe impl Send for Network {}

unsafe impl Sync for Network {}

impl Network {
    pub fn new(vs: &nn::Path) -> Self {
        Network {
            l1: nn::linear(vs, StrategoState::FEATURES as i64, 256, Default::default()),
            l2: nn::linear(vs, 256, 128, Default::default()),
            l3: nn::linear(vs, 128, 64, Default::default()),
            l4: nn::linear(vs, 64, 1, Default::default()),
        }
    }

    pub fn get(&self, pos: &mut StrategoState) -> f32 {
        let xs = Tensor::from_slice(&pos.features(pos.stm() as usize));
        self.forward(&xs).double_value(&[]) as f32
    }

    pub fn forward(&self, xs: &Tensor) -> Tensor {
        xs.apply(&self.l1)
            .relu()
            .apply(&self.l2)
            .relu()
            .apply(&self.l3)
            .relu()
            .apply(&self.l4)
            .tanh()
    }
}
