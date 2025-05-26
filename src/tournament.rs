use crate::{
    stratego::{GameState, StrategoState},
    Algorithm,
};
use engine::Engine;
use ordered_float::OrderedFloat;
use rating::Ranking;
use schedule::Schedule;

pub mod engine;
mod rating;
mod schedule;

pub struct Tournament {
    engines: Vec<Engine>,
    results: Vec<Ranking>,
    limit: usize,
}

impl Tournament {
    pub fn new(limit: usize) -> Self {
        Self {
            engines: Vec::new(),
            results: Vec::new(),
            limit,
        }
    }

    pub fn add(&mut self, name: &str, algorithm: Algorithm, cheating: bool) {
        self.results.push(Ranking::new(self.engines.len()));
        self.engines.push(Engine::new(name, algorithm, cheating));
    }

    pub fn run(&mut self, rounds: usize) {
        assert!(self.engines.len() >= 2);

        let players = self.engines.len();
        let schedule = Schedule::from(players, rounds);
        let games = schedule.games();

        let mut rounds = vec![0usize; self.limit + 1];
        let mut branching = vec![0usize; self.limit + 1];
        let mut length = vec![0usize; games];

        println!("info tournament rounds {}", schedule.games());
        for (i, pair) in schedule.enumerate() {
            self.play(
                i,
                games,
                pair.0,
                pair.1,
                &mut rounds,
                &mut branching,
                &mut length,
            );
        }

        println!("{}", self.result());
        println!("{:?}", rounds);
        println!("{:?}", branching);
        println!("{:?}", length);
    }

    fn play(
        &mut self,
        index: usize,
        limit: usize,
        i: usize,
        j: usize,
        rounds: &mut Vec<usize>,
        branching: &mut Vec<usize>,
        length: &mut Vec<usize>,
    ) {
        let mut history = Vec::new();

        let deployments = self.deployment(i, j);
        let pos_str = format!("{}/8/8/{} r", deployments.1, deployments.0);

        let winner = self.game_loop(i, j, &mut history, &pos_str, rounds, branching);
        self.results[i].update(winner[0]);
        self.results[j].update(winner[1]);

        length[index] = history.len();

        println!(
            "info game {}/{} pos {} moves {}",
            index,
            limit,
            pos_str,
            history.len()
        );
    }

    fn result(&mut self) -> String {
        let mut results = String::new();

        self.results.sort_by_key(|r| OrderedFloat::from(r.points()));
        for (rank, result) in self.results.iter().rev().enumerate() {
            results.push_str(&format!(
                "info result rank {} name {} rating {} points {} played {} W {} D {} L {}\n",
                rank + 1,
                self.engines[result.index()].name(),
                result.diff(),
                result.points(),
                result.games(),
                result.wins(),
                result.draws(),
                result.losses()
            ));
        }

        results
    }

    fn game_loop(
        &mut self,
        i: usize,
        j: usize,
        moves: &mut Vec<String>,
        pos_str: &str,
        rounds: &mut Vec<usize>,
        branching: &mut Vec<usize>,
    ) -> [f32; 2] {
        let indices = [i, j];

        let mut pos = StrategoState::from(&pos_str);

        let mut ply = 0;
        let mut stm = 0;
        while !pos.game_over() {
            let gen = pos.gen();

            if gen.len() == 0 {
                pos.set_game_state(GameState::Loss);
                break;
            }

            if moves.len() > self.limit {
                pos.set_game_state(GameState::Draw);
                break;
            }

            let player_pos = if self.engines[indices[stm]].cheating() {
                pos.clone()
            } else {
                pos.anonymize(stm ^ 1)
            };

            let mov = self.engines[indices[stm]].go(player_pos);
            moves.push(format!("{}", mov));

            #[cfg(feature = "info")]
            println!("info move {} stm {} moves {:?}", mov, stm, moves);

            let mov = gen.iter().find(|m| format!("{}", m) == format!("{}", mov));
            if mov.is_none() {
                println!("{}", pos);
                println!("{:?}", moves);

                // TODO: unreachable!();
                return [0.5, 0.5];
            }

            rounds[ply] += 1;
            branching[ply] += gen.len();

            stm ^= 1;
            ply += 1;

            let mov = mov.unwrap();
            pos.make(mov);
        }

        let mut result = [0.0, 0.0];
        match pos.game_state() {
            GameState::Win => result[stm] = 1.0,
            GameState::Draw => result = [0.5, 0.5],
            GameState::Loss => result[stm ^ 1] = 1.0,
            GameState::Ongoing => unreachable!(),
        };

        // println!("{}", pos);

        result
    }

    fn deployment(&mut self, i: usize, j: usize) -> (String, String) {
        (
            self.engines[i]
                .deployment()
                .to_ascii_uppercase()
                .split('/')
                .rev()
                .collect::<Vec<_>>()
                .join("/"),
            self.engines[j].deployment().to_ascii_lowercase(),
        )
    }
}
