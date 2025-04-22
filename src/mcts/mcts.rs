use ordered_float::OrderedFloat;
use rand::distr::weighted::WeightedIndex;

use super::{iteration, node::Node, Search};
use crate::{
    deployment::Deployment,
    policy::Policy,
    select::Select,
    stratego::{Move, StrategoState},
    value::Value,
};
use std::sync::Arc;

pub struct MCTS {
    root: Arc<Node>,
    pos: Option<StrategoState>,
    iterations: usize,
    value: Value,
    policy: Policy,
    select: Select,
    deployment: Deployment,
}

impl Search for MCTS {
    fn select(&self, node: &Node, moves: &[Move]) -> Option<Arc<Node>> {
        let children = node.children();

        let legal: Vec<_> = children
            .filter(|c| moves.iter().any(|m| c.mov().unwrap() == *m))
            .collect();

        let choice = legal
            .iter()
            .max_by_key(|c| OrderedFloat::from((self.select)(c)))
            .cloned();

        choice
    }

    fn value(&self, pos: &mut StrategoState) -> f32 {
        (self.value)(pos)
    }

    fn policy(&self, pos: &StrategoState, moves: &Vec<Move>) -> WeightedIndex<f32> {
        (self.policy)(pos, moves)
    }

    fn deployment(&self) -> String {
        (self.deployment)()
    }
}

impl MCTS {
    pub fn new(
        iterations: usize,
        value: Value,
        policy: Policy,
        select: Select,
        deployment: Deployment,
    ) -> Self {
        Self {
            root: Node::new(),
            pos: None,
            iterations,
            value,
            policy,
            select,
            deployment,
        }
    }

    pub fn go(&mut self, pos: &StrategoState) -> Move {
        self.set_pos(None);
        self.set_root(pos);
        self.run(pos);

        #[cfg(feature = "info")]
        {
            let mut children: Vec<_> = self.root.children().collect();
            children.sort_by_key(|c| c.stats().visits);

            for c in children {
                let stats = c.stats();

                println!(
                    "info move {} visits {} reward {}",
                    c.mov().unwrap(),
                    stats.visits,
                    stats.reward,
                );
            }
        }

        self.root.max_visits().unwrap().mov().unwrap()
    }

    pub fn run(&mut self, pos: &StrategoState) {
        for _ in 0..self.iterations {
            let mut pos = pos.clone();
            let node = Arc::clone(&self.root);

            iteration::execute_one::<MCTS, false>(&mut pos, node, self);
        }
    }

    pub fn set_pos(&mut self, pos: Option<StrategoState>) {
        self.pos = pos;
    }

    pub fn root(&self) -> &Node {
        &self.root
    }

    pub fn set_root(&mut self, new: &StrategoState) {
        let old_root = self.root.clone();

        self.root = if self.pos.is_some() {
            println!("info node searching for subtree");
            let new = self.recursive_find(old_root, &self.pos.clone().unwrap(), new, 2);

            if new.is_some() {
                println!("info node found subtree")
            }

            new.unwrap_or(Node::new())
        } else {
            Node::new()
        };

        self.pos = Some(new.clone());
    }

    fn recursive_find(
        &self,
        node: Arc<Node>,
        old: &StrategoState,
        new: &StrategoState,
        depth: usize,
    ) -> Option<Arc<Node>> {
        if old.board() == new.board() {
            return Some(node);
        }

        if depth == 0 || node.is_empty() {
            return None;
        }

        for child in node.children() {
            let mut child_board = old.clone();
            child_board.make(child.mov().unwrap());

            let found = self.recursive_find(child, &mut child_board, new, depth - 1);

            if !found.is_none() {
                return found;
            }
        }

        None
    }
}
