use crate::buffer::SearchData;
use rand::{
    rng,
    seq::{IteratorRandom, SliceRandom},
    Rng,
};
use stratego::{
    deployment::Deployment,
    mcts::{Search, MCTS},
    policy::Policy,
    select::Select,
    stratego::{GameState, StrategoState},
    value::{Network, Value},
};

pub struct DatagenThread {
    mcts_sim: MCTS,
    mcts_net: MCTS,
    buffer: Vec<SearchData>,
    games: usize,
}

impl DatagenThread {
    const LIMIT: usize = 500;
    const RANDOM: usize = 10;

    pub fn new(iterations: usize, games: usize, network: Network) -> Self {
        Self {
            mcts_sim: MCTS::new(
                iterations,
                Value::SimulationUniform,
                Policy::Uniform,
                Select::UCT(1.41),
                Deployment::Heuristic(50, 0),
            ),
            mcts_net: MCTS::new(
                iterations,
                Value::Network(network),
                Policy::Uniform,
                Select::UCT(1.41),
                Deployment::Heuristic(50, 0),
            ),
            buffer: Vec::with_capacity(games),
            games,
        }
    }

    pub fn run(mut self) -> Vec<SearchData> {
        for _ in 0..self.games {
            self.game_loop();
        }

        self.buffer
    }

    fn game_loop(&mut self) {
        let deployment = self.deployment();
        let mut pos = StrategoState::from(&deployment);
        let mut data = Vec::new();

        let mut rng = rng();
        let mut ply = 0;

        while ply < DatagenThread::RANDOM && !pos.game_over() {
            ply += 1;

            let mov = pos.gen().iter().choose(&mut rng);

            if let Some(mov) = mov {
                pos.make(mov);
            } else {
                // Handle two-squares and more-squares rule
                pos.set_game_state(GameState::Loss);
            }
        }

        let index = usize::from(rng.random::<bool>());

        ply = 0;
        while !pos.game_over() {
            ply += 1;

            let gen = pos.gen();
            if gen.len() == 0 {
                pos.set_game_state(GameState::Loss);
                break;
            }

            if ply > DatagenThread::LIMIT {
                pos.set_game_state(GameState::Draw);
                break;
            }

            let red = pos.features::<0>();
            let blue = pos.features::<1>();

            let mov = if ply % 2 == index {
                self.mcts_net.go(&pos)
            } else {
                self.mcts_sim.go(&pos)
            };

            let mov = gen.iter().find(|m| format!("{}", m) == format!("{}", mov));
            if mov.is_none() {
                unreachable!();
            }

            let mov = mov.unwrap();
            pos.make(mov);

            let root = if ply % 2 == 0 {
                self.mcts_net.root()
            } else {
                self.mcts_sim.root()
            };

            let mut policy = Vec::new();
            for child in root.children() {
                let mov = child.mov().unwrap();
                policy.push((mov, child.stats().visits));
            }

            data.push(SearchData::new(
                [red, blue],
                root.stats().reward / root.stats().visits as f32,
                policy,
                !pos.stm(),
            ));
        }

        if pos.game_state() == GameState::Draw {
            return;
        }

        // Last move is from other stm
        let mut result = match pos.game_state() {
            GameState::Win => -1.0,
            GameState::Draw => 0.0,
            GameState::Loss => 1.0,
            GameState::Ongoing => unreachable!(),
        };

        for i in (0..data.len()).rev() {
            data[i].result = result;

            result = -result;
        }

        self.buffer.append(&mut data);
    }

    fn deployment(&mut self) -> String {
        let red = self
            .mcts_sim
            .deployment()
            .to_ascii_uppercase()
            .split('/')
            .rev()
            .collect::<Vec<_>>()
            .join("/");
        let blue = self.mcts_net.deployment().to_ascii_lowercase();

        format!("{}/8/8/{} r", blue, red)
    }
}
