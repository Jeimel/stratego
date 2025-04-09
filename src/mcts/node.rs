use crate::stratego::{GameState, Move};
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::{Rc, Weak},
};

pub struct NodeStats {
    pub visits: usize,
    pub availability: usize,
    pub reward: f32,
}

impl Default for NodeStats {
    fn default() -> Self {
        Self {
            visits: 0,
            availability: 0,
            reward: 0.0,
        }
    }
}

impl NodeStats {
    pub fn new() -> Self {
        Self {
            visits: 0,
            availability: 1,
            reward: 0.0,
        }
    }
}

pub struct Node {
    mov: Option<Move>,
    parent: Option<Weak<Node>>,
    state: RefCell<GameState>,
    children: RefCell<Vec<Rc<Node>>>,
    stats: RefCell<NodeStats>,
}

impl Node {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            mov: None,
            parent: None,
            state: RefCell::new(GameState::default()),
            children: Default::default(),
            stats: RefCell::new(NodeStats::default()),
        })
    }

    pub fn add(self: Rc<Self>, mov: Move, state: GameState) -> Rc<Node> {
        let mut children = self.children.borrow_mut();
        let parent = Rc::downgrade(&self);

        let child = Rc::new(Node {
            mov: Some(mov),
            parent: Some(parent.clone()),
            children: Default::default(),
            state: RefCell::new(state),
            stats: RefCell::new(NodeStats::new()),
        });

        children.push(Rc::clone(&child));
        child
    }

    pub fn untried(&self, moves: &[Move]) -> Vec<Move> {
        moves
            .iter()
            .filter(|mov| !self.children().any(|c| c.mov.as_ref().unwrap() == *mov))
            .cloned()
            .collect()
    }

    pub fn update(&self, reward: f32) {
        let mut stats = self.stats.borrow_mut();

        stats.visits += 1;
        stats.reward += reward;
    }

    pub fn mov(&self) -> Option<Move> {
        self.mov
    }

    pub fn parent(&self) -> Option<Rc<Node>> {
        self.parent.as_ref().and_then(Weak::upgrade)
    }

    pub fn children(&self) -> impl Iterator<Item = Rc<Node>> {
        self.children.borrow().clone().into_iter()
    }

    pub fn is_empty(&self) -> bool {
        self.children.borrow().is_empty()
    }

    pub fn game_state(&self) -> GameState {
        *self.state.borrow()
    }

    pub fn stats(&self) -> Ref<'_, NodeStats> {
        self.stats.borrow()
    }

    pub fn stats_mut(&self) -> RefMut<'_, NodeStats> {
        self.stats.borrow_mut()
    }

    pub fn parent_visits(&self) -> usize {
        self.parent().unwrap().stats.borrow().visits
    }

    pub fn max_visits(&self) -> Option<Rc<Node>> {
        self.children().max_by_key(|c| c.stats().visits)
    }
}
