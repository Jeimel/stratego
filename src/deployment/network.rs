use super::random;
use crate::stratego::Position;
use ordered_float::OrderedFloat;
use tch::{
    nn::{self, Module},
    Tensor,
};

#[derive(Debug)]
pub struct Network {
    l1: nn::Linear,
    l2: nn::Linear,
    l3: nn::Linear,
    l4: nn::Linear,
    l5: nn::Linear,
}

impl Network {
    const ATTEMPTS: usize = 50;

    pub fn new(vs: &nn::Path) -> Self {
        Network {
            l1: nn::linear(vs, 24 * 7, 100, Default::default()),
            l2: nn::linear(vs, 100, 100, Default::default()),
            l3: nn::linear(vs, 100, 100, Default::default()),
            l4: nn::linear(vs, 100, 100, Default::default()),
            l5: nn::linear(vs, 100, 1, Default::default()),
        }
    }

    pub fn get(&self) -> String {
        let (deployment, _) = (0..Network::ATTEMPTS)
            .map(|_| {
                let deployment = random();
                let data = Network::tensor(&deployment);

                let mut scores = [0f32; 1];
                self.forward(&data).copy_data(&mut scores, 1);

                (deployment, scores[0])
            })
            .max_by_key(|(_, score)| OrderedFloat::from(*score))
            .unwrap();

        deployment
    }

    pub fn tensor(deployment: &str) -> Tensor {
        const SYMBOLS: [char; 7] = [
            Position::SYMBOLS[8],
            Position::SYMBOLS[9],
            Position::SYMBOLS[10],
            Position::SYMBOLS[11],
            Position::SYMBOLS[12],
            Position::SYMBOLS[13],
            Position::SYMBOLS[15],
        ];

        let mut data = [0f32; 24 * 7];

        let (mut file, mut rank) = (0, 0);
        for c in deployment.chars() {
            match c {
                c if c.is_numeric() => file += c as u32 - '0' as u32,
                '/' => (file, rank) = (0, rank + 1),
                _ => {
                    let sq = (file + rank * 8) as usize;
                    let piece = SYMBOLS.iter().position(|&symbol| symbol == c).unwrap();

                    data[piece + sq * 7] = 1.0;

                    file += 1;
                }
            }
        }

        Tensor::from_slice(&data)
    }
}

impl nn::Module for Network {
    fn forward(&self, xs: &tch::Tensor) -> tch::Tensor {
        xs.apply(&self.l1)
            .relu()
            .apply(&self.l2)
            .relu()
            .apply(&self.l3)
            .relu()
            .apply(&self.l4)
            .relu()
            .apply(&self.l5)
            .sigmoid()
    }
}
