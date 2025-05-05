use crate::stratego::StrategoState;
use tch::{
    nn::{self},
    Tensor,
};

#[derive(Debug)]
pub struct Network {
    l1_1: nn::Linear,
    l1_2: nn::Linear,
    l2: nn::Linear,
    l3: nn::Linear,
}

unsafe impl Send for Network {}

unsafe impl Sync for Network {}

impl Network {
    pub fn new(vs: &nn::Path) -> Self {
        Network {
            l1_1: nn::linear(vs, StrategoState::FEATURES as i64, 256, Default::default()),
            l1_2: nn::linear(vs, StrategoState::FEATURES as i64, 256, Default::default()),
            l2: nn::linear(vs, 512, 32, Default::default()),
            l3: nn::linear(vs, 32, 1, Default::default()),
        }
    }

    pub fn get(&self, pos: &mut StrategoState) -> f32 {
        let red = Tensor::from_slice(&pos.features::<0>());
        let blue = Tensor::from_slice(&pos.features::<1>());

        let (us, them) = if pos.stm() { (blue, red) } else { (red, blue) };

        self.forward(&us, &them).double_value(&[]) as f32
    }

    pub fn forward(&self, us: &Tensor, them: &Tensor) -> Tensor {
        Tensor::cat(&[us.apply(&self.l1_1), them.apply(&self.l1_2)], 0)
            .clamp(0.0, 1.0)
            .square()
            .apply(&self.l2)
            .clamp(0.0, 1.0)
            .square()
            .apply(&self.l3)
            .tanh()
    }

    pub fn forward_batch(&self, us: &Tensor, them: &Tensor) -> Tensor {
        Tensor::cat(&[us.apply(&self.l1_1), them.apply(&self.l1_2)], 1)
            .clamp(0.0, 1.0)
            .square()
            .apply(&self.l2)
            .clamp(0.0, 1.0)
            .square()
            .apply(&self.l3)
            .tanh()
    }
}
