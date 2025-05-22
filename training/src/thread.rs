use crate::buffer::SearchData;
use rand::{rng, seq::IteratorRandom};
use stratego::{
    deployment::Deployment,
    mcts::{Search, MCTS},
    policy::Policy,
    select::Select,
    stratego::{GameState, StrategoState},
    value::{heuristic, Network, Value},
};

pub struct DatagenThread {
    mcts: MCTS,
    buffer: Vec<SearchData>,
    games: usize,
}

impl DatagenThread {
    const LIMIT: usize = 150;
    const RANDOM: usize = 10;

    pub fn new(iterations: usize, games: usize, network: Network) -> Self {
        Self {
            mcts: MCTS::new(
                iterations,
                Value::NetworkCutoff(network, 0.1),
                Policy::Uniform,
                Select::UCT(1.41),
                Deployment::Heuristic(50, false),
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

            let mov = self.mcts.go(&pos);

            let mov = gen.iter().find(|m| format!("{}", m) == format!("{}", mov));
            if mov.is_none() {
                unreachable!();
            }

            let heuristic = heuristic(&mut pos, 750.0);

            let mov = mov.unwrap();
            pos.make(mov);

            let root = self.mcts.root();

            let mut policy = Vec::new();
            for child in root.children() {
                let mov = child.mov().unwrap();
                policy.push((mov, child.stats().visits));
            }

            data.push(SearchData::new(
                [red, blue],
                root.stats().reward / root.stats().visits as f32,
                heuristic,
                policy,
                !pos.stm(),
            ));
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
            .mcts
            .deployment()
            .to_ascii_uppercase()
            .split('/')
            .rev()
            .collect::<Vec<_>>()
            .join("/");
        let blue = self.mcts.deployment().to_ascii_lowercase();

        format!("{}/8/8/{} r", blue, red)
    }
}
