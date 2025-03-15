use super::{iteration, node::Node};
use crate::stratego::{Move, MoveStack, Position};
use std::rc::Rc;

pub struct Search {
    root_pos: Position,
    stack: MoveStack,
    root_node: Rc<Node>,
}

impl Search {
    pub fn new(pos: Position, stack: MoveStack) -> Self {
        Search {
            root_pos: pos,
            stack,
            root_node: Node::new(),
        }
    }

    pub fn run(&mut self, iterations: usize) -> Move {
        for _ in 0..iterations {
            let pos = self.root_pos;
            let mut stack = self.stack.clone();
            let node = Rc::clone(&self.root_node);

            iteration::execute_one(pos, &mut stack, node);
        }

        let mut children: Vec<_> = self.root_node.children().collect();

        children.sort_by_key(|c| c.visits());
        for c in children {
            println!(
                "move {} visits {} reward {} availability {}",
                c.mov.unwrap(),
                c.visits(),
                c.reward(),
                c.availability()
            );
        }

        self.root_node
            .children()
            .max_by_key(|c| c.visits())
            .map(|c| c.mov.unwrap_or_default())
            .unwrap_or_default()
    }
}
