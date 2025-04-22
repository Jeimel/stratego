use crate::buffer::{ReplayBuffer, SearchData};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use stratego::{
    deployment::Deployment,
    mcts::MCTS,
    policy::uniform,
    select::uct,
    stratego::{GameState, StrategoState},
    value::simulation_uniform,
};

pub struct DatagenThread<'a> {
    mcts: MCTS,
    buffer: Arc<Mutex<ReplayBuffer>>,
    abort: &'a AtomicBool,
}

impl<'a> DatagenThread<'a> {
    pub fn new(iterations: usize, buffer: Arc<Mutex<ReplayBuffer>>, abort: &'a AtomicBool) -> Self {
        Self {
            mcts: MCTS::new(
                iterations,
                simulation_uniform,
                uniform,
                uct,
                Some(Deployment::DATASET),
            ),
            buffer,
            abort,
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.abort.load(Ordering::Relaxed) {
                break;
            }

            self.game_loop();
        }
    }

    fn game_loop(&mut self) {
        let deployment = self.deployment();
        let mut pos = StrategoState::from(&deployment);
        let mut data = SearchData {
            pos: deployment,
            result: 0.0,
            moves: Vec::new(),
        };

        let mut ply = 0;
        while !pos.game_over() {
            ply += 1;

            let gen = pos.gen();
            if gen.len() == 0 {
                pos.set_game_state(GameState::Loss);
                break;
            }

            if ply > 150 {
                pos.set_game_state(GameState::Draw);
                break;
            }

            let mov = self.mcts.go(&pos);
            let mov = gen.iter().find(|m| format!("{}", m) == format!("{}", mov));
            if mov.is_none() {
                unreachable!();
            }

            let mov = mov.unwrap();
            pos.make(mov);

            let root = self.mcts.root();

            let mut policy = Vec::new();
            for child in root.children() {
                let mov = child.mov().unwrap();
                policy.push((mov, child.stats().visits));
            }

            let best = root.max_visits().unwrap();
            data.push(
                mov,
                best.stats().reward / best.stats().visits as f32,
                policy,
            );
        }

        let stm = pos.stm() as usize as f32;
        data.result = match pos.game_state() {
            GameState::Ongoing => unreachable!(),
            GameState::Win => -1.0 + 2.0 * stm,
            GameState::Draw => 0.0,
            GameState::Loss => 1.0 - 2.0 * stm,
        };

        let mut buffer = self.buffer.lock().unwrap();
        buffer.push(&data, self.abort);
    }

    fn deployment(&mut self) -> String {
        let red = Deployment::RANDOM
            .get()
            .to_ascii_uppercase()
            .split('/')
            .rev()
            .collect::<Vec<_>>()
            .join("/");
        let blue = Deployment::RANDOM.get().to_ascii_lowercase();

        format!("{}/8/8/{} r", blue, red)
    }
}
