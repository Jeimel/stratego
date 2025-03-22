use super::{iteration, node::Node};
use crate::stratego::{GameState, Move, StrategoState};
use rand::{rng, seq::IteratorRandom};
use std::rc::Rc;

impl StrategoState {
    pub fn rollout(&mut self) -> f32 {
        let mut rng = rng();

        let stm = self.stm();

        while !self.game_over() {
            let mov = self.gen().iter().choose(&mut rng);

            if let Some(mov) = mov {
                self.make(mov);
            } else {
                // Handle two-squares and more-squares rule
                self.set_game_state(GameState::Loss);
            }
        }

        let current = f32::from(stm == self.stm());
        match self.game_state() {
            GameState::Draw => 0.0,
            GameState::Win => -1.0 + (2.0 * current),
            GameState::Loss => 1.0 + (-2.0 * current),
            GameState::Ongoing => unreachable!(),
        }
    }
}

pub struct MCTS {
    root: Rc<Node>,
    pos: Option<StrategoState>,
    iterations: usize,
}

impl MCTS {
    pub fn new(iterations: usize) -> Self {
        Self {
            root: Node::new(),
            pos: None,
            iterations,
        }
    }

    pub fn go(&mut self, pos: &StrategoState) -> Move {
        self.set_root(pos);
        self.run(pos);

        self.max_visits().unwrap().mov.unwrap()
    }

    pub fn run(&mut self, pos: &StrategoState) {
        for _ in 0..self.iterations {
            let mut pos = pos.clone();
            let node = Rc::clone(&self.root);

            iteration::execute_one(&mut pos, node);
        }

        /*
        let mut children: Vec<_> = self.root.children().collect();

        children.sort_by_key(|c| c.visits());
        for c in children {
            println!(
                "info move {} visits {} reward {} availability {} state {:?}",
                c.mov.unwrap(),
                c.visits(),
                c.reward(),
                c.availability(),
                c.game_state(),
            );
        }
        */
    }

    pub fn max_visits(&self) -> Option<Rc<Node>> {
        self.root.children().max_by_key(|c| c.visits())
    }

    pub fn set_pos(&mut self, pos: Option<StrategoState>) {
        self.pos = pos;
    }

    pub fn set_root(&mut self, new: &StrategoState) {
        let old_root = self.root.clone();

        self.root = if self.pos.is_some() {
            // println!("info node searching for subtree");
            let new = self.recursive_find(old_root, &self.pos.clone().unwrap(), new, 2);

            if new.is_some() {
                // println!("info node found subtree")
            }

            new.unwrap_or(Node::new())
        } else {
            Node::new()
        };

        self.pos = Some(new.clone());
    }

    fn recursive_find(
        &self,
        node: Rc<Node>,
        old: &StrategoState,
        new: &StrategoState,
        depth: usize,
    ) -> Option<Rc<Node>> {
        if old.board() == new.board() {
            return Some(node);
        }

        if depth == 0 || node.is_empty() {
            return None;
        }

        for child in node.children() {
            let mut child_board = old.clone();
            child_board.make(child.mov.unwrap());

            let found = self.recursive_find(child, &mut child_board, new, depth - 1);

            if !found.is_none() {
                return found;
            }
        }

        None
    }
}
