use crate::buffer::SearchData;
use stratego::{
    deployment::Deployment,
    mcts::MCTS,
    policy::Policy,
    select::Select,
    stratego::{GameState, StrategoState},
    value::{Network, Value},
};

pub struct DatagenThread {
    mcts: MCTS,
    buffer: Vec<SearchData>,
    games: usize,
}

impl DatagenThread {
    pub fn new(iterations: usize, games: usize, network: Network) -> Self {
        Self {
            mcts: MCTS::new(
                iterations,
                Value::Network(network),
                Policy::Uniform,
                Select::UCT(1.41),
                Deployment::Dataset,
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

            let features = pos.features(pos.stm() as usize);

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

            data.push(SearchData::new(features, policy));
        }

        // Last move is from other stm
        let mut result = match pos.game_state() {
            GameState::Ongoing => unreachable!(),
            GameState::Win => -1.0,
            GameState::Draw => 0.0,
            GameState::Loss => 1.0,
        };

        for i in (0..data.len()).rev() {
            data[i].target = result;
            result = -result;
        }

        self.buffer.append(&mut data);
    }

    fn deployment(&mut self) -> String {
        let red = Deployment::Heuristic
            .get()
            .to_ascii_uppercase()
            .split('/')
            .rev()
            .collect::<Vec<_>>()
            .join("/");
        let blue = Deployment::Heuristic.get().to_ascii_lowercase();

        format!("{}/8/8/{} r", blue, red)
    }
}
